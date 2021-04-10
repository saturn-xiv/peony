#pragma once

#include <chrono>
#include <filesystem>

#include <sqlite3.h>

namespace peony {

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
class Sqlite3 {
 public:
  Sqlite3(const std::filesystem::path& file);
  ~Sqlite3();

  // https://stackoverflow.com/questions/57123453/how-to-use-diesel-with-sqlite-connections-and-avoid-database-is-locked-type-of
  void wal_mode(const std::chrono::microseconds& timeout);

  void execute(const std::string& sql);
  void execute(const std::string& sql, sqlite3_stmt** query);

 private:
  sqlite3* db;
};
}  // namespace peony
