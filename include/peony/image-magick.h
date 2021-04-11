#pragma once

#include <cstdint>
#include <filesystem>

namespace peony {
namespace image_magick {
void resize(const std::filesystem::path& src, const uint16_t width,
            const uint16_t height, const std::filesystem::path& target);
void merge(const std::filesystem::path& src, const std::filesystem::path& cover,
           const std::filesystem::path& target);
void rotate(const std::filesystem::path& src, const int8_t degrees,
            const std::filesystem::path& target);
}  // namespace image_magick
}  // namespace peony
