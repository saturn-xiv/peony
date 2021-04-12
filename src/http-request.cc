#include "peony/http-request.h"

#include <curl/curl.h>
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

size_t curl_string_callback(void* contents, size_t size, size_t nmemb,
                            std::string* s) {
  size_t newLength = size * nmemb;
  try {
    s->append((char*)contents, newLength);
  } catch (std::bad_alloc& e) {
    return 0;
  }
  return newLength;
}
std::optional<std::string> get(const std::string url) {
  BOOST_LOG_TRIVIAL(debug) << "HTTP GET " << url;
  CURL* curl = curl_easy_init();
  if (curl) {
    std::string body;

    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, false);
    curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, true);
    curl_easy_setopt(curl, CURLOPT_URL, url);
    curl_easy_setopt(curl, CURLOPT_FOLLOWLOCATION, 1L);
    curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, curl_string_callback);
    curl_easy_setopt(curl, CURLOPT_WRITEDATA, &body);

    CURLcode res = curl_easy_perform(curl);
    if (res == CURLE_OK) {
      boost::algorithm::trim(body);
      BOOST_LOG_TRIVIAL(debug) << body;
      return body;
    }
    BOOST_LOG_TRIVIAL(error)
        << "get " << url << ": " << curl_easy_strerror(res);

    curl_easy_cleanup(curl);
  }
  return std::nullopt;
}
