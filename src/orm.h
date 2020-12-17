#ifndef PEONY_ORM_H_
#define PEONY_ORM_H_

#include "common.h"

#include <pqxx/pqxx>
#include <sqlite3.h>

namespace peony
{
    namespace postgresql
    {
        class Config
        {
        public:
            Config(const toml::table &root);

            operator toml::table() const;

        private:
            std::string host;
            unsigned short port;
            std::string user;
            std::string password;
            std::string db;
            unsigned short pool_size;
        };
    } // namespace postgresql
    namespace sqlite3
    {
        class Config
        {
        private:
            std::string file;
        };
    } // namespace sqlite3
} // namespace peony

#endif