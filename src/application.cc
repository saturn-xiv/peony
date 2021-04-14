#include "peony/application.h"
#include "peony/crypt.h"
#include "peony/postgresql.h"
#include "peony/sqlite3.h"
#include "peony/theme.h"
#include "peony/utils.h"

#include <sodium.h>
#include <boost/log/core.hpp>
#include <boost/log/expressions.hpp>
#include <boost/log/trivial.hpp>
#include <boost/program_options.hpp>
#include <boost/property_tree/ini_parser.hpp>
// #include <boost/property_tree/json_parser.hpp>
// #include <boost/property_tree/xml_parser.hpp>

void peony::Application::launch(int argc, char** argv) {
  std::string config;
  boost::program_options::options_description desc("Allowed options");
  desc.add_options()("help,h", "Prints help information")(
      "version,v", "Print version info and exit")("debug,d",
                                                  "Run on debug mode")(
      "config,c",
      boost::program_options::value<std::string>(&config)->default_value(
          "config.ini"),
      "Config file");

  boost::program_options::variables_map vm;
  boost::program_options::store(
      boost::program_options::parse_command_line(argc, argv, desc), vm);
  boost::program_options::notify(vm);

  if (vm.count("help")) {
    std::cout << desc << std::endl;
    return;
  }
  if (vm.count("version")) {
    std::cout << version() << std::endl;
    return;
  }

  if (sodium_init() == -1) {
    throw std::invalid_argument("sodium init");
  }
  soci::register_factory_sqlite3();
  soci::register_factory_postgresql();

  boost::log::core::get()->set_filter(boost::log::trivial::severity >=
                                      (vm.count("debug")
                                           ? boost::log::trivial::debug
                                           : boost::log::trivial::info));

  BOOST_LOG_TRIVIAL(debug) << "run on debug level";
  BOOST_LOG_TRIVIAL(info) << "load config from " << config;
  boost::property_tree::ptree tree;
  boost::property_tree::read_ini(config, tree);
  this->launch(tree);
  BOOST_LOG_TRIVIAL(warning) << "exit...";
}
