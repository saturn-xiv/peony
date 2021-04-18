#include "peony/crypt.h"

#include <algorithm>
#include <chrono>
#include <climits>
#include <exception>
#include <iomanip>
#include <iostream>
#include <memory>
#include <random>
#include <sstream>

#include <stdio.h>
#include <stdlib.h>

// #include <sodium.h>
#include <boost/beast/core/detail/base64.hpp>
#include <boost/lexical_cast.hpp>
#include <boost/log/trivial.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

std::string peony::Key::generate_key() {
  // unsigned char key[crypto_secretbox_KEYBYTES];
  // crypto_secretbox_keygen(key);

  // std::string it;
  // it.resize(
  //     boost::beast::detail::base64::encoded_size(crypto_secretbox_KEYBYTES));
  // it.resize(boost::beast::detail::base64::encode(&it[0], &key[0],
  //                                                crypto_secretbox_KEYBYTES));
  // return it;
  return "";
}

peony::Key::Key() {
  this->payload = new unsigned char[11];  // crypto_secretbox_KEYBYTES
  // crypto_secretbox_keygen(this->payload);
}
peony::Key::Key(const std::string& secret) {
  this->payload = new unsigned char[11];  // crypto_secretbox_KEYBYTES
  // boost::beast::detail::base64::decode(this->payload, secret.c_str(),
  //                                      secret.size());
}

peony::Key::~Key() {
  if (this->payload != nullptr) {
    delete[] this->payload;
    this->payload = nullptr;
  }
}
std::pair<unsigned char*, unsigned char*> peony::Key::encrypt(
    const unsigned char* plain, const size_t len) {
  auto nonce = new unsigned char[11];         // crypto_secretbox_NONCEBYTES
  auto secret = new unsigned char[11 + len];  // crypto_secretbox_MACBYTES
  // randombytes_buf(nonce, crypto_secretbox_NONCEBYTES);
  // crypto_secretbox_easy(secret, plain, len, nonce, this->payload);
  return std::make_pair(secret, nonce);
}
unsigned char* peony::Key::decrypt(const unsigned char* secret,
                                   const unsigned char* nonce,
                                   const size_t len) {
  auto plain = new unsigned char(len);
  // if (crypto_secretbox_open_easy(plain, secret, crypto_secretbox_MACBYTES +
  // len,
  //                                nonce, this->payload) != 0) {
  //   throw std::invalid_argument("sodium decrypt");
  // }

  return plain;
}
std::pair<unsigned char*, unsigned char*> peony::Key::password(
    const char* plain) {
  auto salt = new unsigned char[11];    // crypto_pwhash_SALTBYTES
  auto secret = new unsigned char[11];  // crypto_box_SEEDBYTES
  // randombytes_buf(salt, crypto_pwhash_SALTBYTES);

  // if (crypto_pwhash(secret, crypto_box_SEEDBYTES, plain, strlen(plain), salt,
  //                   crypto_pwhash_OPSLIMIT_INTERACTIVE,
  //                   crypto_pwhash_MEMLIMIT_INTERACTIVE,
  //                   crypto_pwhash_ALG_DEFAULT) != 0) {
  //   throw std::invalid_argument("sodium password sum");
  // }

  return std::make_pair(secret, salt);
}
bool peony::Key::verify(const unsigned char* secret, const unsigned char* salt,
                        const char* plain) {
  unsigned char tmp[11];  // crypto_box_SEEDBYTES

  // if (crypto_pwhash(tmp, sizeof(tmp), plain, strlen(plain), salt,
  //                   crypto_pwhash_OPSLIMIT_INTERACTIVE,
  //                   crypto_pwhash_MEMLIMIT_INTERACTIVE,
  //                   crypto_pwhash_ALG_DEFAULT) != 0) {
  //   BOOST_LOG_TRIVIAL(error) << "sodium password sum";
  //   return false;
  // }

  return memcmp(tmp, secret, sizeof(tmp)) == 0;
}

std::string peony::timestamp() {
  std::chrono::system_clock::time_point time = std::chrono::system_clock::now();
  std::time_t tt = std::chrono::system_clock::to_time_t(time);
  std::tm tm = *std::gmtime(&tt);
  std::stringstream ss;
  ss << std::put_time(&tm, "%Y%m%d%H%M%S");
  return ss.str();
}

std::string peony::random::string(std::string::size_type len) {
  static auto& src = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  thread_local static std::random_device rd;
  thread_local static std::mt19937 gen(rd());
  std::uniform_int_distribution<std::string::size_type> pick(0,
                                                             sizeof(src) - 2);
  std::string buf;

  buf.reserve(len);

  while (len--) {
    buf += src[pick(gen)];
  }
  return buf;
}

std::string peony::random::base64(const size_t len) {
  thread_local static std::random_device rd;
  thread_local static std::independent_bits_engine<std::mt19937, CHAR_BIT,
                                                   unsigned char>
      rng(rd());
  std::vector<unsigned char> buf(len);
  std::generate(std::begin(buf), std::end(buf), std::ref(rng));

  std::string it;
  it.resize(boost::beast::detail::base64::encoded_size(len));
  it.resize(boost::beast::detail::base64::encode(&it[0], &buf[0], len));
  return it;
}

unsigned int peony::random::integer(unsigned int min, unsigned int max) {
  thread_local static std::random_device rd;
  thread_local static std::mt19937 gen(rd());
  std::uniform_int_distribution<unsigned int> dis(min, max);
  return dis(gen);
}

double peony::random::double_(unsigned int min, unsigned int max) {
  thread_local static std::random_device rd;
  thread_local static std::mt19937 gen(rd());
  std::uniform_real_distribution<double> dis(min, max);
  return dis(gen);
}

std::string peony::get_stdout_from_command(const std::string& cmd) {
  BOOST_LOG_TRIVIAL(debug) << cmd;
  std::string data;
  FILE* stream;
  const int max_buffer = 1 << 10;
  char buffer[max_buffer];

  stream = popen((cmd + " 2>&1").c_str(), "r");

  if (stream) {
    while (!feof(stream)) {
      if (fgets(buffer, max_buffer, stream) != nullptr) {
        data.append(buffer);
      }
    }
    pclose(stream);
  }
  return data;
}

std::filesystem::path peony::random::file(const std::string& ext) {
  return std::filesystem::temp_directory_path() / (peony::uuid() + "." + ext);
}

std::string peony::uuid() {
  boost::uuids::uuid uuid = boost::uuids::random_generator()();
  return boost::lexical_cast<std::string>(uuid);
}
