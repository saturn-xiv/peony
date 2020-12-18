#include "orm.h"

#define PEONY_POSTGRESQL_SCHEMA_MIGRATIONS_EXISTS                              \
  "SELECT table_name AS name FROM information_schema.tables WHERE table_name " \
  "= 'schema_migrations'"
#define PEONY_POSTGRESQL_SCHEMA_MIGRATIONS_VERSION "SELECT VERSION() AS value"
// --------------------------------------------------------
#define PEONY_SQLITE3_SCHEMA_MIGRATIONS_EXISTS             \
  "SELECT name FROM sqlite_master WHERE type='table' AND " \
  "name='schema_migrations'"

#define PEONY_SQLITE3_VERSION "SELECT SQLITE_VERSION() AS value"

// --------------------------------------------------------
#define PEONY_MYSQL_SCHEMA_MIGRATIONS_EXISTS                        \
  "SELECT table_name AS name FROM information_schema.tables WHERE " \
  "table_schema = DATABASE() AND table_name = 'schema_migrations'"
#define PEONY_MYSQL_VERSION "SELECT VERSION() AS value"