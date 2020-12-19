#include "crypt.h"

std::string peony::crypt::uuid() {
  boost::uuids::uuid uid = boost::uuids::random_generator()();
  std::stringstream ss;
  ss << uid;
  return ss.str();
}

std::string peony::crypt::timestamp() {
  std::chrono::system_clock::time_point time = std::chrono::system_clock::now();
  std::time_t tt = std::chrono::system_clock::to_time_t(time);
  std::tm tm = *std::gmtime(&tt);
  std::stringstream ss;
  ss << std::put_time(&tm, "%Y%m%d%H%M%S");
  return ss.str();
}

std::string peony::crypt::random_str(std::string::size_type len) {
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

std::string peony::crypt::random_base64(const size_t len) {
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

unsigned int peony::crypt::random_uint(unsigned int min, unsigned int max) {
  thread_local static std::random_device rd;
  thread_local static std::mt19937 gen(rd());
  std::uniform_int_distribution<unsigned int> dis(min, max);
  return dis(gen);
}

double peony::crypt::random_double(unsigned int min, unsigned int max) {
  thread_local static std::random_device rd;
  thread_local static std::mt19937 gen(rd());
  std::uniform_real_distribution<double> dis(min, max);
  return dis(gen);
}