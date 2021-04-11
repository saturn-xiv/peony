#include "peony/image-magick.h"
#include "peony/crypt.h"

#include <streambuf>

#include <boost/log/trivial.hpp>

#define PEONY_PNG_EXT "png"

void peony::image_magick::resize(const std::filesystem::path& src,
                                 const uint16_t width, const uint16_t height,
                                 const std::filesystem::path& target) {
  std::stringstream ss;
  ss << "convert -resize $(identify -ping -format '%wx%h!' {back}) {cover} "
        "{tmp} && convert {back} -compose over {tmp} -composite {target}";

  const auto cmd = ss.str();
  BOOST_LOG_TRIVIAL(debug) << cmd;
  auto _ = system(cmd.c_str());
}

void peony::image_magick::merge(const std::filesystem::path& src,
                                const std::filesystem::path& cover,
                                const std::filesystem::path& target) {
  const auto tmp = peony::random::file(PEONY_PNG_EXT);
  std::stringstream ss;
  ss << "convert -resize $(identify -ping -format '%wx%h!' '" << src << "') '"
     << cover << "' '" << tmp << "' && convert '" << src << "' -compose over '"
     << tmp << "' -composite '" << target << "'";

  const auto cmd = ss.str();
  BOOST_LOG_TRIVIAL(debug) << cmd;
  auto _ = system(cmd.c_str());
}

void peony::image_magick::rotate(const std::filesystem::path& src,
                                 const int8_t degrees,
                                 const std::filesystem::path& target) {
  std::stringstream ss;
  ss << "convert -rotate '" << degrees << "' '" << src << "' '" << target
     << "'";
  const auto cmd = ss.str();
  BOOST_LOG_TRIVIAL(debug) << cmd;
  auto _ = system(cmd.c_str());
}
