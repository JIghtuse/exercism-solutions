#include "hexadecimal.h"
#include <cctype>
#include <algorithm>
#include <experimental/optional>
#include <numeric>

using std::experimental::optional;
using std::experimental::nullopt;

namespace {

constexpr int kHexBase = 16;

inline optional<int> hex_char_to_digit(char c)
{
    if (isdigit(c)) {
        return c - '0';
    }
    if ('a' <= c && c <= 'h') {
        return c - 'a' + 10;
    }
    return nullopt;
}

} // namespace

int hexadecimal::convert(std::experimental::string_view sv)
{
    int result = 0;
    for (auto c : sv) {
        auto digit = hex_char_to_digit(c);
        if (!digit) {
            return 0;
        }
        result = kHexBase * result + *digit;
    }
    return result;
}
