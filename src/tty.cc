#include "peony/tty.h"

#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/file.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <termios.h>
#include <unistd.h>

peony::SerialPort::SerialPort(const std::string &name)
    : name(name), tty(open((std::string("/dev/") + name).c_str(), O_RDWR)) {
  // if (flock(port, LOCK_EX | LOCK_NB) == -1) {
  //   std::stringstream ss;
  //   ss << name << " is already locked by another process";

  //   throw std::invalid_argument(ss.str());
  // }
  if (tty < 0) {
    std::stringstream ss;
    ss << "open serial port " << this->name << "(" << errno
       << "): " << strerror(errno);
    throw std::invalid_argument(ss.str());
  }

  struct termios opt;

  if (tcgetattr(this->tty, &opt) != 0) {
    std::stringstream ss;
    ss << "tcgetattr " << this->name << "(" << errno
       << "): " << strerror(errno);
    throw std::invalid_argument(ss.str());
  }

  cfsetispeed(&opt, B9600);
  cfsetospeed(&opt, B9600);

  opt.c_cflag &= ~PARENB;
  opt.c_cflag &= ~CSTOPB;
  opt.c_cflag &= ~CSIZE;
  opt.c_cflag |= CS8;

  opt.c_cflag |= (CLOCAL | CREAD);

  opt.c_lflag &= ~(ICANON | ECHO | ECHOE | ISIG);

  opt.c_oflag &= ~OPOST;
  opt.c_oflag &= ~(ONLCR | OCRNL);

  opt.c_iflag &= ~(ICRNL | INLCR);
  opt.c_iflag &= ~(IXON | IXOFF | IXANY);
  opt.c_cc[VTIME] = 1;
  opt.c_cc[VMIN] = 0;

  if (tcsetattr(this->tty, TCSANOW, &opt) != 0) {
    std::stringstream ss;
    ss << "tcsetattr " << this->name << "(" << errno
       << "): " << strerror(errno);
    throw std::invalid_argument(ss.str());
  }
  tcflush(this->tty, TCIOFLUSH);
}

peony::SerialPort::~SerialPort() {
  std::lock_guard<std::mutex> locker(this->locker);
  close(this->tty);
}

void peony::SerialPort::send(const std::string &line) {
  while (true) {
    BOOST_LOG_TRIVIAL(info) << "write \"" << line << "\" to " << this->name;
    {
      std::lock_guard<std::mutex> locker(this->locker);
      const auto len = write(this->tty, line.c_str(), sizeof(line));
      if (len >= 0) {
        BOOST_LOG_TRIVIAL(info)
            << "written " << this->name << " " << len << " bytes";
        return;
      }
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(200));
  }
}

void peony::SerialPort::listen(const uint16_t port) {
  std::string payload;
  char line[1 << 10];

  zmq::context_t ctx;
  zmq::socket_t sock(ctx, zmq::socket_type::pub);

  {
    std::stringstream ss;
    ss << "tcp://*:" << port;
    const auto addr = ss.str();
    BOOST_LOG_TRIVIAL(info) << "bind " << this->name << " to tcp " << addr;
    sock.bind(addr);
  }

  std::this_thread::sleep_for(std::chrono::milliseconds(20));

  while (true) {
    memset(&line, '\0', sizeof(line));
    std::this_thread::sleep_for(std::chrono::milliseconds(20));
    {
      std::lock_guard<std::mutex> locker(this->locker);
      const int len = read(this->tty, &line, sizeof(line));

      if (len == 0) {
        continue;
      }

      if (len < 0) {
        BOOST_LOG_TRIVIAL(error) << "read from " << this->name << "(" << errno
                                 << "): " << strerror(errno);
        continue;
      }

      BOOST_LOG_TRIVIAL(info) << "received " << len << " bytes: " << line;
      this->payload += std::string(line, len);

      for (;;) {
        const auto size = this->on_match(payload);
        if (!size) {
          break;
        }
        const auto it = size.value();
        const auto msg = this->payload.substr(it.first, it.second - it.first);
        BOOST_LOG_TRIVIAL(info) << "match " << name << ": " << msg;
        sock.send(zmq::buffer(msg));
        BOOST_LOG_TRIVIAL(debug)
            << "split " << name << ": " << payload.substr(0, it.second);
        this->payload = this->payload.substr(it.second);
        if (this->payload.length() > 0) {
          BOOST_LOG_TRIVIAL(debug)
              << "remain " << name << ": " << this->payload;
        }
      }

      if (this->payload.length() >= (1 << 12)) {
        BOOST_LOG_TRIVIAL(warning)
            << "clear buffer " << name << ": " << this->payload;
        this->payload.clear();
      }
    }
  }
}

std::optional<std::pair<size_t, size_t>> peony::SerialPort::match(
    const std::string &payload, const std::string &begin,
    const std::string &end) const {
  const auto b = payload.find(begin);
  if (b == std::string::npos) {
    return std::nullopt;
  }
  const auto e = payload.find(end);
  if (e == std::string::npos) {
    return std::nullopt;
  }
  return std::make_pair(b, e + end.length());
}
