#include "etl.h"

std::map<char, int> etl::transform(std::map<int, std::vector<char>> old)
{
    auto result = std::map<char, int>{};
    for (const auto& pair : old) {
        for (const auto& item : pair.second) {
            result[::tolower(item)] = pair.first;
        }
    }
    return result;
}

