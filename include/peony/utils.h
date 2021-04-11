#pragma once

#include <csignal>
#include <cstdint>
#include <deque>
#include <exception>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <mutex>
#include <optional>
#include <ostream>
#include <set>
#include <streambuf>
#include <string>
#include <string_view>
#include <tuple>
#include <unordered_map>
#include <utility>
#include <vector>

#include <boost/date_time/local_time/local_time.hpp>
#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/log/trivial.hpp>

#define TOML_EXCEPTIONS 1
#include <toml.hpp>

#include <httplib.h>
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

}  // namespace nlohmann

#ifndef PEONY_DATE_TIME_ZONESPEC_CSV
#define PEONY_DATE_TIME_ZONESPEC_CSV "date_time_zonespec.csv"
#endif

#ifndef PEONY_LOCALHOST
#define PEONY_LOCALHOST "127.0.0.1"
#endif

namespace peony {
namespace utils {
std::string ping(const std::string &host, const uint8_t count = 4,
                 const uint8_t timeout = 1);
std::string nslookup(const std::string &host,
                     const std::string &server = "8.8.8.8");
std::string mac(const std::string &name);
void watchdog();
std::string hostname();
std::string file2str(const std::filesystem::path &file);

std::string uptime();

boost::posix_time::ptime str2time(const std::string &time,
                                  const std::string &format);
boost::local_time::local_date_time str2time(const std::string &time,
                                            const std::string &format,
                                            const std::string &timezone);
std::vector<std::string> timezone_regions();
boost::local_time::time_zone_ptr str2tz(const std::string &timezone);

unsigned long str2ul(const std::string &s);

}  // namespace utils

}  // namespace peony
