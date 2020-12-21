#ifndef PEONY_CRAWLER_H_
#define PEONY_CRAWLER_H_

#include "env.h"

#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/log/trivial.hpp>
#include <pqxx/pqxx>

namespace peony {
class Crawler {
 public:
  Crawler(const std::shared_ptr<pqxx::connection> connection)
      : connection(connection) {}

  void get(const std::string& name, const std::string& home,
           const std::string& path) const;
  std::optional<std::pair<std::string, boost::posix_time::ptime>> latest(
      const std::string& name) const;
  void rolling(const unsigned short years) const;

 private:
  const std::shared_ptr<pqxx::connection> connection;
};
}  // namespace peony

#endif
