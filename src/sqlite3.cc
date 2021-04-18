#include "peony/sqlite3.h"

peony::sqlite3::Dao::Dao(std::shared_ptr<soci::session> db) : db(db) {}
std::string peony::sqlite3::Dao::version() {
  std::string it;
  *db << "SELECT SQLITE_VERSION()", soci::into(it);
  return it;
}

void peony::sqlite3::Dao::wal_mode(const std::chrono::milliseconds& timeout) {
  *db << "PRAGMA synchronous = OFF";
  *db << "PRAGMA journal_mode = WAL";
  *db << "PRAGMA foreign_keys = ON";
  *db << "PRAGMA busy_timeout = " << timeout.count();
}
