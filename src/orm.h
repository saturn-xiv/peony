#ifndef PEONY_ORM_H_
#define PEONY_ORM_H_

#include <sqlite3.h>
#include <pqxx/pqxx>

#include "common.h"
#include "crypt.h"
#include "pool.h"

namespace peony {
namespace orm {
class Migration {
 public:
  Migration(const std::string version, const std::string name,
            const std::string up, const std::string down,
            const std::optional<boost::posix_time::ptime> run_at);
  friend std::ostream &operator<<(std::ostream &out, const Migration &self) {
    out << "### " << self.version << " " << self.name;
    if (self.run_at) {
      out << " "
          << boost::posix_time::to_iso_extended_string(self.run_at.value());
    }
    out << " ###" << std::endl
        << "--- UP ---" << std::endl
        << self.up << "--- DOWN ---" << std::endl
        << self.down;

    return out;
  }
  bool operator<(const Migration &self) const;

  friend class SchemaDao;

  std::string version;
  std::string name;
  std::string up;
  std::string down;
  std::optional<boost::posix_time::ptime> run_at;
};

class SchemaDao {
 public:
  void generate(const std::string &name) const;
  void load();
  void migrate();
  void rollback();
  friend std::ostream &operator<<(std::ostream &out, const SchemaDao &self) {
    const std::string fmt = "%1$-15s %2$-24s ";

    out << (boost::format(fmt) % "VERSION" % "NAME") << "RUN AT" << std::endl;
    for (auto it = self.migrations.begin(); it != self.migrations.end(); it++) {
      out << (boost::format(fmt) % it->version % it->name);
      if (it->run_at) {
        out << boost::posix_time::to_iso_extended_string(it->run_at.value());
      } else {
        out << "N/A";
      };
      out << std::endl;
    }
    return out;
  }

 protected:
  virtual void execute(const std::string &script) const = 0;
  virtual void delete_(const std::string &version) const = 0;
  virtual void insert(const std::string &version) const = 0;
  virtual std::optional<boost::posix_time::ptime> run_at(
      const std::string &version) const = 0;

  std::vector<Migration> migrations;
  std::mutex locker;
  std::filesystem::path root;
};
}  // namespace orm

// https://www.postgresql.org/docs/current/libpq-connect.html
// https://libpqxx.readthedocs.io/en/latest/index.html
namespace postgresql {
void load_prepares(pqxx::connection &connection,
                   const std::filesystem::path &path);
class Connection : public peony::pool::Connection {
 public:
  ~Connection() override;

  friend class Factory;
  std::shared_ptr<pqxx::connection> context;

 private:
};

class Factory : public peony::pool::Factory {
 public:
  Factory(const std::string host, const unsigned short int port,
          const std::string db, const std::string user,
          const std::optional<std::string> password,
          const std::optional<std::filesystem::path> prepares);
  std::shared_ptr<peony::pool::Connection> create() override;
  std::string name() const override;

 private:
  const std::string host;
  const unsigned short int port;
  const std::string user;
  const std::optional<std::string> password;
  const std::string db;
  const std::optional<std::filesystem::path> prepares;
};

class Config {
 public:
  Config(const toml::table &root);

  operator toml::table() const;

  std::shared_ptr<peony::pool::Pool<Connection>> open(
      const std::optional<std::filesystem::path> &prepares) const;

 private:
  std::string host;
  unsigned short port;
  std::string user;
  std::optional<std::string> password;
  std::string db;
  unsigned short pool_size;
};

class SchemaDao : public peony::orm::SchemaDao {
 public:
  SchemaDao(const std::shared_ptr<pqxx::connection> connection,
            const std::filesystem::path root);

  void execute(const std::string &script) const override;
  void delete_(const std::string &version) const override;
  void insert(const std::string &version) const override;
  std::optional<boost::posix_time::ptime> run_at(
      const std::string &version) const override;

 private:
  const std::shared_ptr<pqxx::connection> connection;
};

}  // namespace postgresql
namespace sqlite3 {
class Config {
 private:
  std::filesystem::path file;
};

}  // namespace sqlite3
}  // namespace peony

#endif