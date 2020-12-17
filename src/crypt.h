#ifndef PEONY_CRYPT_H_
#define PEONY_CRYPT_H_

#include <chrono>
#include <iomanip>
#include <iostream>
#include <sstream>
#include <stdexcept>
#include <vector>
#include <random>
#include <climits>
#include <algorithm>
#include <functional>

#include <boost/beast/core/detail/base64.hpp>
#include <boost/log/trivial.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>

#define PEONT_TIMESTAMP_SIZE 14

namespace peony
{
    namespace crypt
    {
        std::string uuid();
        std::string timestamp();
        std::string random_base64_string(const size_t s);
    } // namespace crypt
} // namespace peony

#endif
