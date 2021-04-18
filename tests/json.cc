#define BOOST_TEST_MODULE Json Test

#include <boost/test/included/unit_test.hpp>

#include "peony/theme.h"

class ModelC {
 public:
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(ModelC, bpt, i, st, ss)
 private:
  boost::posix_time::ptime bpt;
  int i;
  size_t st;
  std::string ss;
};

struct ModelS {
  boost::posix_time::ptime bpt;
  int i;
  size_t st;
  std::string ss;
};
NLOHMANN_DEFINE_TYPE_NON_INTRUSIVE(ModelS, bpt, i, st, ss)

BOOST_AUTO_TEST_CASE(dump_parse_test) {
  {
    ModelS it1{.bpt = boost::posix_time::microsec_clock::local_time()};
    nlohmann::json js1 = it1;
    const std::string jss = js1.dump();
    std::cout << jss << std::endl;
    nlohmann::json js2 = nlohmann::json::parse(jss);

    ModelS it2 = js2.get<ModelS>();
    std::cout << it2.bpt << " " << it2.i << " " << it2.st << std::endl;
  }
}
