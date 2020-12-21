#ifndef PEONY_JWT_H_
#define PEONY_JWT_H_

#include "env.h"

namespace peony {
namespace jwt {
class Config {
 public:
  Config(const std::string &key) : key(key) {}
  std::string encode(const nlohmann::json &object) const;
  nlohmann::json decode(const std::string &token) const;

 private:
  const std::string key;
};
}  // namespace jwt
}  // namespace peony

#endif
