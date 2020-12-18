#ifndef PEONY_CRAWLER_H_
#define PEONY_CRAWLER_H_

#include "common.h"

#include <pqxx/pqxx>

namespace peony {
class Crawler {
 public:
  Crawler(const std::shared_ptr<pqxx::connection> connection,
          std::vector<std::pair<std::string, std::string>> sources);
  Crawler(const std::shared_ptr<pqxx::connection> connection,
          const toml::table &root);
  void execute() const;
  void execute(const std::string &name) const;
  std::optional<std::pair<std::string, boost::posix_time::ptime>> latest(
      const std::string &name) const;

 private:
  void execute(const std::string &name, const std::string &url) const;

  const std::shared_ptr<pqxx::connection> connection;
  std::vector<std::pair<std::string, std::string>> sources;
};
}  // namespace peony

#endif
