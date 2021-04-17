// https://www.twilio.com/docs/usage/api

#include "peony/twilio.h"

std::shared_ptr<peony::twilio::sms::Response> peony::twilio::Client::sms(
    const std::string& to, const std::string& message) {
  std::stringstream url;
  url << this->api_host << this->account_sid << "/Messages.json";

  cpr::Response res = cpr::Post(
      cpr::Url{url.str()},
      cpr::Authentication{this->account_sid.c_str(), this->auth_token.c_str()},
      cpr::Payload{{"From", this->from}, {"Body", message}, {"To", to}});
  BOOST_LOG_TRIVIAL(debug) << res.status_code << " "
                           << res.header["content-type"] << " " << res.text;
  if (res.status_code != 201) {
    throw std::invalid_argument(res.text);
  }
  const auto js = nlohmann::json::parse(res.text);
  const auto it = std::make_shared<peony::twilio::sms::Response>();
  *it = js.get<peony::twilio::sms::Response>();

  return it;
}
