#include "queue.h"

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