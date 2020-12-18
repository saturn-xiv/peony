#ifndef PEONY_QUEUE_H_
#define PEONY_QUEUE_H_

#include <amqp.h>
#include <amqp_framing.h>
#include <amqp_tcp_socket.h>
#include <zmq.hpp>

#include "common.h"

namespace peony {
// https://github.com/alanxz/rabbitmq-c/tree/master/examples
namespace rabbitmq {
class Config {
 public:
  Config(const toml::table &root);

  operator toml::table() const;

 private:
  std::string host;
  unsigned short port;
  std::string user;
  std::string password;
  std::string virtual_host;
};
}  // namespace rabbitmq
namespace zeromq {
class Tcp {
 private:
  unsigned short port;
};
class Udp {
 private:
  unsigned short port;
};
class Socket {
 private:
  std::string file;
};
}  // namespace zeromq
}  // namespace peony
#endif