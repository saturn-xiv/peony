#pragma once

#include <chrono>
#include <cstdint>
#include <exception>
#include <filesystem>
#include <iostream>
#include <memory>
#include <mutex>
#include <optional>
#include <sstream>
#include <string>
#include <thread>
#include <utility>

#include <soci/soci.h>
#include <boost/log/trivial.hpp>

#ifndef PEONY_DEFAULT_POOL_SIZE
#define PEONY_DEFAULT_POOL_SIZE 32
#endif

namespace peony {
namespace orm {
class Logger : public soci::logger_impl {
 public:
  void start_query(std::string const& query) override;

 private:
  Logger* do_clone() const override;
};
}  // namespace orm
}  // namespace peony
