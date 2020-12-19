#ifndef PEONY_CACHE_H_
#define PEONY_CACHE_H_

#include <deque>
#include <mutex>
#include <sstream>
#include <string>

#include "env.h"

#include <hiredis/hiredis.h>
#include <boost/log/trivial.hpp>

namespace peony {

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

class Redis {
 public:
  Redis(std::string host = PEONY_DEFAULT_HOST, unsigned int port = 6379,
        std::optional<std::string> user = std::nullopt,
        std::optional<std::string> password = std::nullopt,
        std::optional<std::string> prefix = std::nullopt, unsigned short db = 0,
        size_t pool_size = PEONY_DEFAULT_POOL_SIZE)
      : host(host),
        port(port),
        user(user),
        password(password),
        prefix(prefix),
        db(db),
        pool_size(pool_size) {}
  Redis(const toml::table &root);
  friend std::ostream &operator<<(std::ostream &out, const Redis &self) {
    out << "tcp://";
    if (self.user) {
      out << self.user.value() << '@';
    }
    out << self.host << ':' << self.port << '/' << self.db;
    return out;
  }

  NLOHMANN_DEFINE_TYPE_INTRUSIVE(Redis, host, port, user, password, prefix, db,
                                 pool_size)

  operator toml::table() const;

 private:
  std::string host;
  unsigned short port;
  std::optional<std::string> user;
  std::optional<std::string> password;
  std::optional<std::string> prefix;
  unsigned short db;
  size_t pool_size;
};

}  // namespace peony
#endif