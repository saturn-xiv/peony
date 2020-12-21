#include "twilio.h"

std::shared_ptr<httplib::Client> peony::Twilio::open() const {
  auto cli = std::make_shared<httplib::Client>("https://api.twilio.com");
  cli->enable_server_certificate_verification(false);
  cli->set_basic_auth(this->account_sid.c_str(), this->auth_token.c_str());
  return cli;
}

// https://www.twilio.com/docs/usage/webhooks/sms-webhooks#type-1-incoming-message
nlohmann::json peony::Twilio::sms(
    const std::string &to, const std::string &message,
    const std::optional<std::string> callback) const {
  BOOST_LOG_TRIVIAL(info) << "send sms message to " << to;
  BOOST_LOG_TRIVIAL(debug) << message;
  auto cli = this->open();
  httplib::MultipartFormDataItems form;
  form.push_back({"To", to.c_str(), "", ""});
  form.push_back({"From", this->from.c_str(), "", ""});
  form.push_back({"Body", message.c_str(), "", ""});
  if (callback) {
    form.push_back({"StatusCallback", callback.value().c_str()});
  }

  std::stringstream path;
  path << "/2010-04-01/Accounts/" << this->account_sid << "/Messages.json";

  auto res = cli->Post(path.str().c_str(), form);
  if (!res) {
    std::stringstream ss;
    ss << res.error();
    throw std::runtime_error(ss.str());
  }
  if (res->status != 200 || res->status != 201) {
    std::stringstream ss;
    ss << "status: " << res->status << ", body: " << res.error();
    throw std::runtime_error(ss.str());
  }

  auto payload = nlohmann::json(res->body);
  BOOST_LOG_TRIVIAL(info) << "done.";
  return payload;
}

peony::Twilio::Twilio(const toml::table &root) {
  this->from = root["from"].value<std::string>().value();
  this->account_sid = root["account-sid"].value<std::string>().value();
  this->auth_token = root["auth-token"].value<std::string>().value();
}

peony::Twilio::operator toml::table() const {
  toml::table root;
  root.insert("account-sid", this->account_sid);
  root.insert("auth-token", this->auth_token);
  root.insert("from", this->from);
  return root;
}
