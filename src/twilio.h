#ifndef PEONY_TWILIO_H_
#define PEONY_TWILIO_H_

#include <string>

#include "env.h"

#include <boost/log/trivial.hpp>

#include <cpprest/http_client.h>
#include <nlohmann/json.hpp>
#include <toml.hpp>

namespace peony {
namespace twilio {

class Config {
 public:
  Config() {}
  Config(std::string from, std::string account_sid, std::string auth_token)
      : from(from), account_sid(account_sid), auth_token(auth_token) {}
  Config(const toml::table &root);
  friend std::ostream &operator<<(std::ostream &out, const Config &self) {
    out << self.account_sid;
    return out;
  }
  web::http::client::http_client connect() const;

  operator toml::table() const;
  friend class Client;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Config, from, account_sid, auth_token)

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
