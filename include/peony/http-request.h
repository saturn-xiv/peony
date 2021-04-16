#pragma once

#include "peony/theme.h"

#include <httplib.h>
#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/log/trivial.hpp>

namespace peony {
namespace http {
namespace client {

std::string post(const std::string& url, const std::string& content_type,
                 const std::string& body);
std::string get(const std::string& url);

// std::string gets(const std::string& host, const std::string& path,
//                  const unsigned int port = 443);
// std::string posts(const std::string& host, const std::string& path,
//                   const std::string& body, const unsigned int port = 443);

}  // namespace client

namespace response {
class Ok {
 public:
  Ok();
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Ok, created_at)
 private:
  boost::posix_time::ptime created_at;
};
}  // namespace response

namespace request {
// https://tools.ietf.org/html/rfc6750
std::optional<std::string> token(const httplib::Request& req);

std::locale locale(const httplib::Request& req);
}  // namespace request
}  // namespace http
}  // namespace peony
