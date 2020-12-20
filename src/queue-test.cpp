#define BOOST_TEST_MODULE Queue Test

#include <thread>

#include <boost/test/included/unit_test.hpp>

#include "queue.h"
#include "utils.h"

class EchoTask {
 public:
  EchoTask(int value = 0) : value(value) {}
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(EchoTask, value)
 private:
  int value;
};

BOOST_AUTO_TEST_CASE(rabbitmq_test) {
  peony::utils::init_logging(false, true);
  toml::table root = toml::parse_file("config.toml");

  auto cfg =
      std::make_shared<peony::rabbitmq::Config>(*(root["rabbitmq"].as_table()));

  const std::string queue = "echo";

  {
    std::cout << "run productor" << std::endl;
    auto con = peony::rabbitmq::Connection(cfg, queue);
    for (int i = 0; i < 10; i++) {
      EchoTask task(i);
      nlohmann::json payload;
      nlohmann::to_json(payload, task);
      con.publish(payload);
      std::this_thread::sleep_for(std::chrono::seconds(1));
    }
  }
  {
    std::cout << "run consumer" << std::endl;
    auto consumer = std::make_shared<peony::queue::Echo>();
    auto con = peony::rabbitmq::Connection(cfg, queue);
    con.consume(consumer);
  }
}