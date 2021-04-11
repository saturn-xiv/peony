#pragma once

#include <chrono>
#include <optional>
#include <string>

namespace peony {
class Jwt {
 public:
  Jwt(const std::string& secret, const std::string& issuer);
  ~Jwt();
  std::string sum(const std::string& key, const std::string& val,
                  const std::chrono::seconds& ttl) const;
  std::optional<std::string> verify(const std::string& token,
                                    const std::string& key) const;

 private:
  const std::string _secret;
  const std::string _issuer;
};
}  // namespace peony
