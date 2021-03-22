#ifndef PEONY_ENV_H_
#define PEONY_ENV_H_

#include <pthread.h>
#include <csignal>
#include <cstdint>
#include <deque>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <mutex>
#include <optional>
#include <ostream>
#include <set>
#include <streambuf>
#include <string>
#include <string_view>
#include <tuple>
#include <unordered_map>
#include <utility>
#include <vector>

#include <boost/algorithm/string.hpp>
#include <boost/asio.hpp>
#include <boost/asio/ip/host_name.hpp>
#include <boost/asio/serial_port.hpp>
#include <boost/bind/bind.hpp>
#include <boost/date_time/local_time/local_time.hpp>
#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/foreach.hpp>
#include <boost/format.hpp>
#include <boost/log/core.hpp>
#include <boost/log/expressions.hpp>
#include <boost/log/trivial.hpp>
#include <boost/program_options.hpp>
#include <boost/property_tree/ini_parser.hpp>
#include <boost/property_tree/json_parser.hpp>
#include <boost/property_tree/ptree.hpp>
#include <boost/property_tree/xml_parser.hpp>
#include <boost/thread.hpp>

#include <amqp.h>
#include <amqp_framing.h>
#include <amqp_tcp_socket.h>
#include <hiredis/hiredis.h>
#include <sqlite3.h>
#include <inja/inja.hpp>
#include <jwt/jwt.hpp>
#include <nlohmann/json.hpp>
#include <pqxx/pqxx>
#include <zmq.hpp>

// #include <mailio/imap.hpp>
// #include <mailio/message.hpp>
// #include <mailio/mime.hpp>
// #include <mailio/pop3.hpp>
// #include <mailio/smtp.hpp>

#define TOML_EXCEPTIONS 1
#include <toml.hpp>
#define CPPHTTPLIB_OPENSSL_SUPPORT
#include <httplib.h>

namespace nlohmann {

template <typename T>
struct adl_serializer<std::optional<T>> {
  static void to_json(nlohmann::json &j, const std::optional<T> &opt) {
    if (opt == std::nullopt) {
      j = nullptr;
    } else {
      j = *opt;
    }
  }

  static void from_json(const nlohmann::json &j, std::optional<T> &opt) {
    if (j.is_null()) {
      opt = std::nullopt;
    } else {
      opt = j.get<T>();
    }
  }
};

template <>
struct adl_serializer<std::filesystem::path> {
  static void to_json(nlohmann::json &j, const std::filesystem::path &opt) {
    j = opt.string();
  }

  static void from_json(const nlohmann::json &j, std::filesystem::path &opt) {
    opt = j.get<std::string>();
  }
};

}  // namespace nlohmann

#define PEONY_STR2TS(x) boost::posix_time::time_from_string(x.as<std::string>())
#define PEONY_APPLICATION_JSON_UTF8 "application/json; charset=UTF-8"
#define PEONY_TEXT_HTML_UTF8 "text/html; charset=UTF-8"
#define PEONY_TIMESTAMP_SIZE 14

#ifndef PEONY_DEFAULT_POOL_SIZE
#define PEONY_DEFAULT_POOL_SIZE 32
#endif

#ifndef PEONY_SERIAL_PORT_READ_BUFFER_MAX_SIZE
#define PEONY_SERIAL_PORT_READ_BUFFER_MAX_SIZE 1 << 12
#endif

#ifndef PEONY_DEFAULT_HOST
#define PEONY_DEFAULT_HOST "127.0.0.1"
#endif

namespace peony {

class Application {
 public:
  Application() {}
  ~Application() {}
  void launch(int argc, char **argv);

 protected:
  virtual void launch(boost::property_tree::ptree tree) = 0;

 private:
  std::string config;
};

namespace serial_port {
class Tty {
 public:
  Tty(const std::string &name = "ttyUSB0", const unsigned int baud_rate = 9600,
      const unsigned int character_size = 8,
      const boost::asio::serial_port_base::stop_bits::type stop_bits =
          boost::asio::serial_port_base::stop_bits::one,
      const boost::asio::serial_port_base::parity::type parity =
          boost::asio::serial_port_base::parity::none,
      const boost::asio::serial_port_base::flow_control::type flow_control =
          boost::asio::serial_port_base::flow_control::none) {
    open(name, baud_rate, character_size, stop_bits, parity, flow_control);
  }

  Tty(boost::property_tree::ptree tree) {
    const std::string name = tree.get("tty.name", "ttyUSB0");
    open(name, 9600, 8, boost::asio::serial_port_base::stop_bits::one,
         boost::asio::serial_port_base::parity::none,
         boost::asio::serial_port_base::flow_control::none);
  }
  ~Tty();

  void read();
  void listen();
  void write(const std::string &buf);
  std::vector<std::string> get_port_names() const;

 protected:
  void on_receive(const char *buffer, const boost::system::error_code &ec,
                  const size_t bytes_transferred);
  std::optional<std::pair<size_t, size_t>> match(const std::string &payload,
                                                 const std::string &begin,
                                                 const std::string &end);

  virtual std::optional<std::pair<size_t, size_t>> match(
      const std::string &payload) = 0;
  virtual void execute(const std::string &line) = 0;

 private:
  void open(
      const std::string &name, const unsigned int baud_rate,
      const unsigned int character_size,
      const boost::asio::serial_port_base::stop_bits::type stop_bits,
      const boost::asio::serial_port_base::parity::type parity,
      const boost::asio::serial_port_base::flow_control::type flow_control);

  std::string name;

  std::shared_ptr<boost::asio::serial_port> port;
  boost::asio::io_service io_service;
  char line[1];
  std::string payload;
  std::mutex lock;
};
}  // namespace serial_port

class Crypt {};
class Jwt {};
class Redis {};
class ZeroMQ {};
class Sqlite {};
class PostgreSql {};
class RabbitMQ {};
class Mosquitto {};
class ElasticSearch {};
class Theme {};

namespace utils {
std::string mac(const std::string &name);
void watchdog();

}  // namespace utils
}  // namespace peony

#endif
