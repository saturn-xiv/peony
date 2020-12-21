#include "orm.h"

#define PEONY_DB_PREPARE_FILE "prepares.toml"

/**
 * PostgreSql
 */

#define PEONY_POSTGRESQL_SCHEMA_MIGRATIONS_EXISTS                              \
  "SELECT table_name AS name FROM information_schema.tables WHERE table_name " \
  "= 'schema_migrations'"
#define PEONY_POSTGRESQL_SCHEMA_MIGRATIONS_VERSION "SELECT VERSION() AS value"

std::shared_ptr<pqxx::connection> peony::postgresql::Config::open(
    std::vector<std::filesystem::path> prepares) {
  std::stringstream ss;
  ss << "sslmode=disable"
     << " connect_timeout=" << 6 << " host=" << this->host
     << " port=" << this->port << " dbname=" << this->db
     << " user=" << this->user;
  if (this->password) {
    ss << " password=" << this->password.value();
  }

  auto con = std::make_shared<pqxx::connection>(ss.str());
  for (const auto &it : prepares) {
    auto root = toml::parse_file(it.string());
    for (auto &&[k1, v1] : root) {
      if (v1.is_table()) {
        auto vt = v1.as_table();
        for (auto &&[k2, v2] : *vt) {
          std::optional<std::string> val = v2.value<std::string>();
          if (val) {
            con->prepare(k1 + "." + k2, val.value());
          }
        }
      }
    }
  }
  return con;
}

peony::postgresql::Pool::Pool(const std::shared_ptr<Config> config)
    : config(config) {
  BOOST_LOG_TRIVIAL(info) << "open postgresql pool " << *(this->config);
  while (this->pool.size() < this->config->pool_size) {
    this->pool.push_back(this->create());
  }
}

std::shared_ptr<pqxx::connection> peony::postgresql::Pool::create() {
  std::vector<std::filesystem::path> prepares;
  for (const auto &it : std::filesystem::recursive_directory_iterator(
           std::filesystem::path(this->config->schema / "prepares"))) {
    prepares.push_back(it.path());
  }
  auto con = this->config->open(prepares);
  return con;
}

std::shared_ptr<pqxx::connection> peony::postgresql::Pool::get() {
  std::lock_guard<std::mutex> lock(this->locker);
  if (this->pool.size() == 0) {
    for (auto it = this->used.begin(); it != this->used.end(); ++it) {
      if ((*it).unique()) {
        BOOST_LOG_TRIVIAL(debug) << "creating new postgresql connection to "
                                    "replace discarded connection";
        auto con = this->create();
        this->used.erase(it);
        this->used.insert(con);
        return con;
      }
    }
    throw std::out_of_range("postgresql pool is full");
  }

  auto con = this->pool.front();
  this->pool.pop_front();
  this->used.insert(con);
  return con;
}

void peony::postgresql::Pool::release(std::shared_ptr<pqxx::connection> it) {
  std::lock_guard<std::mutex> lock(this->locker);
  this->pool.push_back(it);
  this->used.erase(it);
}

std::pair<size_t, size_t> peony::postgresql::Pool::status() {
  std::lock_guard<std::mutex> lock(this->locker);
  return std::make_pair(this->pool.size(), this->used.size());
}

peony::postgresql::Config::Config(const toml::table &root) {
  this->host = root["host"].value<std::string>().value_or(PEONY_DEFAULT_HOST);
  this->port = root["port"].value<unsigned short>().value_or(5432);
  this->user = root["user"].value<std::string>().value_or("postgres");
  this->password = root["password"].value<std::string>();
  this->db = root["db"].value<std::string>().value();
  this->pool_size =
      root["pool-size"].value<long>().value_or(PEONY_DEFAULT_POOL_SIZE);
  this->schema = root["schema"].value<std::string>().value_or("db");
}

peony::postgresql::Config::operator toml::table() const {
  toml::table root;
  root.insert("host", this->host);
  root.insert("port", this->port);
  root.insert("user", this->user);

  if (this->password) {
    root.insert("password", this->password.value());
  }
  root.insert("db", this->db);
  root.insert("pool-size", (long)this->pool_size);
  root.insert("schema", this->schema.string());
  return root;
};

  /**
   * MySql
   */

#define PEONY_MYSQL_SCHEMA_MIGRATIONS_EXISTS                        \
  "SELECT table_name AS name FROM information_schema.tables WHERE " \
  "table_schema = DATABASE() AND table_name = 'schema_migrations'"
#define PEONY_MYSQL_VERSION "SELECT VERSION() AS value"

peony::MySql::MySql(const toml::table &root) {
  this->host = root["host"].value<std::string>().value_or(PEONY_DEFAULT_HOST);
  this->port = root["port"].value<unsigned short>().value_or(3306);
  this->user = root["user"].value<std::string>().value_or("root");
  this->password = root["password"].value<std::string>();
  this->db = root["db"].value<std::string>().value();
  this->pool_size =
      root["pool-size"].value<long>().value_or(PEONY_DEFAULT_POOL_SIZE);
}

peony::MySql::operator toml::table() const {
  toml::table root;
  root.insert("host", this->host);
  root.insert("port", this->port);
  root.insert("user", this->user);

  if (this->password) {
    root.insert("password", this->password.value());
  }
  root.insert("db", this->db);
  root.insert("pool-size", (long)this->pool_size);
  return root;
};

  /**
   * Sqlite3
   */

#define PEONY_SQLITE3_SCHEMA_MIGRATIONS_EXISTS             \
  "SELECT name FROM sqlite_master WHERE type='table' AND " \
  "name='schema_migrations'"
#define PEONY_SQLITE3_VERSION "SELECT SQLITE_VERSION() AS value"

/**
 * Migration
 */

bool peony::orm::Migration::operator<(const Migration &self) const {
  return version < self.version;
}

/**
 * SchemaDao
 */

void peony::orm::SchemaDao::load() {
  std::lock_guard<std::mutex> lock(this->locker);

  for (const auto &dir :
       std::filesystem::directory_iterator(this->migrations_root())) {
    const auto root = dir.path();
    BOOST_LOG_TRIVIAL(debug) << "load migration from " << root.string();
    const std::string fn = root.filename().string();
    const std::string version = fn.substr(0, PEONY_TIMESTAMP_SIZE);
    const std::string name = fn.substr(PEONY_TIMESTAMP_SIZE + 1);

    std::ifstream upf(root / "up.sql");
    const std::string up((std::istreambuf_iterator<char>(upf)),
                         std::istreambuf_iterator<char>());

    std::ifstream downf(root / "down.sql");
    const std::string down((std::istreambuf_iterator<char>(downf)),
                           std::istreambuf_iterator<char>());
    this->migrations.push_back(
        Migration(version, name, up, down, this->run_at(version)));
  }
  std::sort(this->migrations.begin(), this->migrations.end());
}

void peony::orm::SchemaDao::migrate() {
  std::lock_guard<std::mutex> lock(this->locker);
  for (auto it = this->migrations.begin(); it != this->migrations.end(); it++) {
    BOOST_LOG_TRIVIAL(info) << "find " << it->version << " " << it->name;
    if (!it->run_at) {
      BOOST_LOG_TRIVIAL(info) << "migrate...";
      this->execute(it->up);
      this->insert(it->version);
    }
  }
}

void peony::orm::SchemaDao::generate(const std::string &name) const {
  const auto root =
      this->migrations_root() / (peony::crypt::timestamp() + "-" + name);
  std::filesystem::create_directories(root);

  for (auto it : {"up.sql", "down.sql"}) {
    const auto tmp = root / it;
    BOOST_LOG_TRIVIAL(info) << "generate file " << tmp;
    std::ofstream out(tmp);
  }
}

void peony::orm::SchemaDao::rollback() {
  std::lock_guard<std::mutex> lock(this->locker);
  std::reverse(this->migrations.begin(), this->migrations.end());
  for (auto it = this->migrations.begin(); it != this->migrations.end(); it++) {
    if (it->run_at) {
      BOOST_LOG_TRIVIAL(info)
          << "find " << it->version << " " << it->name << ", rollback...";
      this->execute(it->down);
      this->delete_(it->version);
      return;
    }
  }
  BOOST_LOG_TRIVIAL(info) << "empty!";
}

/**
 * SchemaDao PG
 */

peony::postgresql::SchemaDao::SchemaDao(
    const std::shared_ptr<peony::postgresql::Config> cfg)
    : config(cfg) {
  const auto root = std::filesystem::path(cfg->schema) / "schema";
  {
    auto con = cfg->open();
    BOOST_LOG_TRIVIAL(debug) << "load schema up script";
    std::ifstream sf(root / "up.sql");
    const std::string sql((std::istreambuf_iterator<char>(sf)),
                          std::istreambuf_iterator<char>());
    BOOST_LOG_TRIVIAL(debug) << sql;
    pqxx::work tx(*con);
    tx.exec(sql);
    tx.commit();
  }
  BOOST_LOG_TRIVIAL(debug) << "load schema prepares";
  this->connection = cfg->open({root / PEONY_DB_PREPARE_FILE});
}

void peony::postgresql::SchemaDao::execute(const std::string &script) const {
  BOOST_LOG_TRIVIAL(debug) << script;
  pqxx::work tx(*(this->connection));
  tx.exec(script);
  tx.commit();
}

std::optional<boost::posix_time::ptime> peony::postgresql::SchemaDao::run_at(
    const std::string &version) const {
  pqxx::work tx(*(this->connection));
  pqxx::result rst =
      tx.exec_prepared("schema_migrations.find-by-version", version);
  tx.commit();

  for (auto it = rst.begin(); it != rst.end(); it++) {
    return PEONY_STR2TS(it[PEONY_COLUMN_CREATED_AT]);
  }

  return std::nullopt;
}

void peony::postgresql::SchemaDao::delete_(const std::string &version) const {
  pqxx::work tx(*(this->connection));
  pqxx::result rst =
      tx.exec_prepared("schema_migrations.delete-by-version", version);
  tx.commit();
}

void peony::postgresql::SchemaDao::insert(const std::string &version) const {
  pqxx::work tx(*(this->connection));
  pqxx::result rst = tx.exec_prepared("schema_migrations.insert", version);
  tx.commit();
}

std::filesystem::path peony::postgresql::SchemaDao::migrations_root() const {
  return this->config->schema / "migrations";
}