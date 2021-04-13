#include "peony/http-request.h"

#include <boost/algorithm/string.hpp>

peony::http::response::Ok::Ok()
    : created_at(boost::posix_time::microsec_clock::local_time()) {}

std::optional<std::string> peony::http::request::token(
    const httplib::Request& req) {
  const auto auth = "Authorization";
  const std::string bearer = "Bearer ";
  if (req.has_header(auth)) {
    auto val = req.get_header_value(auth);
    if (val.find(bearer) == 0) {
      return val.substr(bearer.length());
    }
  }
  return std::nullopt;
}

std::locale locale(const httplib::Request& req) {
  const auto key = "locale";
  // detect from params
  if (req.has_param(key)) {
    auto val = req.get_param_value(key);
    return std::locale(val);
  }
  // TODO: detect from cookie
  // TODO: detect from accept languages
  return std::locale();
}
