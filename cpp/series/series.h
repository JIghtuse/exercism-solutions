#pragma once

#include <vector>
#include <experimental/string_view>

namespace series {

std::vector<int> digits(std::experimental::string_view sv);
std::vector<std::vector<int>> slice(std::experimental::string_view, std::size_t size);

} // namespace series
