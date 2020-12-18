#ifndef PEONY_ENV_H_
#define PEONY_ENV_H_

#define TOML_EXCEPTIONS 1

#define PEONY_STR2TS(x) boost::posix_time::time_from_string(x.as<std::string>())
#define PEONY_APPLICATION_JSON_UTF8 "application/json; charset=UTF-8"
#define PEONY_TEXT_HTML_UTF8 "text/html; charset=UTF-8"
#define PEONT_TIMESTAMP_SIZE 14

#include "config.h"

#endif