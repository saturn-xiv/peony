#define BOOST_TEST_MODULE Crypt Test

#include <boost/test/included/unit_test.hpp>

#include "peony/crypt.h"

BOOST_AUTO_TEST_CASE(random_test) {
  std::cout << "random string: " << peony::random::string(8) << std::endl
            << "random base64: " << peony::random::base64(8) << std::endl
            << "random integer: " << peony::random::integer(1000, 2000)
            << std::endl
            << "random double: " << peony::random::double_(10000, 20000)
            << std::endl
            << "uuid: " << peony::uuid() << std::endl
            << "timestamp: " << peony::timestamp() << std::endl;
}

BOOST_AUTO_TEST_CASE(password_test) {
  const auto hello = "Hello, Peony!";
  auto key = peony::Key(peony::random::base64(32));
  auto secret = key.password(hello);
  std::cout << "raw: " << strlen(hello) << " " << sizeof(hello) << std::endl
            << "password: " << sizeof(secret.first) << std::endl
            << "salt: " << sizeof(secret.second) << std::endl;

  BOOST_TEST(key.verify(secret.first, secret.second, hello));
  BOOST_TEST(!key.verify(secret.first, secret.second, "Hi, Peony!"));

  delete[] secret.first;
  delete[] secret.second;
}

BOOST_AUTO_TEST_CASE(enc_dec_test) {
  const auto hello = "Hello, Peony!";
  const auto len = strlen(hello);

  auto key = peony::Key(peony::random::base64(32));
  auto secret = key.encrypt((unsigned char*)hello, len);
  std::cout << "raw: " << strlen(hello) << " " << sizeof(hello) << std::endl
            << "secrets: " << sizeof(secret.first) << std::endl
            << "nonce: " << sizeof(secret.second) << std::endl;

  auto plain = key.decrypt(secret.first, secret.second, len);

  std::cout << "random key: " << peony::Key::generate_key() << std::endl
            << hello << " vs " << (char*)plain << std::endl;
  BOOST_TEST(strcmp((char*)plain, hello) == 0);

  delete[] secret.first;
  delete[] secret.second;
  delete[] plain;
}
