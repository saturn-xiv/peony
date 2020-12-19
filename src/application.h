#ifndef PEONY_APPLICATION_H_
#define PEONY_APPLICATION_H_

#include <iostream>
#include <string>

#include "env.h"

#include <boost/program_options.hpp>

namespace peony {

class Application {
 public:
  void run(int argc, char** argv);
};

}  // namespace peony
#endif
