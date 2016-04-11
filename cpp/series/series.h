#pragma once

#include <string>
#include <vector>

namespace series {

std::vector<int> digits(const std::string& s);
std::vector<std::vector<int>> slice(const std::string& s, unsigned size);

} // namespace series
