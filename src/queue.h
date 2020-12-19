#ifndef PEONY_QUEUE_H_
#define PEONY_QUEUE_H_

#include <string>

#include "env.h"

#include <amqp.h>
#include <amqp_framing.h>
#include <amqp_tcp_socket.h>
#include <zmq.hpp>

namespace peony {

// https://github.com/alanxz/rabbitmq-c/tree/master/examples
class RabbitMQ {
 public:
  RabbitMQ(std::string host = "127.0.0.1", unsigned short port = 5672,
           std::string user = "guest", std::string password = "guest",
           std::string virtual_host = PEONY_PROJECT_NAME, size_t pool_size = 32)
      : host(host),
        port(port),
        user(user),
        password(password),
        virtual_host(virtual_host),
        pool_size(pool_size) {}
  RabbitMQ(const toml::table &root);

  friend std::ostream &operator<<(std::ostream &out, const RabbitMQ &self) {
    out << self.user << '@' << self.host << ':' << self.port << '/'
        << self.virtual_host;
    return out;
  }

  operator toml::table() const;

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(RabbitMQ, host, port, user, password,
                                 virtual_host, pool_size)

 private:
  std::string host;
  unsigned short port;
  std::string user;
  std::string password;
  std::string virtual_host;
  size_t pool_size;
};

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