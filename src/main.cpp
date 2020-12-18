#include "application.h"

int main(int argc, char** argv) {
  peony::Application app;
  try {
    app.run(argc, argv);
    return EXIT_SUCCESS;
  } catch (const std::exception& e) {
    std::cout << e.what() << std::endl;
  }
  return EXIT_FAILURE;
}