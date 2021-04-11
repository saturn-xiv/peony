#define BOOST_TEST_MODULE Jwt Test

#include <boost/test/included/unit_test.hpp>

#include <jwt-cpp/jwt.h>

#include "peony/jwt.h"

#include <iostream>

BOOST_AUTO_TEST_CASE(decode_test) {
    std::string token =
      "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXUyJ9.eyJpc3MiOiJhdXRoMCJ9."
      "AbIJTDMFc7yUa5MhvcP03nJPyCPzZtQcGEp-zWfOkEE";
  auto decoded = jwt::decode(token);

  for (auto& e : decoded.get_payload_claims()) {
    std::cout << e.first << " = " << e.second << std::endl;
  }

  {
    auto verifier = jwt::verify()
                        .allow_algorithm(jwt::algorithm::hs256{"secret"})
                        .with_issuer("auth0");

    verifier.verify(decoded);
  }
  {
    auto verifier = jwt::verify()
                        .allow_algorithm(jwt::algorithm::hs256{"secret"})
                        .with_issuer("auth1");

    verifier.verify(decoded);
  }
  {
    auto verifier = jwt::verify()
                        .allow_algorithm(jwt::algorithm::hs256{"secret1"})
                        .with_issuer("auth0");

    verifier.verify(decoded);
  }
}

// BOOST_AUTO_TEST_CASE(sum_verify_test) {
//   auto j = peony::Jwt("1234567890123456789012", "issuer");
//   auto j1 = peony::Jwt("1234567890123456789012", "issuer1");
//   auto j2 = peony::Jwt("1234567890123456789011", "issuer");
//   const auto ttl = std::chrono::seconds(60);
//   const auto key = "uid";
//   const auto val = "hello";

//   const std::string token = j.sum(key, val, ttl);
//   std::cout << "token: " << token << std::endl;
//   //   {
//   //     const std::optional<std::string> v = j.verify(token, key);
//   //     BOOST_TEST(v == val);
//   //   }
//   //   BOOST_TEST(j1.verify(token, key) == std::nullopt);
//   //   BOOST_TEST(j2.verify(token, key) == std::nullopt);
// }
