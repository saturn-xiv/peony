#ifndef PEONY_CRYPT_H_
#define PEONY_CRYPT_H_

#include "common.h"

namespace peony {
namespace crypt {

std::string uuid();
std::string timestamp();

std::string random_str(std::string::size_type s);
std::string random_base64(const size_t s);
unsigned int random_uint(unsigned int min, unsigned int max);
double random_double(unsigned int min, unsigned int max);

}  // namespace crypt
}  // namespace peony

#endif
