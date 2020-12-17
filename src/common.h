#ifndef PEONY_COMMON_H_
#define PEONY_COMMON_H_

#include <cassert>
#include <cstdlib>
#include <exception>
#include <iomanip>
#include <iostream>
#include <mutex>
#include <regex>
#include <set>
#include <signal.h>
#include <sstream>
#include <stdexcept>
#include <string>
#include <thread>
#include <unistd.h>

#include <boost/date_time/posix_time/posix_time.hpp>
#include <boost/foreach.hpp>
#include <boost/lambda/lambda.hpp>
#include <boost/log/trivial.hpp>
#include <boost/make_shared.hpp>
#include <boost/program_options.hpp>
#include <boost/uuid/uuid.hpp>
#include <boost/uuid/uuid_generators.hpp>
#include <boost/uuid/uuid_io.hpp>
#include <cpprest/containerstream.h>
#include <cpprest/filestream.h>
#include <cpprest/http_client.h>
#include <cpprest/http_listener.h>
#include <cpprest/interopstream.h>
#include <cpprest/json.h>
#include <cpprest/producerconsumerstream.h>
#include <cpprest/rawptrstream.h>
#include <cpprest/uri.h>
#include <cpprest/ws_client.h>
#include <nlohmann/json.hpp>

#include "config.h"

#define TOML_EXCEPTIONS 1
#include <toml.hpp>

#define PEONY_STR2TS(x) \
    boost::posix_time::time_from_string(x.as<std::string>())
#define PEONY_APPLICATION_JSON_UTF8 "application/json; charset=UTF-8"
#define PEONY_TEXT_HTML_UTF8 "text/html; charset=UTF-8"

#endif