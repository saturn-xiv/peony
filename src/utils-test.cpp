#define BOOST_TEST_MODULE Utils Test

#include <boost/test/included/unit_test.hpp>

#include "utils.h"

BOOST_AUTO_TEST_CASE(timezone_test) {
  for (const auto& it : peony::utils::timezone_regions()) {
    std::cout << it << std::endl;
  }
  {
    for (const auto& it :
         {"America/New_York", "America/Los_Angeles", "Asia/Shanghai"}) {
      std::cout << it << "\t\t" << peony::utils::str2tz(it)->to_posix_string()
                << std::endl;
    }
  }
}