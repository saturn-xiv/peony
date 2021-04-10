#define BOOST_TEST_MODULE Tty Test

#include "peony/tty.h"

#include <boost/test/included/unit_test.hpp>

class SerialPort : public peony::SerialPort {
 public:
  SerialPort(const std::string& name) : peony::SerialPort(name) {}

 protected:
  std::optional<std::pair<size_t, size_t>> on_match(
      const std::string& payload) const override {
    const std::string begin = "[";
    const std::string end = "]";
    return this->match(payload, begin, end);
  }
};

BOOST_AUTO_TEST_CASE(tty_listen) {
  SerialPort tty("ttyUSB0");
  tty.listen(10000);
}
