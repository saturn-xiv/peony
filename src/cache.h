#ifndef PEONY_CACHE_H_
#define PEONY_CACHE_H_

#include "common.h"
#include "pool.h"

#include <hiredis/hiredis.h>

namespace peony {
namespace redis {

class Connection : public peony::pool::Connection {
 public:
  ~Connection() override;
  void clear();
  friend class Factory;

 private:
  redisContext *context;
  std::string prefix;
};

class Factory : public peony::pool::Factory {
 public:
  Factory(const std::string host, const unsigned short int port,
          const unsigned short int db, const std::string prefix);
  std::shared_ptr<peony::pool::Connection> create() override;
  std::string name() const override;

 private:
  const std::string host;
  const unsigned short int port;
  const unsigned short int db;
  const std::string prefix;
};

class Config {
 public:
  Config(const toml::table &root);
  std::shared_ptr<peony::pool::Pool<Connection>> open() const;

  operator toml::table() const;

 private:
  std::string host;
  unsigned short port;
  std::optional<std::string> user;
  std::optional<std::string> password;
  std::string prefix;
  unsigned short db;
  size_t pool_size;
};

}  // namespace redis
}  // namespace peony
#endif