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

#include <soci/postgresql/soci-postgresql.h>
#include <soci/sqlite3/soci-sqlite3.h>
#include <boost/log/trivial.hpp>

#ifndef PEONY_DEFAULT_POOL_SIZE
#define PEONY_DEFAULT_POOL_SIZE 32
#endif

namespace peony {
namespace orm {}
}  // namespace peony
