#ifndef PEONY_ORM_H_
#define PEONY_ORM_H_

#include <deque>
#include <fstream>
#include <mutex>
#include <ostream>
#include <set>
#include <string>
#include <tuple>
#include <unordered_map>
#include <utility>
#include <vector>

#include "env.h"

#include <sqlite3.h>
#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/format.hpp>
#include <boost/log/trivial.hpp>
#include <pqxx/pqxx>

#include "crypt.h"

#define PEONY_COLUMN_CREATED_AT "created_at"
#define PEONY_DEFAULT_SCHEMA_ROOT "db"

namespace peony {

namespace orm {
class Migration {
 public:
  Migration(const std::string version, const std::string name,
            const std::string up, const std::string down,
            const std::optional<boost::posix_time::ptime> run_at = std::nullopt)
      : version(version), name(name), up(up), down(down), run_at(run_at) {}
  friend std::ostream &operator<<(std::ostream &out, const Migration &self) {
    out << "### " << self.version << " " << self.name;
    if (self.run_at) {
      out << " "
          << boost::posix_time::to_iso_extended_string(self.run_at.value());
    }
    out << " ###" << std::endl
        << "--- UP ---" << std::endl
        << self.up << "--- DOWN ---" << std::endl
        << self.down;

    return out;
  }
  bool operator<(const Migration &self) const;

  friend class SchemaDao;

  std::string version;
  std::string name;
  std::string up;
  std::string down;
  std::optional<boost::posix_time::ptime> run_at;
};

class SchemaDao {
 public:
  void load();
  void migrate();
  void rollback();
  void generate(const std::string &name) const;
  friend std::ostream &operator<<(std::ostream &out, const SchemaDao &self) {
    const std::string fmt = "%1$-15s %2$-24s ";

    out << (boost::format(fmt) % "VERSION" % "NAME") << "RUN AT" << std::endl;
    for (auto it = self.migrations.begin(); it != self.migrations.end(); it++) {
      out << (boost::format(fmt) % it->version % it->name);
      if (it->run_at) {
        out << boost::posix_time::to_iso_extended_string(it->run_at.value());
      } else {
        out << "N/A";
      };
      out << std::endl;
    }
    return out;
  }

 protected:
  virtual void execute(const std::string &script) const = 0;
  virtual void delete_(const std::string &version) const = 0;
  virtual void insert(const std::string &version) const = 0;
  virtual std::filesystem::path migrations_root() const = 0;
  virtual std::optional<boost::posix_time::ptime> run_at(
      const std::string &version) const = 0;

  std::vector<Migration> migrations;
  std::mutex locker;
};

}  // namespace orm

// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
// journalctl -f -u postgresql
// https://www.postgresql.org/docs/current/libpq-connect.html
// https://libpqxx.readthedocs.io/en/latest/index.html
namespace postgresql {

class Config {
 public:
  Config(std::string host = PEONY_DEFAULT_HOST, unsigned int port = 5432,
         std::string user = "postgres",
         std::optional<std::string> password = std::nullopt,
         std::string db = PEONY_PROJECT_NAME,
         size_t pool_size = PEONY_DEFAULT_POOL_SIZE,
         std::filesystem::path schema = "db")
      : host(host),
        port(port),
        user(user),
        password(password),
        db(db),
        pool_size(pool_size),
        schema(schema) {}
  Config(const toml::table &root);
  std::shared_ptr<pqxx::connection> open(
      std::optional<std::filesystem::path> prepare = std::nullopt);
  friend std::ostream &operator<<(std::ostream &out, const Config &self) {
    out << self.user << '@' << self.host << ':' << self.port << '/' << self.db;
    return out;
  }
  friend class Pool;
  friend class SchemaDao;
  operator toml::table() const;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Config, host, port, user, password, db,
                                 pool_size, schema)

 private:
  std::string host;
  unsigned short port;
  std::string user;
  std::optional<std::string> password;
  std::string db;
  std::filesystem::path schema;
  size_t pool_size;
};

class Pool {
 public:
  Pool(const std::shared_ptr<Config> config);
  ~Pool() {
    BOOST_LOG_TRIVIAL(warning) << "destory postgresql pool " << this->config;
  }
  std::shared_ptr<pqxx::connection> get();
  void release(std::shared_ptr<pqxx::connection> it);
  std::pair<size_t, size_t> status();

 private:
  std::shared_ptr<pqxx::connection> create();

  const std::shared_ptr<Config> config;
  std::deque<std::shared_ptr<pqxx::connection>> pool;
  std::set<std::shared_ptr<pqxx::connection>> used;
  std::mutex locker;
};

class SchemaDao : public peony::orm::SchemaDao {
 public:
  SchemaDao(const std::shared_ptr<Config> cfg);

  void execute(const std::string &script) const override;
  void delete_(const std::string &version) const override;
  void insert(const std::string &version) const override;
  std::filesystem::path migrations_root() const override;
  std::optional<boost::posix_time::ptime> run_at(
      const std::string &version) const override;

 private:
  std::shared_ptr<pqxx::connection> connection;
  const std::shared_ptr<Config> config;
};

}  // namespace postgresql

// use DB-NAME
// show tables;
// desc TABLE-NAME;
// SELECT table_name FROM information_schema.tables WHERE table_schema =
// 'databasename' AND table_name = 'testtable'; SHOW TABLES LIKE 'tablename';
class MySql {
 public:
  MySql(std::string host = PEONY_DEFAULT_HOST, unsigned int port = 3306,
        std::string user = "root",
        std::optional<std::string> password = std::nullopt,
        std::string db = PEONY_PROJECT_NAME,
        size_t pool_size = PEONY_DEFAULT_POOL_SIZE)
      : host(host),
        port(port),
        user(user),
        password(password),
        db(db),
        pool_size(pool_size) {}
  MySql(const toml::table &root);
  friend std::ostream &operator<<(std::ostream &out, const MySql &self) {
    out << self.user << '@' << self.host << ':' << self.port << '/' << self.db;
    return out;
  }
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(MySql, host, port, user, password, db,
                                 pool_size)

  operator toml::table() const;

 private:
  std::string host;
  unsigned short port;
  std::string user;
  std::optional<std::string> password;
  std::string db;
  size_t pool_size;
};

// .show Displays current settings for various parameters
// .databases Provides database names and files
// .quit Quit sqlite3 program
// .tables Show current tables
// .schema Display schema of table
// .header Display or hide the output table header
// .mode Select mode for the output table
// .dump Dump database in SQL text format
// pragma compile_options;
// SELECT name FROM sqlite_master WHERE type='table' AND name='TABLE_NAME'
class Sqlite3 {
 public:
  Sqlite3(std::string file = "db") : file(file) {}
  friend std::ostream &operator<<(std::ostream &out, const Sqlite3 &self) {
    out << self.file;
    return out;
  }

 private:
  std::filesystem::path file;
};

}  // namespace peony

#endif