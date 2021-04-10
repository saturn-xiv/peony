#include "peony/sqlite3.h"

#include <memory>

#include <boost/log/trivial.hpp>

peony::Sqlite3::Sqlite3(const std::filesystem::path& file) {
  BOOST_LOG_TRIVIAL(info) << "open db " << file;
  if (sqlite3_initialize() != SQLITE_OK) {
    throw std::invalid_argument("falied to initialize sqlite3");
  }
  if (sqlite3_open_v2(file.c_str(), &db,
                      SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE,
                      NULL) != SQLITE_OK) {
    throw std::invalid_argument(sqlite3_errmsg(db));
  }
}

void peony::Sqlite3::wal_mode(const std::chrono::microseconds& timeout) {
  std::stringstream ss;
  ss << "PRAGMA synchronous = OFF;";
  ss << "PRAGMA journal_mode = WAL;";
  ss << "PRAGMA foreign_keys = ON;";
  ss << "PRAGMA busy_timeout = " << timeout.count() << ";";
  execute(ss.str());
}

peony::Sqlite3::~Sqlite3() {
  BOOST_LOG_TRIVIAL(info) << "close db";
  if (db != NULL) {
    sqlite3_close(db);
    db = NULL;
    sqlite3_shutdown();
  }
}

void peony::Sqlite3::execute(const std::string& sql, sqlite3_stmt** query) {
  BOOST_LOG_TRIVIAL(debug) << "execute " << sql;
  if (sqlite3_prepare_v2(db, sql.c_str(), -1, query, NULL) != SQLITE_OK) {
    throw std::invalid_argument(sqlite3_errmsg(db));
  }
}

void peony::Sqlite3::execute(const std::string& sql) {
  sqlite3_stmt* query = NULL;
  execute(sql, &query);
  const auto it = sqlite3_step(query);
  if (NULL != query) {
    sqlite3_finalize(query);
  }
  if (it != SQLITE_DONE) {
    throw std::invalid_argument(sqlite3_errmsg(db));
  }
}
