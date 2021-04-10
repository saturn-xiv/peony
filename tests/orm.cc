#define BOOST_TEST_MODULE Jwt Test

#include <boost/test/included/unit_test.hpp>

#include "peony/mysql.h"
#include "peony/orm.h"
#include "peony/postgresql.h"
#include "peony/sqlite3.h"

BOOST_AUTO_TEST_CASE(sqlite3_test) {
  //   peony::Sqlite3 db("db");
  //   db.wal_mode(std::chrono::seconds(5));

  //   {
  //     const char* SQL = R"sql(
  // create table if not exists t1(
  //     ci INTEGER NOT NULL,
  //     cs VARCHAR(10) NOT NULL,
  //     ct TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
  // )
  //       )sql";
  //     db.execute(SQL);
  //   }
  //   {
  //     char* sql = sqlite3_mprintf(
  //         "insert into t1(cs, ci, ct) values('%q', '%i', CURRENT_TIMESTAMP)",
  //         "hello", 1, "aaa");
  //     db.execute(sql);
  //     sqlite3_free(sql);
  //   }

  //   {
  //     char* sql = sqlite3_mprintf("select ci,cs,ct from t1 order by ct
  //     desc"); sqlite3_stmt* query = NULL; db.execute(sql, &query);
  //     sqlite3_free(sql);

  //     while (sqlite3_step(query) == SQLITE_ROW) {
  //       const auto size = sqlite3_column_count(query);
  //       for (auto i = 0; i < size; i++) {
  //         std::cout << sqlite3_column_name(query, i) << " "
  //                   << sqlite3_column_type(query, i) << std::endl;
  //       }
  //       std::cout << sqlite3_column_int(query, 0) << " "
  //                 << sqlite3_column_text(query, 1) << " "
  //                 << sqlite3_column_text(query, 2) << std::endl;
  //     }
  //     if (NULL != query) {
  //       sqlite3_finalize(query);
  //     }
  //   }
}

BOOST_AUTO_TEST_CASE(postgresql_test) {}

BOOST_AUTO_TEST_CASE(mysql_test) {}

BOOST_AUTO_TEST_CASE(migration_test) {}
