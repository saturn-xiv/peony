#ifndef PEONY_AWS_H_
#define PEONY_AWS_H_

#include "common.h"

namespace peony {
namespace aws {
namespace s3 {
class Config {
 public:
  Config(const toml::table &root);

  operator toml::table() const;

 private:
  std::string region;
  std::string endpoint;
  std::string access_key;
  std::string secret_key;
};
}  // namespace s3
}  // namespace aws

}  // namespace peony
#endif
