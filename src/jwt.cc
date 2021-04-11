#include "peony/jwt.h"

#include <iostream>

#include <boost/log/trivial.hpp>
#include <jwt/jwt.hpp>

peony::Jwt::Jwt(const std::string& secret, const std::string& issuer)
    : _secret(secret), _issuer(issuer) {}
peony::Jwt::~Jwt() {}
std::string peony::Jwt::sum(
    const std::unordered_map<std::string, nlohmann::json>& payload,
    const std::chrono::seconds& ttl) const {
  jwt::jwt_object obj{jwt::params::algorithm("HS512"),
                      jwt::params::secret(_secret)};
  obj.add_claim("iss", _issuer)
      .add_claim("iat", std::chrono::system_clock::now())
      .add_claim("exp", std::chrono::system_clock::now() + ttl);

  for (const auto& it : payload) {
    obj.payload().add_claim(it.first, it.second);
  }
  return obj.signature();
}
const nlohmann::json peony::Jwt::verify(const std::string& token) const {
  try {
    const auto obj = jwt::decode(
        token, jwt::params::algorithms({"HS512"}), jwt::params::secret(_secret),
        jwt::params::verify(true), jwt::params::issuer(_issuer));

    return obj.payload().create_json_obj();

  } catch (const jwt::TokenExpiredError& e) {
    BOOST_LOG_TRIVIAL(error) << "token expire: " << e.what();

  } catch (const jwt::SignatureFormatError& e) {
    BOOST_LOG_TRIVIAL(error) << "token signature format error: " << e.what();

  } catch (const jwt::DecodeError& e) {
    BOOST_LOG_TRIVIAL(error) << "token decode: " << e.what();

  } catch (const jwt::VerificationError& e) {
    BOOST_LOG_TRIVIAL(error) << "token verification: " << e.what();
  } catch (...) {
    BOOST_LOG_TRIVIAL(error) << "parse token failed";
  }

  return {};
}
