#pragma once

#include <chrono>
#include <optional>
#include <string>
#include <unordered_map>

#include <nlohmann/json.hpp>

namespace peony {
class Jwt {
 public:
  Jwt(const std::string& secret, const std::string& issuer);
  ~Jwt();
  std::string sum(
      const std::unordered_map<std::string, nlohmann::json>& payload,
      const std::chrono::seconds& ttl) const;
  const nlohmann::json parse(const std::string& token) const;

 private:
  const std::string _secret;
  const std::string _issuer;
};
}  // namespace peony
