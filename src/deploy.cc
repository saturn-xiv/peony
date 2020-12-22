#include "deploy.h"
#include "crypt.h"

typedef std::map<std::string, std::string> Env;
typedef std::pair<std::vector<Env>, peony::deploy::models::Task> Job;

#define PEONY_DEPLOY_SSH_HOST "ssh.host"
#define PEONY_DEPLOY_SSH_PORT "ssh.port"
#define PEONY_DEPLOY_SSH_USER "ssh.user"
#define PEONY_DEPLOY_SSH_KEY "ssh.key"

void load(const std::string& name,
          peony::deploy::models::Inventory& inventory) {
  BOOST_LOG_TRIVIAL(info) << "load inventory from " << name;
  // TODO
}

void load(const std::string& name, peony::deploy::models::Recipe& recipe) {
  BOOST_LOG_TRIVIAL(info) << "load recipe from " << name;
  // TODO
}
std::filesystem::path root_dir(const Env& env) {
  const auto root =
      std::filesystem::path("tmp") /
      (env.at(PEONY_DEPLOY_SSH_USER) + "@" + env.at(PEONY_DEPLOY_SSH_HOST));
  std::filesystem::create_directories(root);
  return root;
}

void peony::deploy::execute(const std::filesystem::path& root,
                            std::stringstream& stream) {
  const auto tmp = root / (peony::crypt::timestamp() + ".log");
  stream << " >" << tmp.string() << " 2>&1";
  const std::string command = stream.str();
  BOOST_LOG_TRIVIAL(info) << command;

  const auto exit = std::system(command.c_str());
  if (EXIT_SUCCESS != exit) {
    std::stringstream ss;
    ss << "bad shell script return " << exit;
    throw std::runtime_error(ss.str());
  }
  BOOST_LOG_TRIVIAL(info) << "done.";
}
void peony::deploy::execute(const std::map<std::string, std::string>& env,
                            const peony::deploy::models::Task_Download& task) {
  BOOST_LOG_TRIVIAL(info) << "download " << env.at(PEONY_DEPLOY_SSH_USER) << "@"
                          << env.at(PEONY_DEPLOY_SSH_HOST) << task.src()
                          << " to " << task.dest();
  // TODO
}
void peony::deploy::execute(const std::map<std::string, std::string>& env,
                            const peony::deploy::models::Task_Upload& task) {
  // ssh -T -o ConnectTimeout=3 -o ConnectionAttempts=5 -o
  // StrictHostKeyChecking=no -o PasswordAuthentication=no -p {port} -i {key}
  // rsync -rlptD -v src desc
  // rsync -rlptD -zz -e ssh src desc
  /*

  std::stringstream ss;
      // ssh -T -o ConnectTimeout=3 -o ConnectionAttempts=5 -o
  StrictHostKeyChecking=no -o PasswordAuthentication=no -p {port} -i {key}
      // rsync -rlptD -v src desc
      // rsync -rlptD -zz -e ssh src desc
      if (this->is_local())
      {
          ss << "bash " << tmp;
      }
      else
      {
          ss << "ssh -T -o ConnectTimeout=3 -o ConnectionAttempts=5 -o
  StrictHostKeyChecking=no -o PasswordAuthentication=no -p "
             << this->port << " -i " << this->key
             << " " << this->user << "@" << this->host << " 'bash -s'";
      }

  */

  BOOST_LOG_TRIVIAL(info) << "upload " << task.src() << " to "
                          << env.at(PEONY_DEPLOY_SSH_USER) << "@"
                          << env.at(PEONY_DEPLOY_SSH_HOST) << task.dest();
  // TODO
}
void peony::deploy::execute(const std::map<std::string, std::string>& env,
                            const peony::deploy::models::Task_Script& task) {
  BOOST_LOG_TRIVIAL(info) << "run " << task.file() << " on "
                          << env.at(PEONY_DEPLOY_SSH_USER) << "@"
                          << env.at(PEONY_DEPLOY_SSH_HOST);
  // TODO
}
void peony::deploy::execute(const std::map<std::string, std::string>& env,
                            const peony::deploy::models::Task& task) {
  BOOST_LOG_TRIVIAL(info) << "run " << task.name() << " on "
                          << env.at(PEONY_DEPLOY_SSH_USER) << "@"
                          << env.at(PEONY_DEPLOY_SSH_HOST);
  switch (task.payload_case()) {
    case peony::deploy::models::Task::PayloadCase::kScript:
      /* code */
      break;
    case peony::deploy::models::Task::PayloadCase::kUpload:
      break;
    case peony::deploy::models::Task::PayloadCase::kDownload:
      execute(env, task.download());
      break;
    default:
      BOOST_LOG_TRIVIAL(warning) << "unknown job type";
      break;
  }
}

std::set<std::string> load_hosts(
    const peony::deploy::models::Inventory& inventory,
    const peony::deploy::models::Task& task) {
  BOOST_LOG_TRIVIAL(debug) << "load hosts for " << task.name() << "@"
                           << inventory.name();
  std::set<std::string> items;
  for (const auto& tgn : task.groups()) {
    for (const auto& ig : inventory.groups()) {
      if (ig.name() == tgn) {
        for (const auto& host : ig.hosts()) {
          items.insert(host);
        }
      }
    }
  }
  for (const auto& host : task.hosts()) {
    items.insert(host);
  }
  BOOST_LOG_TRIVIAL(debug) << "find " << items.size();
  return items;
}

Env load_host_env(const peony::deploy::models::Inventory& inventory,
                  const peony::deploy::models::Task& task,
                  const std::string& host) {
  BOOST_LOG_TRIVIAL(debug) << "load host " << host << " from "
                           << inventory.name();
  Env items;
  // TODO global
  // TODO load from inventory
  // TODO load from group
  // TODO load from host
  BOOST_LOG_TRIVIAL(debug) << items.size() << " items";
  return items;
}

std::vector<Job> load_jobs(const peony::deploy::models::Inventory& inventory,
                           const peony::deploy::models::Recipe& recipe) {
  std::vector<Job> jobs;
  for (const auto& task : recipe.tasks()) {
    std::vector<Env> items;
    const auto hosts = load_hosts(inventory, task);
    for (const auto& host : hosts) {
      items.push_back(load_host_env(inventory, task, host));
    }
    jobs.push_back(std::make_pair(items, task));
  }
  return jobs;
}

void thread_func(const std::map<std::string, std::string>& env,
                 const peony::deploy::models::Task& task, int* errors,
                 std::mutex* errors_l) {
  try {
    peony::deploy::execute(env, task);
  } catch (const std::exception& e) {
    BOOST_LOG_TRIVIAL(fatal) << e.what();
    errors_l->lock();
    (*errors)++;
    errors_l->unlock();
  }
}

void peony::deploy::execute(const std::string& recipe_name,
                            const std::string& inventory_name) {
  peony::deploy::models::Recipe recipe;
  load(std::filesystem::path(recipe_name), recipe);
  peony::deploy::models::Inventory inventory;
  load(std::filesystem::path(inventory_name), inventory);
  const auto jobs = load_jobs(inventory, recipe);

  for (const auto& job : jobs) {
    int errors;
    std::mutex errors_l;
    std::vector<std::shared_ptr<std::thread>> pool;
    for (const auto& host : job.first) {
      auto it = std::make_shared<std::thread>(thread_func, host, job.second,
                                              &errors, &errors_l);
      pool.push_back(it);
    }
    for (const auto& it : pool) {
      it->join();
    }
    if (errors > 0) {
      return;
    }
  }
}