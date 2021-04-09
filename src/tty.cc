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

#include <chrono>
#include <exception>
#include <sstream>
#include <string>
#include <thread>

peony::SerialPort::SerialPort(const std::string &name)
    : name(name), port(open("/dev/ttyUSB0", O_RDWR)) {
  printf("######## %i\n", port);
  if (port == -1) {
    perror("open serial error");
    return;
  }
  struct termios opt;

  if (tcgetattr(port, &opt) != 0) {
    perror("tcgetattr fd");
    return;
  }

  tcflush(port, TCIOFLUSH);
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

  tcflush(port, TCIFLUSH);

  if (tcsetattr(port, TCSANOW, &opt) != 0) {
    perror("tcsetattr fd");
    return;
  }
  tcflush(port, TCIOFLUSH);

  // port = open("/dev/ttyUSB0", O_RDWR);
  // // if (flock(port, LOCK_EX | LOCK_NB) == -1) {
  // //   std::stringstream ss;
  // //   ss << name << " is already locked by another process";

  // //   throw std::invalid_argument(ss.str());
  // // }
  // if (port < 0) {
  //   std::stringstream ss;
  //   ss << "open serial port " << name << "(" << errno
  //      << "): " << strerror(errno);
  //   throw std::invalid_argument(ss.str());
  // }

  // struct termios tty;
  // if (tcgetattr(port, &tty) != 0) {
  //   std::stringstream ss;
  //   ss << "tcgetattr " << name << "(" << errno << "): " << strerror(errno);
  //   throw std::invalid_argument(ss.str());
  // }

  // cfsetispeed(&tty, B9600);
  // cfsetospeed(&tty, B9600);

  // tty.c_cflag &= ~PARENB;
  // tty.c_cflag &= ~CSTOPB;
  // tty.c_cflag &= ~CSIZE;
  // tty.c_cflag |= CS8;
  // tty.c_cflag |= (CLOCAL | CREAD);
  // tty.c_lflag &= ~(ICANON | ECHO | ECHOE | ISIG);
  // tty.c_oflag &= ~OPOST;
  // tty.c_oflag &= ~(ONLCR | OCRNL);
  // tty.c_iflag &= ~(ICRNL | INLCR);
  // tty.c_iflag &= ~(IXON | IXOFF | IXANY);
  // tty.c_cc[VTIME] = 1;
  // tty.c_cc[VMIN] = 0;

  // tcflush(port, TCIFLUSH);
  // // Save tty settings, also checking for error
  // if (tcsetattr(port, TCSANOW, &tty) != 0) {
  //   std::stringstream ss;
  //   ss << "tcsetattr " << name << "(" << errno << "): " << strerror(errno);
  //   throw std::invalid_argument(ss.str());
  // }

  printf("######## %i\n", port);
}

peony::SerialPort::~SerialPort() {
  std::lock_guard<std::mutex> locker(this->locker);
  close(port);
}

void peony::SerialPort::send(const std::string &line) {
  while (true) {
    BOOST_LOG_TRIVIAL(info) << "write \"" << line << "\" to " << name;
    {
      std::lock_guard<std::mutex> locker(this->locker);
      const auto len = write(port, line.c_str(), sizeof(line));
      if (len >= 0) {
        BOOST_LOG_TRIVIAL(info) << "written " << name << " " << len << " bytes";
        return;
      }
    }
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
  }
}

void peony::SerialPort::listen(const uint16_t port) {
  char buffer[1024];
  printf("######## %i\n", port);
  // for (;;) {
  //   memset(&buffer, '\0', sizeof(buffer));
  //   int length;
  //   int readByte;
  //   readByte = read(port, buffer, sizeof(buffer));
  //   printf("read %i bytes, received %s\n", readByte, buffer);
  // }

  // std::string payload;
  // char line[1 << 8];

  // while (true) {
  //   memset(&line, '\0', sizeof(line));
  //   std::this_thread::sleep_for(std::chrono::milliseconds(1000 * 5));
  //   {
  //     std::lock_guard<std::mutex> locker(this->locker);
  //     int len = read(port, &line, sizeof(line));
  //     if (len < 0) {
  //       BOOST_LOG_TRIVIAL(error)
  //           << "read from " << name << "(" << errno << "): " <<
  //           strerror(errno);
  //       continue;
  //     }

  //     BOOST_LOG_TRIVIAL(info) << "received " << len << " bytes: " << line;
  //     payload += std::string(line, len);

  //     const auto size = this->on_match(payload);
  //     if (size) {
  //       const auto it = size.value();
  //       const auto msg = payload.substr(it.first, it.second - it.first);
  //       BOOST_LOG_TRIVIAL(info) << "match " << name << ": " << msg;
  //       try {
  //         // TODO publish msg
  //         // this->execute(msg);
  //       } catch (const std::exception &e) {
  //         BOOST_LOG_TRIVIAL(error) << "handle " << name << ": " << e.what();
  //       }
  //       BOOST_LOG_TRIVIAL(debug)
  //           << "split " << name << " :" << payload.substr(0, it.second);
  //       payload = payload.substr(it.second);
  //       if (payload.length() > 0) {
  //         BOOST_LOG_TRIVIAL(debug)
  //             << "remain " << name << " :" << payload.substr(0, it.second);
  //       }
  //     }

  //     if (payload.length() >= (1 << 12)) {
  //       BOOST_LOG_TRIVIAL(warning)
  //           << "clear buffer " << name << ": " << payload;
  //       payload.clear();
  //     }
  //   }
  // }
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
