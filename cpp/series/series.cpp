#include "series.h"

std::vector<int> series::digits(const std::string& s)
{
    auto result = std::vector<int>{};
    for (auto c : s) {
        result.push_back(c - '0');
    }
    return result;
}

std::vector<std::vector<int>> series::slice(const std::string& s, unsigned size)
{
    auto result = std::vector<std::vector<int>>{};
    result.reserve(1 + s.length() / size);
    for (auto& v: digits(s)) {
        result.push_back(std::vector<int>{v});
    }
    return result;
}
