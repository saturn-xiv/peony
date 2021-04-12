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
