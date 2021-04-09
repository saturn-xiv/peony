#define BOOST_TEST_MODULE Tty Test

#include "peony/tty.h"

#include <boost/test/included/unit_test.hpp>

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

// BOOST_AUTO_TEST_CASE(tty_listen) {
//   int fd;
//   fd = open("/dev/ttyUSB0", O_RDWR);
//   if (fd == -1) {
//     perror("open serial error");
//     return;
//   }
//   struct termios opt;

//   if (tcgetattr(fd, &opt) != 0) {
//     perror("tcgetattr fd");
//     return;
//   }

//   tcflush(fd, TCIOFLUSH);
//   cfsetispeed(&opt, B9600);
//   cfsetospeed(&opt, B9600);

//   opt.c_cflag &= ~PARENB;
//   opt.c_cflag &= ~CSTOPB;
//   opt.c_cflag &= ~CSIZE;
//   opt.c_cflag |= CS8;

//   opt.c_cflag |= (CLOCAL | CREAD);

//   opt.c_lflag &= ~(ICANON | ECHO | ECHOE | ISIG);

//   opt.c_oflag &= ~OPOST;
//   opt.c_oflag &= ~(ONLCR | OCRNL);

//   opt.c_iflag &= ~(ICRNL | INLCR);
//   opt.c_iflag &= ~(IXON | IXOFF | IXANY);
//   opt.c_cc[VTIME] = 1;
//   opt.c_cc[VMIN] = 0;

//   tcflush(fd, TCIFLUSH);

//   if (tcsetattr(fd, TCSANOW, &opt) != 0) {
//     perror("tcsetattr fd");
//     return;
//   }
//   tcflush(fd, TCIOFLUSH);

//   char buffer[1024];
//   for (;;) {
//     memset(&buffer, '\0', sizeof(buffer));
//     int length;
//     int readByte;
//     readByte = read(fd, buffer, sizeof(buffer));
//     printf("read %i bytes, received %s\n", readByte, buffer);
//   }
// }
