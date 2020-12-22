#ifndef PEONY_DEPLOY_H_
#define PEONY_DEPLOY_H_

#include <string>
#include <vector>

#include "env.h"

#include "deploy.grpc.pb.h"

namespace peony {
namespace deploy {
void execute(const std::map<std::string, nlohmann::json>& host);
}  // namespace deploy
}  // namespace peony

#endif
