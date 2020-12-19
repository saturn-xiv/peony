#include "aws.h"

peony::aws::S3::operator toml::table() const {
  toml::table root;
  root.insert("region", this->region);
  root.insert("endpoint", this->endpoint);
  root.insert("access-key", this->access_key);
  root.insert("secret-key", this->secret_key);

  return root;
};