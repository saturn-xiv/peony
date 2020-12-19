#include "aws.h"

peony::aws::S3::S3(const toml::table &root) {
  this->region = root["region"].value<std::string>().value();
  this->endpoint = root["endpoint"].value<std::string>().value();
  this->access_key = root["access-key"].value<std::string>().value();
  this->secret_key = root["secret-key"].value<std::string>().value();
}

peony::aws::S3::operator toml::table() const {
  toml::table root;
  root.insert("region", this->region);
  root.insert("endpoint", this->endpoint);
  root.insert("access-key", this->access_key);
  root.insert("secret-key", this->secret_key);

  return root;
};