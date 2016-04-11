#include "series.h"

std::vector<int> series::digits(const std::string& s)
{
    auto result = std::vector<int>{};
    for (auto c : s) {
        result.push_back(c - '0');
    }
    return result;
}
