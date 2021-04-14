#pragma once

#include <string>

#include <boost/property_tree/ptree.hpp>

namespace peony {
class Application {
 public:
  Application() {}
  ~Application() {}
  void launch(int argc, char** argv);

 protected:
  virtual void launch(const boost::property_tree::ptree& tree) = 0;
  virtual std::string version() const = 0;

 private:
  std::string config;
};
}  // namespace peony
