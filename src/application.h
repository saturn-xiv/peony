#ifndef PEONY_APPLICATION_H_
#define PEONY_APPLICATION_H_

namespace peony {
class Application {
 public:
  void run(int argc, char** argv);
};

}  // namespace peony

namespace edelweiss {
class Application {
 public:
  void run(int argc, char** argv);
};
}  // namespace edelweiss
#endif
