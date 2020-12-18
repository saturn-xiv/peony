#define BOOST_TEST_MODULE Crype Test

#include <boost/test/included/unit_test.hpp>

#include "crypt.h"

BOOST_AUTO_TEST_CASE(ssha512_test) {}

BOOST_AUTO_TEST_CASE(password_test) {}

BOOST_AUTO_TEST_CASE(encode_decode_test) {}

BOOST_AUTO_TEST_CASE(random_test) {
  std::cout << "UUID: " << peony::crypt::uuid() << std::endl;
  std::cout << "TIMESTAMP: " << peony::crypt::timestamp() << std::endl;

  std::cout << "STRING: " << peony::crypt::random_str(32) << std::endl;
  std::cout << "BASE64: " << peony::crypt::random_base64(32) << std::endl;
  std::cout << "UNSIGNED INT: " << peony::crypt::random_uint(1000, 2000)
            << std::endl;
  std::cout << "DOUBLE: " << peony::crypt::random_double(1000, 2000)
            << std::endl;
}