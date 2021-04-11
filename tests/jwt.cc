#define BOOST_TEST_MODULE Jwt Test

#include <boost/test/included/unit_test.hpp>

#include "peony/jwt.h"

#include <iostream>

BOOST_AUTO_TEST_CASE(sum_verify_test) {
  auto j = peony::Jwt("1234567890123456789012", "issuer");
  auto j1 = peony::Jwt("1234567890123456789012", "issuer1");
  auto j2 = peony::Jwt("1234567890123456789011", "issuer");

  const std::string val = "hello";
  const std::string key = "msg";
  std::unordered_map<std::string, nlohmann::json> payload{{key, val}};
  const auto ttl = std::chrono::seconds(60);

  const auto token = j.sum(payload, ttl);
  std::cout << "token: " << token << std::endl;
  {
    const auto v = j.verify(token);
    std::cout << v << std::endl;
    BOOST_TEST(v[key] == val);
  }
  {
    const auto v = j1.verify(token);
    std::cout << "bad key: " << v << std::endl;
    BOOST_TEST(v.empty());
  }
  {
    const auto v = j2.verify(token);
    std::cout << "bad issuer: " << v << std::endl;
    BOOST_TEST(v.empty());
  }
}
