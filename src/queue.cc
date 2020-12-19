#include "queue.h"

peony::RabbitMQ::RabbitMQ(const toml::table &root) {
  this->host = root["host"].value<std::string>().value_or(PEONY_DEFAULT_HOST);
  this->port = root["port"].value<unsigned int>().value_or(5672);
  this->user = root["user"].value<std::string>().value();
  this->password = root["password"].value<std::string>().value();
  this->virtual_host = root["virtual-host"].value<std::string>().value();
  this->pool_size =
      root["pool-size"].value<long>().value_or(PEONY_DEFAULT_POOL_SIZE);
}

peony::RabbitMQ::operator toml::table() const {
  toml::table root;
  root.insert("host", this->host);
  root.insert("port", this->port);
  root.insert("user", this->user);
  root.insert("password", this->password);
  root.insert("virtual-host", this->virtual_host);
  root.insert("pool-size", (long)this->pool_size);
  return root;
};