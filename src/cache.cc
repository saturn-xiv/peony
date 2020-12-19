#include "cache.h"

peony::Redis::Redis(const toml::table &root) {
  this->host = root["host"].value<std::string>().value_or(PEONY_DEFAULT_HOST);
  this->port = root["port"].value<unsigned int>().value_or(6379);
  this->user = root["user"].value<std::string>();
  this->password = root["password"].value<std::string>();
  this->prefix = root["prefix"].value<std::string>();
  this->db = root["db"].value<unsigned short>().value_or(0);
  this->pool_size =
      root["pool-size"].value<long>().value_or(PEONY_DEFAULT_POOL_SIZE);
}

peony::Redis::operator toml::table() const {
  toml::table root;
  root.insert("host", this->host);
  root.insert("port", this->port);
  if (this->prefix) {
    root.insert("prefix", this->prefix.value());
  }
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

// void peony::redis::Connection::clear() {
//   redisReply *reply = (redisReply *)redisCommand(context, "FLUSHDB");
//   if (reply->type == REDIS_REPLY_ERROR) {
//     throw std::runtime_error(reply->str);
//   }
//   freeReplyObject(reply);
// }
// peony::redis::Connection::~Connection() {
//   BOOST_LOG_TRIVIAL(debug) << "close redis connection";
//   if (NULL != context) {
//     redisFree(this->context);
//     context = NULL;
//   }
// }
// // ---------------------------
// peony::redis::Factory::Factory(const std::string host,
//                                const unsigned short int port,
//                                const unsigned short int db,
//                                const std::string prefix)
//     : host(host), port(port), db(db), prefix(prefix) {
//   BOOST_LOG_TRIVIAL(info) << "init redis connection factory";
// }

// std::string peony::redis::Factory::name() const {
//   std::ostringstream ss;
//   ss << "tcp://" << host << ":" << port << "/" << db << "/" << prefix;
//   return ss.str();
// }

// std::shared_ptr<peony::pool::Connection> peony::redis::Factory::create() {
//   BOOST_LOG_TRIVIAL(debug) << "open redis " << name();
//   redisContext *context = redisConnect(host.c_str(), port);
//   if (context == NULL) {
//     throw std::invalid_argument("can't allocate redis context");
//   }

//   {
//     redisReply *reply = (redisReply *)redisCommand(context, "SELECT %i", db);
//     if (reply->type == REDIS_REPLY_ERROR) {
//       throw std::invalid_argument(reply->str);
//     }
//     freeReplyObject(reply);
//   }

//   std::shared_ptr<Connection> con(new Connection());
//   con->prefix = this->prefix;
//   con->context = context;

//   return std::static_pointer_cast<peony::pool::Connection>(con);
// }

// // ------------------------

// // ---------------------------

// std::shared_ptr<peony::pool::Pool<peony::redis::Connection>>
// peony::redis::Config::open() const {
//   std::shared_ptr<peony::redis::Factory> factory(new peony::redis::Factory(
//       this->host, this->port, this->db, this->prefix));
//   std::shared_ptr<peony::pool::Pool<peony::redis::Connection>> pool(
//       new peony::pool::Pool<peony::redis::Connection>(this->pool_size,
//                                                       factory));
//   return pool;
// }
