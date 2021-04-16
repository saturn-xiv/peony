#pragma once

#include <filesystem>
#include <fstream>
#include <locale>
#include <optional>
#include <streambuf>
#include <string>

#include <boost/date_time/local_time/local_time.hpp>
#include <boost/log/trivial.hpp>
#include <inja/inja.hpp>
#include <nlohmann/json.hpp>

namespace nlohmann {

template <typename T>
struct adl_serializer<std::optional<T>> {
  static void to_json(nlohmann::json &j, const std::optional<T> &opt) {
    if (opt == std::nullopt) {
      j = nullptr;
    } else {
      j = *opt;
    }
  }

  static void from_json(const nlohmann::json &j, std::optional<T> &opt) {
    if (j.is_null()) {
      opt = std::nullopt;
    } else {
      opt = j.get<T>();
    }
  }
};

template <>
struct adl_serializer<std::filesystem::path> {
  static void to_json(nlohmann::json &j, const std::filesystem::path &opt) {
    j = opt.string();
  }

  static void from_json(const nlohmann::json &j, std::filesystem::path &opt) {
    opt = j.get<std::string>();
  }
};

#define PEONY_BOOST_PTIME_ISO8601_FORMAT "%Y-%m-%dT%H:%M:%SZ%q"
#define PEONY_LOCALE_ENGLISH "en_US.UTF-8"

template <>
struct adl_serializer<boost::posix_time::ptime> {
  static void to_json(nlohmann::json &j, const boost::posix_time::ptime &opt) {
    static boost::posix_time::time_facet *facet =
        new boost::posix_time::time_facet();
    facet->format(PEONY_BOOST_PTIME_ISO8601_FORMAT);
    std::stringstream ss;
    // ss.exceptions(std::ios_base::failbit);
    ss.imbue(std::locale(std::locale(PEONY_LOCALE_ENGLISH), facet));
    ss << opt;
    j = ss.str();
  }

  static void from_json(const nlohmann::json &j,
                        boost::posix_time::ptime &opt) {
    static boost::posix_time::time_input_facet *facet =
        new boost::posix_time::time_input_facet();
    facet->format(PEONY_BOOST_PTIME_ISO8601_FORMAT);

    std::stringstream ss;
    // ss.exceptions(std::ios_base::failbit);
    ss.imbue(std::locale(std::locale(PEONY_LOCALE_ENGLISH), facet));
    ss.str(j.get<std::string>());
    ss >> opt;
    ss.clear();
  }
};

template <typename T>
struct adl_serializer<std::shared_ptr<T>> {
  static void to_json(json &j, const std::shared_ptr<T> &opt) {
    if (opt.get()) {
      j = *opt;
    } else {
      j = nullptr;
    }
  }
};

}  // namespace nlohmann

#define PEONY_APPLICATION_JSON_UTF8 "application/json; charset=UTF-8"
#define PEONY_TEXT_HTML_UTF8 "text/html; charset=UTF-8"
#define PEONY_PLAIN_TEXT_UTF8 "text/plain; charset=UTF-8"
