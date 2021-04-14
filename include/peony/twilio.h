#pragma once

#include "peony/http-request.h"

namespace peony {

namespace twilio {

namespace sms {
struct Request {
  std::string body;
  std::string from;
  std::string to;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Request, body, from, to)

struct SubResourceUri {
  std::string media;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(SubResourceUri, media)
struct Response {
  std::string account_sid;
  std::string api_version;
  std::string body;
  std::string date_created;
  std::string sent;
  std::string date_updated;
  std::string direction;
  std::optional<int> error_code;
  std::optional<std::string> error_message;
  std::string from;
  std::string messaging_service_id;
  int num_media;
  int num_segments;
  std::optional<std::string> price;
  std::optional<std::string> price_unit;
  std::string sid;
  std::string status;
  std::vector<SubResourceUri> sub_resource_uris;
  std::string to;
  std::string uri;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(Response, account_sid, api_version, body,
                                   date_created, sent, date_updated, direction,
                                   error_code, error_message, from,
                                   messaging_service_id, num_media,
                                   num_segments, price, price_unit, sid, status,
                                   sub_resource_uris, to, uri)
}  // namespace sms

class Client {
 public:
  Client(const std::string& account_sid, const std::string& auth_token,
         const std::string& from)
      : account_sid(account_sid), auth_token(auth_token), from(from) {}

  peony::twilio::sms::Response sms(const std::string& to,
                                   const std::string& message);

 private:
  std::string account_sid;
  std::string auth_token;
  std::string from;
};

}  // namespace twilio
}  // namespace peony
