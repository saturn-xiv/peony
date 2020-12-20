#ifndef PEONY_QUEUE_H_
#define PEONY_QUEUE_H_

#include <iostream>
#include <string>
#include <string_view>
#include <unordered_map>
#include <utility>
#include <vector>

#include "env.h"

#include <amqp.h>
#include <amqp_framing.h>
#include <amqp_tcp_socket.h>
#include <boost/log/trivial.hpp>
#include <zmq.hpp>

#include "crypt.h"

namespace peony {
namespace queue {

class Consumer {
 public:
  virtual void handle(const nlohmann::json &payload) = 0;
};

class Echo : public Consumer {
 public:
  void handle(const nlohmann::json &payload) override {
    BOOST_LOG_TRIVIAL(info) << "receive message: " << payload.dump(4);
  }
};

}  // namespace queue

namespace rabbitmq {

class Config {
 public:
  Config(std::string host = PEONY_DEFAULT_HOST, unsigned short port = 5672,
         std::string user = "guest", std::string password = "guest",
         std::string virtual_host = PEONY_PROJECT_NAME)
      : host(host),
        port(port),
        user(user),
        password(password),
        virtual_host(virtual_host) {}
  Config(const toml::table &root);

  friend class Connection;
  friend std::ostream &operator<<(std::ostream &out, const Config &self) {
    out << self.user << '@' << self.host << ':' << self.port << '/'
        << self.virtual_host;
    return out;
  }
  operator toml::table() const;

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Config, host, port, user, password,
                                 virtual_host)

 private:
  std::string host;
  unsigned short port;
  std::string user;
  std::string password;
  std::string virtual_host;
};

class Connection {
 public:
  Connection(std::shared_ptr<Config> cfg, const std::string &queue);
  ~Connection();
  void publish(const nlohmann::json &payload);
  void consume(std::shared_ptr<peony::queue::Consumer> consumer);

 private:
  void ensure_queue();
  void die_on_error(int code);
  void die_on_amqp_error(amqp_rpc_reply_t reply);
  std::string_view bytes2str(amqp_bytes_t buf);

  const int channel_id;
  const amqp_bytes_t routing_key;
  const amqp_bytes_t exchange;
  const amqp_bytes_t queue;
  amqp_connection_state_t connection;
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