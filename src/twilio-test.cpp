#define BOOST_TEST_MODULE Twilio Test

#include <boost/test/included/unit_test.hpp>

#include "twilio.h"
#include "utils.h"

BOOST_AUTO_TEST_CASE(to_json_test) {
  peony::twilio::Config cfg("from", "sid", "token");
  nlohmann::json buf;
  nlohmann::to_json(buf, cfg);
  std::cout << buf.dump(4) << std::endl;
}

BOOST_AUTO_TEST_CASE(from_json_test) {
  const std::string buf = R"###(
    {
        "account_sid": "sss",
        "auth_token": "ttt",
        "from": "fff"
    }
)###";
  peony::twilio::Config cfg;
  auto js = nlohmann::json::parse(buf);
  nlohmann::from_json(js, cfg);
  std::cout << "SID: " << cfg << std::endl;
}

BOOST_AUTO_TEST_CASE(sms_test) {
  peony::utils::init_logging(false, true);
  toml::table root = toml::parse_file("config.toml");
  auto cfg = root["twilio"].as_table();
  auto cli = peony::twilio::Config(*cfg);
  cli.sms(((*cfg)["to"]).value<std::string>().value(), "Hello, peony!");
}