#ifndef PEONY_ENV_H_
#define PEONY_ENV_H_

#define PEONY_STR2TS(x) boost::posix_time::time_from_string(x.as<std::string>())
#define PEONY_APPLICATION_JSON_UTF8 "application/json; charset=UTF-8"
#define PEONY_TEXT_HTML_UTF8 "text/html; charset=UTF-8"
#define PEONT_TIMESTAMP_SIZE 14

#include "config.h"

#include <optional>

#include <nlohmann/json.hpp>

#define TOML_EXCEPTIONS 1
#include <toml.hpp>

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

}  // namespace nlohmann

#endif