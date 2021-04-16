#define BOOST_TEST_MODULE Twilio Test

#include <boost/test/included/unit_test.hpp>

#include "peony/twilio.h"

BOOST_AUTO_TEST_CASE(models_test) {
  {
    peony::twilio::sms::Request req;
    nlohmann::json js = req;
    std::cout << js.dump() << std::endl;
  }
  {
    peony::twilio::sms::Response res;
    nlohmann::json js = res;
    std::cout << js.dump() << std::endl;
  }
}

BOOST_AUTO_TEST_CASE(sms_test) {}
