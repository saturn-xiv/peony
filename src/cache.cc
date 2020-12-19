#include "cache.h"

void peony::cache::set(redisContext *ctx, const std::string &key,
                       const nlohmann::json &val, size_t ttl) {
  redisReply *reply = (redisReply *)redisCommand(
      ctx, "SET %s %s EX %i", key.c_str(), val.dump().c_str(), ttl);
  if (reply->type == REDIS_REPLY_ERROR) {
    BOOST_LOG_TRIVIAL(error) << "SET " << key << " " << reply->str;
  }
  freeReplyObject(reply);
}

std::optional<nlohmann::json> peony::cache::get(redisContext *ctx,
                                                const std::string &key) {
  redisReply *reply = (redisReply *)redisCommand(ctx, "GET %s", key.c_str());
  if (reply->type == REDIS_REPLY_ERROR) {
    BOOST_LOG_TRIVIAL(error) << "get by key " << reply->str;
    return std::nullopt;
  }
  nlohmann::json it(reply->str);
  freeReplyObject(reply);
  return it;
}

void peony::cache::clear(redisContext *ctx) {
  redisReply *reply = (redisReply *)redisCommand(ctx, "FLUSHDB");
  if (reply->type == REDIS_REPLY_ERROR) {
    BOOST_LOG_TRIVIAL(error) << "flush db " << reply->str;
  }
  freeReplyObject(reply);
}

std::vector<std::string> peony::cache::all(redisContext *ctx) {
  redisReply *reply = (redisReply *)redisCommand(ctx, "KEYS *");
  if (reply->type == REDIS_REPLY_ERROR) {
    BOOST_LOG_TRIVIAL(error) << "list key " << reply->str;
  }

  std::vector<std::string> items;
  for (auto i = 0; i < reply->elements; i++) {
    std::string k(reply->element[i]->str);
    items.push_back(k);
  }
  freeReplyObject(reply);
  return items;
}

redisContext *peony::redis::Config::open() {
  auto context = redisConnect(this->host.c_str(), this->port);
  if (context == NULL) {
    throw std::invalid_argument("can't allocate redis context");
  }

  {
    redisReply *reply = (redisReply *)redisCommand(context, "SELECT %i", db);
    if (NULL == reply) {
      throw std::invalid_argument("can't select db");
    }
    if (reply->type == REDIS_REPLY_ERROR) {
      auto ex = std::invalid_argument(reply->str);
      freeReplyObject(reply);
      throw ex;
    }
    freeReplyObject(reply);
  }
  return context;
}

peony::redis::Config::Config(const toml::table &root) {
  this->host = root["host"].value<std::string>().value_or(PEONY_DEFAULT_HOST);
  this->port = root["port"].value<unsigned int>().value_or(6379);
  this->user = root["user"].value<std::string>();
  this->password = root["password"].value<std::string>();
  this->db = root["db"].value<unsigned short>().value_or(0);
  this->pool_size =
      root["pool-size"].value<long>().value_or(PEONY_DEFAULT_POOL_SIZE);
}

peony::redis::Config::operator toml::table() const {
  toml::table root;
  root.insert("host", this->host);
  root.insert("port", this->port);
  if (this->user) {
    root.insert("user", this->user.value());
  }
  if (this->password) {
    root.insert("password", this->password.value());
  }
  root.insert("db", this->db);
  root.insert("pool-size", (long)this->pool_size);
  return root;
};

peony::redis::Pool::~Pool() {
  BOOST_LOG_TRIVIAL(warning) << "destory redis pool " << this->config;
  for (auto it : this->pool) {
    redisFree(it);
  }
  for (auto it : this->used) {
    redisFree(it);
  }
}

peony::redis::Pool::Pool(const std::shared_ptr<Config> config)
    : config(config) {
  BOOST_LOG_TRIVIAL(info) << "open redis pool " << this->config;
  while (this->pool.size() < this->config->pool_size) {
    this->pool.push_back(this->config->open());
  }
}

redisContext *peony::redis::Pool::get() {
  std::lock_guard<std::mutex> lock(this->locker);
  if (this->pool.size() == 0) {
    for (auto it = this->used.begin(); it != this->used.end(); ++it) {
      if (NULL == (*it)) {
        BOOST_LOG_TRIVIAL(debug)
            << "creating new redis connection to replace discarded connection";
        auto con = this->config->open();
        this->used.erase(it);
        this->used.insert(con);
        return con;
      }
    }
    throw std::out_of_range("redis pool is full");
  }

  auto con = this->pool.front();
  this->pool.pop_front();
  this->used.insert(con);
  return con;
}

void peony::redis::Pool::release(redisContext *it) {
  std::lock_guard<std::mutex> lock(this->locker);
  this->pool.push_back(it);
  this->used.erase(it);
}

std::pair<size_t, size_t> peony::redis::Pool::status() {
  std::lock_guard<std::mutex> lock(this->locker);
  return std::make_pair(this->pool.size(), this->used.size());
}
