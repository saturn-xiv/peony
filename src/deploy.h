#ifndef PEONY_DEPLOY_H_
#define PEONY_DEPLOY_H_

#include <mutex>
#include <set>
#include <string>
#include <thread>
#include <utility>
#include <vector>

#include "env.h"

#include <boost/log/trivial.hpp>

#include "deploy.grpc.pb.h"

namespace peony {
namespace deploy {
void execute(const std::filesystem::path& root, std::stringstream& stream);
void execute(const std::map<std::string, std::string>& env,
             const peony::deploy::models::Task_Download& task);
void execute(const std::map<std::string, std::string>& env,
             const peony::deploy::models::Task_Upload& task);
void execute(const std::map<std::string, std::string>& env,
             const peony::deploy::models::Task_Script& task);
void execute(const std::map<std::string, std::string>& env,
             const peony::deploy::models::Task& task);
void execute(const std::string& recipe, const std::string& inventory);
}  // namespace deploy
}  // namespace peony

#endif
