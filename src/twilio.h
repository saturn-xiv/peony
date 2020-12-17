#ifndef PEONY_TWILIO_H_
#define PEONY_TWILIO_H_

#include "common.h"

namespace peony {
namespace twilio {

class Config {
 public:
  Config(const toml::table &root);
  web::http::client::http_client connect() const;

  operator toml::table() const;
  friend class Client;

 private:
  std::string from;
  std::string account_sid;
  std::string auth_token;
};

class Client {
 public:
  Client(const Config config);
  web::json::value sms(const std::string &to, const std::string &message) const;

 private:
  web::json::value get(const web::uri_builder &builder) const;
  web::json::value form(const web::uri_builder &builder,
                        const utf8string &body) const;

  const Config config;
};

}  // namespace twilio
}  // namespace peony
#endif
