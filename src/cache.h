#ifndef PEONY_CACHE_H_
#define PEONY_CACHE_H_

#include <deque>
#include <mutex>
#include <set>
#include <sstream>
#include <string>
#include <utility>
#include <vector>

#include "env.h"

#include <hiredis/hiredis.h>
#include <boost/log/trivial.hpp>

namespace peony {

namespace cache {

void set(redisContext *ctx, const std::string &key, const nlohmann::json &val,
         size_t ttl = 60 * 60 * 24);
std::optional<nlohmann::json> get(redisContext *ctx, const std::string &key);
void clear(redisContext *ctx);
std::vector<std::string> all(redisContext *ctx);

}  // namespace cache

namespace redis {

class Config {
 public:
  Config(std::string host = PEONY_DEFAULT_HOST, unsigned int port = 6379,
         std::optional<std::string> user = std::nullopt,
         std::optional<std::string> password = std::nullopt,
         unsigned short db = 0, size_t pool_size = PEONY_DEFAULT_POOL_SIZE)
      : host(host),
        port(port),
        user(user),
        password(password),
        db(db),
        pool_size(pool_size) {}
  Config(const toml::table &root);
  redisContext *open();

  friend class Pool;
  friend std::ostream &operator<<(std::ostream &out, const Config &self) {
    out << "tcp://";
    if (self.user) {
      out << self.user.value() << '@';
    }
    out << self.host << ':' << self.port << '/' << self.db;
    return out;
  }

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Config, host, port, user, password, db,
                                 pool_size)

  operator toml::table() const;

 private:
  std::string host;
  unsigned short port;
  std::optional<std::string> user;
  std::optional<std::string> password;
  unsigned short db;
  size_t pool_size;
};

class Pool {
 public:
  Pool(const std::shared_ptr<Config> config);
  ~Pool();
  redisContext *get();
  void release(redisContext *it);
  std::pair<size_t, size_t> status();

 private:
  redisContext *create();

  const std::shared_ptr<Config> config;
  std::deque<redisContext *> pool;
  std::set<redisContext *> used;
  std::mutex locker;
};

}  // namespace redis

// class Connection : public peony::pool::Connection {
//  public:
//   ~Connection() override;
//   void clear();
//   friend class Factory;

//  private:
//   redisContext *context;
//   std::string prefix;
// };

// class Factory : public peony::pool::Factory {
//  public:
//   Factory(const std::string host, const unsigned short int port,
//           const unsigned short int db, const std::string prefix);
//   std::shared_ptr<peony::pool::Connection> create() override;
//   std::string name() const override;

//  private:
//   const std::string host;
//   const unsigned short int port;
//   const unsigned short int db;
//   const std::string prefix;
// };

}  // namespace peony
#endif