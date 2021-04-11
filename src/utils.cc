#include "peony/utils.h"
#include "peony/crypt.h"

#include <sys/utsname.h>

#include <boost/algorithm/string.hpp>
#include <boost/asio.hpp>
#include <boost/asio/ip/host_name.hpp>
#include <boost/date_time/time_facet.hpp>
#include <boost/foreach.hpp>
#include <boost/format.hpp>

std::string peony::utils::ping(const std::string& host, const uint8_t count,
                               const uint8_t timeout) {
  std::stringstream ss;
  ss << "ping -c " << count << " -W " << timeout << " " << host;
  return peony::get_stdout_from_command(ss.str());
}
std::string peony::utils::nslookup(const std::string& host,
                                   const std::string& server) {
  std::stringstream ss;
  ss << "dig @" << server << " " << host;
  return peony::get_stdout_from_command(ss.str());
}

std::string peony::utils::mac(const std::string& name) {
  std::ifstream fd("/sys/class/net/" + name + "/address");
  std::string it((std::istreambuf_iterator<char>(fd)),
                 std::istreambuf_iterator<char>());
  boost::trim(it);
  return it;
}

#define PEONY_PING_WATCHDOG 5
void ping_watchdog(int sig) { alarm(PEONY_PING_WATCHDOG); }
void peony::utils::watchdog() {
  BOOST_LOG_TRIVIAL(info) << "start watch dog";
  signal(SIGALRM, ping_watchdog);
  alarm(PEONY_PING_WATCHDOG);
}

std::string peony::utils::hostname() { return boost::asio::ip::host_name(); }

std::string peony::utils::file2str(const std::filesystem::path& file) {
  std::ifstream tmp(file);
  std::string str((std::istreambuf_iterator<char>(tmp)),
                  std::istreambuf_iterator<char>());
  boost::trim(str);
  return str;
}

boost::posix_time::ptime peony::utils::str2time(const std::string& time,
                                                const std::string& format) {
  const std::locale loc = std::locale(
      std::locale::classic(), new boost::posix_time::time_input_facet(format));
  std::istringstream is(time);
  is.imbue(loc);

  boost::posix_time::ptime it;
  is >> it;
  return it;
}

std::vector<std::string> peony::utils::timezone_regions() {
  boost::local_time::tz_database tzd;
  tzd.load_from_file(PEONY_DATE_TIME_ZONESPEC_CSV);
  return tzd.region_list();
}

boost::local_time::time_zone_ptr peony::utils::str2tz(
    const std::string& timezone) {
  boost::local_time::tz_database tzd;
  tzd.load_from_file(PEONY_DATE_TIME_ZONESPEC_CSV);
  boost::local_time::time_zone_ptr tz = tzd.time_zone_from_region(timezone);
  return tz;
}

boost::local_time::local_date_time peony::utils::str2time(
    const std::string& time, const std::string& format,
    const std::string& timezone) {
  const std::locale loc = std::locale(
      std::locale::classic(), new boost::posix_time::time_input_facet(format));
  std::istringstream is(time);
  is.imbue(loc);

  boost::posix_time::ptime cur;
  is >> cur;

  boost::local_time::time_zone_ptr utc(
      new boost::local_time::posix_time_zone("UTC"));
  boost::local_time::local_date_time dt(cur, utc);

  auto tz = peony::utils::str2tz(timezone);
  return dt.local_time_in(tz);
}

unsigned long peony::utils::str2ul(const std::string& s) {
  unsigned long v;
  std::istringstream ss(s);
  ss >> v;
  return v;
}

std::string peony::utils::uptime() {
  const static auto boot = std::chrono::system_clock::now();
  auto now = std::chrono::system_clock::now();
  // TODO
  return "";
}
