#ifndef PEONY_ORM_H_
#define PEONY_ORM_H_

#include <filesystem>
#include <string>

#include "env.h"

#include <sqlite3.h>
#include <pqxx/pqxx>

namespace peony {

// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
// journalctl -f -u postgresql
// https://www.postgresql.org/docs/current/libpq-connect.html
// https://libpqxx.readthedocs.io/en/latest/index.html
class PostgreSql {
 public:
  PostgreSql(std::string host = "127.0.0.1", unsigned int port = 5432,
             std::string user = "postgres",
             std::optional<std::string> password = std::nullopt,
             std::string db = PEONY_PROJECT_NAME, size_t pool_size = 32)
      : host(host),
        port(port),
        user(user),
        password(password),
        db(db),
        pool_size(pool_size) {}
  PostgreSql(const toml::table &root);
  friend std::ostream &operator<<(std::ostream &out, const PostgreSql &self) {
    out << self.user << '@' << self.host << ':' << self.port << '/' << self.db;
    return out;
  }
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(PostgreSql, host, port, user, password, db,
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

// use DB-NAME
// show tables;
// desc TABLE-NAME;
// SELECT table_name FROM information_schema.tables WHERE table_schema =
// 'databasename' AND table_name = 'testtable'; SHOW TABLES LIKE 'tablename';
class MySql {
 public:
  MySql(std::string host = "127.0.0.1", unsigned int port = 3306,
        std::string user = "root",
        std::optional<std::string> password = std::nullopt,
        std::string db = PEONY_PROJECT_NAME, size_t pool_size = 32)
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