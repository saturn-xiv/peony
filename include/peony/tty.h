#pragma once

#include <cstdint>
#include <mutex>
#include <optional>
#include <utility>

#include <boost/log/trivial.hpp>

// https://blog.mbedded.ninja/programming/operating-systems/linux/linux-serial-ports-using-c-cpp/
namespace peony {

class SerialPort {
 public:
  SerialPort(const std::string &name);
  ~SerialPort();

  void send(const std::string &line);
  void listen(const uint16_t port);

 protected:
  std::optional<std::pair<size_t, size_t>> match(const std::string &payload,
                                                 const std::string &begin,
                                                 const std::string &end) const;
  virtual std::optional<std::pair<size_t, size_t>> on_match(
      const std::string &payload) const = 0;

 private:
  const std::string name;
  const int tty;
  std::string payload;
  std::mutex locker;
};
}  // namespace peony
