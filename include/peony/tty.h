#pragma once

#include <stdio.h>
#include <string.h>

#include <exception>
#include <mutex>
#include <optional>
#include <string>
#include <utility>

#include <errno.h>
#include <fcntl.h>
#include <termios.h>
#include <unistd.h>

// https://blog.mbedded.ninja/programming/operating-systems/linux/linux-serial-ports-using-c-cpp/
namespace peony {

class SerialPort {
 public:
  SerialPort(const std::string name);
  ~SerialPort();

 protected:
  std::optional<std::pair<size_t, size_t>> match(const std::string &payload,
                                                 const std::string &begin,
                                                 const std::string &end);
  virtual std::optional<std::pair<size_t, size_t>> match(
      const std::string &payload) const = 0;

 private:
  struct termios tty;
  char line[1 << 12];
  std::string payload;
  std::mutex locker;
};
}  // namespace peony
