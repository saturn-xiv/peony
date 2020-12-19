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
  NLOHMANN_DEFINE_TYPE_INTRUSIVE(S3, region, endpoint, access_key, secret_key)

 private:
  std::string region;
  std::string endpoint;
  std::string access_key;
  std::string secret_key;
};

}  // namespace aws

}  // namespace peony
#endif
