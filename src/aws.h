#ifndef PEONY_AWS_H_
#define PEONY_AWS_H_

#include <string>

#include "env.h"

namespace peony {
namespace aws {

class S3 {
 public:
  S3() {}
  S3(const toml::table &root);

  operator toml::table() const;

 private:
  std::string region;
  std::string endpoint;
  std::string access_key;
  std::string secret_key;
};

}  // namespace aws

}  // namespace peony
#endif
