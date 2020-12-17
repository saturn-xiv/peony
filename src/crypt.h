#ifndef PEONY_CRYPT_H_
#define PEONY_CRYPT_H_

#include "common.h"

namespace peony {
namespace crypt {
std::string uuid();
std::string timestamp();
std::string random_base64_string(const size_t s);
}  // namespace crypt
}  // namespace peony

#endif
