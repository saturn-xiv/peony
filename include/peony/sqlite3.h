#pragma once

#include "peony/orm.h"

#include <soci/sqlite3/soci-sqlite3.h>

// #include <sqlite3.h>

namespace peony {

namespace sqlite3 {
/**
 *
 *  .show Displays current settings for various parameters
 *  .databases Provides database names and files
 *  .quit Quit sqlite3 program
 *  .tables Show current tables
 *  .schema Display schema of table
 *  .header Display or hide the output table header
 *  .mode Select mode for the output table
 *  .dump Dump database in SQL text format
 *  pragma compile_options;
 *  SELECT name FROM sqlite_master WHERE type='table' AND name='TABLE_NAME'
 *
 */

class Dao {
 public:
  Dao(std::shared_ptr<soci::session> db);
  std::string version();
  //   https://stackoverflow.com/questions/57123453/how-to-use-diesel-with-sqlite-connections-and-avoid-database-is-locked-type-of
  void wal_mode(const std::chrono::milliseconds& timeout);

 private:
  std::shared_ptr<soci::session> db;
};

}  // namespace sqlite3
}  // namespace peony
