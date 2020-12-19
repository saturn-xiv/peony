#include "application.h"
#include "cache.h"
#include "crypt.h"
#include "orm.h"
#include "queue.h"
#include "tty.h"
#include "utils.h"

void run_recipe(const std::string& name) {
  // TODO
}
void run_http(const toml::table& root) {
  // TODO
}
void run_rpc(const toml::table& root) {
  // TODO
}
void run_consumer(const toml::table& root) {
  // TODO
}

void peony::Application::run(int argc, char** argv) {
  boost::program_options::options_description generic("Generic options");
  generic.add_options()("version,v", "print version")(
      "help,h", "display argument help information");

  boost::program_options::options_description global("Global options");
  global.add_options()(
      "config,c",
      boost::program_options::value<std::string>()->default_value(
          "config.toml"),
      "configuration file(toml)")(
      "debug,d", boost::program_options::bool_switch(), "debug mode");

  boost::program_options::options_description db("PostgreSQL options");
  db.add_options()("db-generate", boost::program_options::value<std::string>(),
                   "generate a latest migration by name")(
      "db-migrate", boost::program_options::bool_switch(),
      "migrate database to latest migration")(
      "db-rollback", boost::program_options::bool_switch(),
      "rollback database the last migration")(
      "db-status", boost::program_options::bool_switch(),
      "show database schema status");

  boost::program_options::options_description cache("Redis options");
  cache.add_options()("cache-clear", boost::program_options::bool_switch(),
                      "remove all cache items")(
      "cache-list", boost::program_options::bool_switch(),
      "list all cache items");

  boost::program_options::options_description ops("Ops options");
  ops.add_options()("recipe,r", boost::program_options::value<std::string>(),
                    "recipe name(toml)");

  boost::program_options::options_description run_mode("Run mode options");
  run_mode.add_options()("http", boost::program_options::bool_switch(),
                         "start http server")(
      "rpc", boost::program_options::bool_switch(), "start grpc server")(
      "consumer", boost::program_options::bool_switch(),
      "consume rabbitmq tasks");

  boost::program_options::options_description desc("Allowed options");
  desc.add(generic).add(global).add(db).add(cache).add(ops).add(run_mode);
  boost::program_options::variables_map vm;
  boost::program_options::store(
      boost::program_options::parse_command_line(argc, argv, desc), vm);
  boost::program_options::notify(vm);

  const bool debug = vm["debug"].as<bool>();
  if (vm.count("help")) {
    std::cout << desc << std::endl;
    return;
  }
  if (vm.count("version")) {
    std::cout << PEONY_GIT_VERSION << std::endl;
    return;
  }

  peony::utils::init_logging(false, debug);
  if (vm.count("recipe")) {
    const auto name = vm["recipe"].as<std::string>();
    run_recipe(name);
    return;
  }

  BOOST_LOG_TRIVIAL(info) << PEONY_PROJECT_NAME << "(" << PEONY_GIT_VERSION
                          << ")";
  BOOST_LOG_TRIVIAL(debug) << "run in debug mode";
  const std::string config = vm["config"].as<std::string>();
  BOOST_LOG_TRIVIAL(info) << "load from " << config;
  toml::table root = toml::parse_file(config);

  if (vm["cache-list"].as<bool>()) {
    // TODO
    return;
  }

  if (vm["cache-clear"].as<bool>()) {
    // TODO
    return;
  }

  auto pg_cnf = std::make_shared<peony::postgresql::Config>(
      *(root["postgresql"].as_table()));
  if (vm.count("db-gnerate")) {
    const auto name = vm["db-generate"].as<std::string>();
    auto dao = peony::postgresql::SchemaDao(pg_cnf);
    dao.generate(name);
    return;
  }
  if (vm["db-migrate"].as<bool>()) {
    auto dao = peony::postgresql::SchemaDao(pg_cnf);
    dao.load();
    dao.migrate();
    return;
  }
  if (vm["db-rollback"].as<bool>()) {
    auto dao = peony::postgresql::SchemaDao(pg_cnf);
    dao.load();
    dao.rollback();
    return;
  }
  if (vm["db-status"].as<bool>()) {
    auto dao = peony::postgresql::SchemaDao(pg_cnf);
    dao.load();
    std::cout << dao << std::endl;
    return;
  }

  auto pg_pool = std::make_shared<peony::postgresql::Pool>(pg_cnf);

  if (vm["http"].as<bool>()) {
    run_http(root);
    return;
  }
  if (vm["rpc"].as<bool>()) {
    run_rpc(root);
    return;
  }
  if (vm["consumer"].as<bool>()) {
    run_consumer(root);
    return;
  }
  BOOST_LOG_TRIVIAL(warning) << "exit...";
  return;
}