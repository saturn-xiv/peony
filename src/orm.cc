#include "orm.h"

#define PEONY_POSTGRESQL_SCHEMA_MIGRATIONS_EXISTS                              \
  "SELECT table_name AS name FROM information_schema.tables WHERE table_name " \
  "= 'schema_migrations'"
#define PEONY_POSTGRESQL_SCHEMA_MIGRATIONS_VERSION "SELECT VERSION() AS value"

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