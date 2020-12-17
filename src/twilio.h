#ifndef PEONY_TWILIO_H_
#define PEONY_TWILIO_H_

#include "common.h"

namespace peony
{
    namespace twilio
    {
        class Config
        {
        public:
            Config(const toml::table &root);

            operator toml::table() const;

        private:
            std::string from;
            std::string account_sid;
            std::string auth_token;
        };
    } // namespace twilio
} // namespace peony
#endif
