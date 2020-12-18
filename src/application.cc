#include "application.h"
#include "orm.h"
#include "utils.h"

#define CONFIG_POSTGRESQL_KEY "postgresql"
#define CONFIG_REDIS_KEY "redis"

void peony::Application::recipe(const std::string& name) {
  // TODO
}
void peony::Application::http(const toml::table& root) {
  // TODO
}
void peony::Application::rpc(const toml::table& root) {
  // TODO
}
void peony::Application::consumer(const toml::table& root) {
  // TODO
}
void peony::Application::postgresql(const toml::table& root,
                                    const PostgreSqlAction action) {
  auto node = root[CONFIG_POSTGRESQL_KEY];
  if (!node.is_table()) {
    throw std::invalid_argument("invalid postgresql config");
  }
  {
    // peony::postgresql::Config cfg(*(node.as_table()));
    // auto pg = cfg.open();
    // auto db = pg->get();
  }
  // const auto cfg = peony::Config(root);
  // {
  //   auto pg = cfg.postgresql.open(std::nullopt);
  //   auto db = pg->get();
  //   peony::postgresql::SchemaDao dao(db->context, std_fs::path(db_schema));
  //   dao.load();

  // }

  // auto pg = cfg.postgresql.open(std_fs::path(db_schema) / "prepares.toml");

  // {
  //   auto db = pg->get();
  //   auto node = root["crawler"];
  //   if (!node.is_table()) {
  //     BOOST_LOG_TRIVIAL(error) << "con't find crawler config";
  //     return EXIT_FAILURE;
  //   }
  //   peony::crawler::Crawler crawler(db->context, *(node.as_table()));

  //   if (vm["crawler-all"].as<bool>()) {
  //     crawler.execute();
  //     return;
  //   }
  //   if (vm.count("crawler-name")) {
  //     const auto name = vm["crawler-name"].as<std::string>();
  //     crawler.execute(name);
  //     return;
  //   }
  // }

  // auto redis = cfg.redis.open();
  // { auto it = redis.get(); }

  // auto router = peony::api::Router(cfg.http);
  // {
  //   peony::nut::Plugin nut;
  //   nut.mount(router);
  // }
  // router.start();
  // TODO
}
void peony::Application::redis(const toml::table& root,
                               const RedisAction action) {
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
  db.add_options()("db-migrate", boost::program_options::bool_switch(),
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
    this->recipe(name);
    return;
  }

  BOOST_LOG_TRIVIAL(info) << PEONY_PROJECT_NAME << "(" << PEONY_GIT_VERSION
                          << ")";
  BOOST_LOG_TRIVIAL(debug) << "run in debug mode";
  const std::string config = vm["config"].as<std::string>();
  BOOST_LOG_TRIVIAL(info) << "load from " << config;
  toml::table root = toml::parse_file(config);

  if (vm["cache-list"].as<bool>()) {
    this->redis(root, RedisAction::LIST);
    return;
  }

  if (vm["cache-clear"].as<bool>()) {
    this->redis(root, RedisAction::CLEAR);
    return;
  }

  if (vm["db-migrate"].as<bool>()) {
    this->postgresql(root, PostgreSqlAction::MIGRATE);
    return;
  }
  if (vm["db-rollback"].as<bool>()) {
    this->postgresql(root, PostgreSqlAction::ROLLBACK);
    return;
  }
  if (vm["db-status"].as<bool>()) {
    this->postgresql(root, PostgreSqlAction::STATUS);
    return;
  }
  if (vm["http"].as<bool>()) {
    this->http(root);
    return;
  }
  if (vm["rpc"].as<bool>()) {
    this->rpc(root);
    return;
  }
  if (vm["consumer"].as<bool>()) {
    this->consumer(root);
    return;
  }
  BOOST_LOG_TRIVIAL(warning) << "exit...";
  return;
}