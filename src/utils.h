#ifndef PEONY_UTILS_H_
#define PEONY_UTILS_H_

#include "common.h"

#define PEONY_TIMEZONE_SPEC_FILE "date_time_zonespec.csv"

namespace peony {

namespace utils {
void init_logging(bool daemon, bool debug);
void watchdog(int dur);

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
