#include "series.h"
#include <stdexcept>

using std::experimental::string_view;

std::vector<int> series::digits(string_view sv)
{
    auto result = std::vector<int>{};
    for (auto c : sv) {
        result.push_back(c - '0');
    }
    return result;
}

std::vector<std::vector<int>> series::slice(string_view sv, std::size_t size)
{
    if (size > sv.length()) {
        throw std::domain_error("Not enough digits to slice");
    }
    auto result = std::vector<std::vector<int>>{};
    result.reserve(1 + sv.length() / size);

    for (std::size_t i = 0; i != 1 + sv.length() - size; ++i) {
        result.emplace_back(digits(sv.substr(i, size)));
    }
    return result;
}
