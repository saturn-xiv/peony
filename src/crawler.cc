#include "crawler.h"

void peony::Crawler::get(const std::string &name, const std::string &home,
                         const std::string &path) const {
  BOOST_LOG_TRIVIAL(info) << "fetch " << name << " from " << home << path;
  httplib::Client cli(home.c_str());
  cli.enable_server_certificate_verification(false);
  auto res = cli.Get(path.c_str());
  if (!res) {
    std::stringstream ss;
    ss << res.error();
    throw std::runtime_error(ss.str());
  }
  if (res->status != 200) {
    throw std::runtime_error(res->body);
  }
  pqxx::work tx(*(this->connection));
  pqxx::result rst = tx.exec_prepared("crawlers_logs.insert", name, res->body);
  tx.commit();
  BOOST_LOG_TRIVIAL(info) << "done.";
}

std::optional<std::pair<std::string, boost::posix_time::ptime>>
peony::Crawler::latest(const std::string &name) const {
  pqxx::work tx(*(this->connection));
  pqxx::result rst = tx.exec_prepared("crawlers_logs.find-by-name", name);
  tx.commit();

  for (auto it = rst.begin(); it != rst.end(); it++) {
    const auto body = it["body"].as<std::string>();
    const auto created_at = PEONY_STR2TS(it["created_at"]);
    std::pair<std::string, boost::posix_time::ptime> ret(body, created_at);
    return ret;
  }

  return std::nullopt;
}

void peony::Crawler::rolling(const unsigned short years) const {
  pqxx::work tx(*(this->connection));
  pqxx::result rst = tx.exec_prepared("crawlers_logs.rolling-by-years", years);
  tx.commit();
}
// void peony::Crawler::execute() const {
//   for (auto it : this->sources) {
//     try {
//       this->execute(it.first, it.second);
//     } catch (const std::exception &e) {
//       std::cout << e.what() << std::endl;
//     }
//   }
// }

// void peony::Crawler::execute(const std::string &name) const {
//   for (auto it : this->sources) {
//     if (it.first == name) {
//       this->execute(it.first, it.second);
//       return;
//     }
//   }
//   BOOST_LOG_TRIVIAL(error) << "can't find crawler nameed by " << name;
// }

// void peony::Crawler::execute(const std::string &name,
//                              const std::string &url) const {
//

//   httplib::Client cli(url);
//   auto res = cli.Get("/hi");
//   cli.enable_server_certificate_verification(false);

//   // web::http::client::http_client_config cfg;
//   // cfg.set_validate_certificates(false);

//   // web::http::client::http_client client(U(url), cfg);
//   // std::string body;
//   // client.request(web::http::methods::GET)
//   //     .then([](const web::http::http_response &response) {
//   //       const auto code = response.status_code();
//   //       if (code == web::http::status_codes::OK) {
//   //         return response.extract_string();
//   //       }
//   //       std::stringstream ss;
//   //       ss << "HTTP " << code;
//   //       throw std::runtime_error(ss.str());
//   //     })
//   //     .then([&body](const pplx::task<std::string> &task) { body =
//   //     task.get();
//   //     }) .wait();

// }