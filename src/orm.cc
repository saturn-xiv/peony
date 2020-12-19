#include "orm.h"

#define PEONY_POSTGRESQL_SCHEMA_MIGRATIONS_EXISTS                              \
  "SELECT table_name AS name FROM information_schema.tables WHERE table_name " \
  "= 'schema_migrations'"
#define PEONY_POSTGRESQL_SCHEMA_MIGRATIONS_VERSION "SELECT VERSION() AS value"

peony::PostgreSql::PostgreSql(const toml::table &root) {
  this->host = root["host"].value<std::string>().value_or(PEONY_DEFAULT_HOST);
  this->port = root["port"].value<unsigned short>().value_or(5432);
  this->user = root["user"].value<std::string>().value_or("postgres");
  this->password = root["password"].value<std::string>();
  this->db = root["db"].value<std::string>().value();
  this->pool_size =
      root["pool-size"].value<long>().value_or(PEONY_DEFAULT_POOL_SIZE);
}

peony::PostgreSql::operator toml::table() const {
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
// --------------------------------------------------------
#define PEONY_SQLITE3_SCHEMA_MIGRATIONS_EXISTS             \
  "SELECT name FROM sqlite_master WHERE type='table' AND " \
  "name='schema_migrations'"

#define PEONY_SQLITE3_VERSION "SELECT SQLITE_VERSION() AS value"

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

// --------------------------------------------------------
#define PEONY_MYSQL_SCHEMA_MIGRATIONS_EXISTS                        \
  "SELECT table_name AS name FROM information_schema.tables WHERE " \
  "table_schema = DATABASE() AND table_name = 'schema_migrations'"
#define PEONY_MYSQL_VERSION "SELECT VERSION() AS value"