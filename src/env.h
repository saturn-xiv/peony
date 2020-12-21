#ifndef PEONY_ENV_H_
#define PEONY_ENV_H_

#define PEONY_STR2TS(x) boost::posix_time::time_from_string(x.as<std::string>())
#define PEONY_APPLICATION_JSON_UTF8 "application/json; charset=UTF-8"
#define PEONY_TEXT_HTML_UTF8 "text/html; charset=UTF-8"
#define PEONY_TIMESTAMP_SIZE 14

#ifndef PEONY_DEFAULT_POOL_SIZE
#define PEONY_DEFAULT_POOL_SIZE 32
#endif

#ifndef PEONY_DEFAULT_HOST
#define PEONY_DEFAULT_HOST "127.0.0.1"
#endif

#include "config.h"

#include <filesystem>
#include <optional>

#include <nlohmann/json.hpp>

#define TOML_EXCEPTIONS 1
#include <toml.hpp>
#define CPPHTTPLIB_OPENSSL_SUPPORT
#include <httplib.h>

namespace nlohmann {

template <typename T>
struct adl_serializer<std::optional<T>> {
  static void to_json(nlohmann::json& j, const std::optional<T>& opt) {
    if (opt == std::nullopt) {
      j = nullptr;
    } else {
      j = *opt;
    }
  }

  static void from_json(const nlohmann::json& j, std::optional<T>& opt) {
    if (j.is_null()) {
      opt = std::nullopt;
    } else {
      opt = j.get<T>();
    }
  }
};

template <>
struct adl_serializer<std::filesystem::path> {
  static void to_json(nlohmann::json& j, const std::filesystem::path& opt) {
    j = opt.string();
  }

  static void from_json(const nlohmann::json& j, std::filesystem::path& opt) {
    opt = j.get<std::string>();
  }
};

}  // namespace nlohmann

#endif