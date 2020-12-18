#ifndef PEONY_ORM_H_
#define PEONY_ORM_H_

#include "common.h"

#include <sqlite3.h>
#include <pqxx/pqxx>

namespace peony {

// https://www.postgresql.org/docs/current/runtime-config-logging.html
// /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
// journalctl -f -u postgresql
// https://www.postgresql.org/docs/current/libpq-connect.html
// https://libpqxx.readthedocs.io/en/latest/index.html
namespace postgresql {

class Pool {};

}  // namespace postgresql

// use DB-NAME
// show tables;
// desc TABLE-NAME;
// SELECT table_name FROM information_schema.tables WHERE table_schema =
// 'databasename' AND table_name = 'testtable'; SHOW TABLES LIKE 'tablename';
namespace mysql {}

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
namespace sqlite3 {

class Config {
 private:
  std::filesystem::path file;
};

}  // namespace sqlite3

}  // namespace peony

#endif