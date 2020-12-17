#ifndef PEONY_CACHE_H_
#define PEONY_CACHE_H_

#include "common.h"

#include <hiredis/hiredis.h>

namespace peony
{
    namespace redis
    {
        class Config
        {
        public:
            Config(const toml::table &root);

            operator toml::table() const;

        private:
            std::string host;
            unsigned short port;
            std::optional<std::string> user;
            std::optional<std::string> password;
            std::string prefix;
            unsigned short db;
        };
    } // namespace redis
} // namespace peony
#endif