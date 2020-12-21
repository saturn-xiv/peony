#ifndef PEONY_TWILIO_H_
#define PEONY_TWILIO_H_

#include <string>

#include "env.h"

#include <boost/log/trivial.hpp>

namespace peony {

class Twilio {
 public:
  Twilio() {}
  Twilio(std::string from, std::string account_sid, std::string auth_token)
      : from(from), account_sid(account_sid), auth_token(auth_token) {}
  Twilio(const toml::table &root);

  nlohmann::json sms(
      const std::string &to, const std::string &message,
      const std::optional<std::string> callback = std::nullopt) const;

  friend std::ostream &operator<<(std::ostream &out, const Twilio &self) {
    out << self.account_sid;
    return out;
  }
  operator toml::table() const;
  friend class Twilio;
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Twilio, from, account_sid, auth_token)

 private:
  std::shared_ptr<httplib::Client> open() const;

  std::string from;
  std::string account_sid;
  std::string auth_token;
};

}  // namespace peony
#endif
