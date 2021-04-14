#include "peony/http-request.h"

#include <curl/curl.h>
#include <boost/algorithm/string.hpp>
#include <boost/asio/connect.hpp>
#include <boost/asio/ip/tcp.hpp>
#include <boost/asio/ssl/error.hpp>
#include <boost/asio/ssl/stream.hpp>
#include <boost/beast/core.hpp>
#include <boost/beast/http.hpp>
#include <boost/beast/ssl.hpp>
#include <boost/beast/version.hpp>

size_t write2string(void* ptr, size_t size, size_t nmemb, std::string* s) {
  s->append(static_cast<char*>(ptr), size * nmemb);
  return size * nmemb;
}

std::string peony::http::client::post(const std::string& url,
                                      const std::string& content_type,
                                      const std::string& body) {
  std::string buf;

  auto curl = curl_easy_init();

  if (!curl) {
    throw std::runtime_error("init curl easy failed");
  }
  curl_easy_setopt(curl, CURLOPT_URL, url.c_str());
  curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, write2string);
  curl_easy_setopt(curl, CURLOPT_WRITEDATA, &buf);
  curl_easy_setopt(curl, CURLOPT_POSTFIELDS, body.c_str());
  curl_easy_setopt(curl, CURLOPT_POSTFIELDSIZE, (long)body.length());

  auto res = curl_easy_perform(curl);

  if (res != CURLE_OK) {
    curl_easy_cleanup(curl);
    throw std::invalid_argument(curl_easy_strerror(res));
  }
  curl_easy_cleanup(curl);
  return buf;
}
std::string peony::http::client::get(const std::string& url) { return ""; }

// std::string peony::http::client::gets(const std::string& host,
//                                       const std::string& path,
//                                       const unsigned int port) {
//   boost::asio::io_context ioc;

//   boost::asio::ssl::context ctx(boost::asio::ssl::context::tlsv12_client);
//   ctx.set_verify_mode(boost::asio::ssl::verify_none);

//   boost::asio::ip::tcp::resolver resolver(ioc);
//   boost::beast::ssl_stream<boost::beast::tcp_stream> stream(ioc, ctx);

//   // if (!SSL_set_tlsext_host_name(stream.native_handle(), host)) {
//   //   boost::beast::error_code ec{static_cast<int>(::ERR_get_error()),
//   //                               boost::asio::error::get_ssl_category()};
//   //   throw boost::beast::system_error{ec};
//   // }

//   // FIXME
//   auto const results = resolver.resolve(host, "443");

//   boost::beast::get_lowest_layer(stream).connect(results);

//   stream.handshake(boost::asio::ssl::stream_base::client);

//   boost::beast::http::request<boost::beast::http::string_body> req{
//       boost::beast::http::verb::get, path, 11};
//   req.set(boost::beast::http::field::host, host);
//   req.set(boost::beast::http::field::user_agent, BOOST_BEAST_VERSION_STRING);

//   boost::beast::http::write(stream, req);

//   boost::beast::flat_buffer buffer;

//   boost::beast::http::response<boost::beast::http::dynamic_body> res;

//   boost::beast::http::read(stream, buffer, res);

//   const auto status = res.result();
//   const std::string body =
//   boost::beast::buffers_to_string(res.body().data());

//   boost::beast::error_code ec;
//   stream.shutdown(ec);
//   if (ec == boost::asio::error::eof) {
//     //
//     http://stackoverflow.com/questions/25587403/boost-asio-ssl-async-shutdown-always-finishes-with-an-error
//     ec = {};
//   }
//   if (ec) {
//     throw boost::beast::system_error{ec};
//   }
//   if (status != boost::beast::http::status::ok) {
//     throw std::invalid_argument(body);
//   }
//   return body;
// }

// std::string peony::http::client::posts(const std::string& host,
//                                        const std::string& path,
//                                        const std::string& payload,
//                                        const unsigned int port) {
//   boost::asio::io_context ioc;

//   boost::asio::ssl::context ctx(boost::asio::ssl::context::tlsv12_client);
//   ctx.set_verify_mode(boost::asio::ssl::verify_none);

//   boost::asio::ip::tcp::resolver resolver(ioc);
//   boost::beast::ssl_stream<boost::beast::tcp_stream> stream(ioc, ctx);

//   // if (!SSL_set_tlsext_host_name(stream.native_handle(), host)) {
//   //   boost::beast::error_code ec{static_cast<int>(::ERR_get_error()),
//   //                               boost::asio::error::get_ssl_category()};
//   //   throw boost::beast::system_error{ec};
//   // }

//   // FIXME
//   auto const results = resolver.resolve(host, "443");

//   boost::beast::get_lowest_layer(stream).connect(results);

//   stream.handshake(boost::asio::ssl::stream_base::client);

//   boost::beast::http::request<boost::beast::http::string_body> req{
//       boost::beast::http::verb::post, path, 11};
//   req.set(boost::beast::http::field::host, host);
//   req.set(boost::beast::http::field::user_agent, BOOST_BEAST_VERSION_STRING);
//   req.set(boost::beast::http::field::content_type,
//   PEONY_APPLICATION_JSON_UTF8); req.body() = payload;

//   boost::beast::http::write(stream, req);

//   boost::beast::flat_buffer buffer;

//   boost::beast::http::response<boost::beast::http::dynamic_body> res;

//   boost::beast::http::read(stream, buffer, res);

//   const auto status = res.result();
//   const std::string body =
//   boost::beast::buffers_to_string(res.body().data());

//   boost::beast::error_code ec;
//   stream.shutdown(ec);
//   if (ec == boost::asio::error::eof) {
//     //
//     http://stackoverflow.com/questions/25587403/boost-asio-ssl-async-shutdown-always-finishes-with-an-error
//     ec = {};
//   }
//   if (ec) {
//     throw boost::beast::system_error{ec};
//   }
//   if (status != boost::beast::http::status::ok) {
//     throw std::invalid_argument(body);
//   }
//   return body;
// }

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
