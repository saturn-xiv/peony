#pragma once

#include <filesystem>
#include <string>

namespace peony {
std::string get_stdout_from_command(const std::string& cmd);
std::string uuid();
std::string timestamp();

namespace random {
std::filesystem::path file(const std::string& ext);

std::string string(std::string::size_type s);
std::string base64(const size_t s);
unsigned int integer(unsigned int min, unsigned int max);
double double_(unsigned int min, unsigned int max);

}  // namespace random

class Key {
 public:
  Key(const std::string& secret);
  ~Key();
  std::pair<unsigned char*, unsigned char*> encrypt(const unsigned char* plain,
                                                    size_t len);
  unsigned char* decrypt(const unsigned char* secret,
                         const unsigned char* nonce, const size_t len);
  std::pair<unsigned char*, unsigned char*> password(const char* plain);
  bool verify(const unsigned char* secret, const unsigned char* salt,
              const char* plain);

  static std::string generate_key();

 private:
  unsigned char* _payload;
};

}  // namespace peony
