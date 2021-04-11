#include "peony/jwt.h"

#include <jwt-cpp/jwt.h>

#include <iostream>

peony::Jwt::Jwt(const std::string& secret, const std::string& issuer)
    : _secret(secret), _issuer(issuer) {}
peony::Jwt::~Jwt() {}
std::string peony::Jwt::sum(const std::string& key, const std::string& val,
                            const std::chrono::seconds& ttl) const {
  return jwt::create()
      .set_type("JWT")
      .set_issuer(_issuer)
      .set_issued_at(std::chrono::system_clock::now())
      .set_expires_at(std::chrono::system_clock::now() + ttl)
      .set_payload_claim(key, jwt::claim(val))
      .sign(jwt::algorithm::hs512{_secret});
}
std::optional<std::string> peony::Jwt::verify(const std::string& token,
                                              const std::string& key) const {
  auto verifier = jwt::verify()
                      .allow_algorithm(jwt::algorithm::hs512{_secret})
                      .with_issuer(_issuer);
  auto decoded = jwt::decode(token);

  verifier.verify(decoded);
  for (const auto& it : decoded.get_payload_claims()) {
    if (it.first == key) {
      return it.second.as_string();
    }
  }

  return std::nullopt;
}
