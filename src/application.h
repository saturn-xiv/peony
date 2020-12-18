#ifndef PEONY_APPLICATION_H_
#define PEONY_APPLICATION_H_

#include "common.h"

namespace peony {

class Application {
 public:
  void run(int argc, char** argv);

 private:
  enum PostgreSqlAction { MIGRATE, ROLLBACK, STATUS };
  enum RedisAction { CLEAR, LIST };
  void http(const toml::table& root);
  void rpc(const toml::table& root);
  void consumer(const toml::table& root);
  void postgresql(const toml::table& root, const PostgreSqlAction action);
  void redis(const toml::table& root, const RedisAction action);
  void recipe(const std::string& name);
};

}  // namespace peony
#endif
