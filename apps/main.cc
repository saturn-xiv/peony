#include <iostream>

#include "../version.h"
#include "peony/application.h"

int main() {
  std::cout << PEONY_GIT_VERSION << std::endl;
  return 0;
}
