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

std::string peony::crypt::random_base64_string(const size_t len) {
  // std::independent_bits_engine<std::default_random_engine, CHAR_BIT, unsigned
  // char> rng; std::vector<unsigned char> buf(len);
  // std::generate(std::begin(buf), std::end(buf), std::ref(rng));
  // it.resize(boost::beast::detail::base64::encode(&it[0], &buf[0], len));

  void *const buf = malloc(sizeof(void *) * len);

  {
    FILE *fp;
    fp = fopen("/dev/urandom", "r");
    fread(buf, 1, len, fp);
    fclose(fp);
  }

  std::string it;
  it.resize(boost::beast::detail::base64::encoded_size(len));
  it.resize(boost::beast::detail::base64::encode(&it[0], buf, len));

  free(buf);
  return it;
}