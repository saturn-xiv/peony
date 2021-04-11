#pragma once

#include <locale>
#include <optional>
#include <string>

#include <httplib.h>

namespace peony {
namespace http {
namespace request {
// https://tools.ietf.org/html/rfc6750
std::optional<std::string> token(const httplib::Request& req);

std::locale locale(const httplib::Request& req);
}  // namespace request
}  // namespace http
}  // namespace peony
