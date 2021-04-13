#include "peony/orm.h"

#define PEONY_TIMESTAMP_SIZE 14

void peony::orm::Logger::start_query(std::string const& query) {
  BOOST_LOG_TRIVIAL(debug) << query;
}

peony::orm::Logger* peony::orm::Logger::do_clone() const {
  return new Logger();
}
