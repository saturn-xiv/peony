#ifndef PEONY_UTILS_H_
#define PEONY_UTILS_H_

#include <vector>

#include "env.h"

#include <boost/date_time/local_time/local_time.hpp>
#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/log/trivial.hpp>

namespace peony {

namespace utils {
void init_logging(bool daemon, bool debug);
void watchdog(int dur);

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

#endif
