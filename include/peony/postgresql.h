#pragma once

#include "peony/orm.h"

#include <soci/postgresql/soci-postgresql.h>

namespace peony {
/**
 *
 * https://www.postgresql.org/docs/current/runtime-config-logging.html
 * /var/lib/postgres/data/postgresql.conf: log_statement = 'all'
 * journalctl -f -u postgresql
 *
 */
class PostgreSql {};
}  // namespace peony
