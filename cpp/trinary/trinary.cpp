#include "trinary.h"

int trinary::to_decimal(std::experimental::string_view sv)
{
    auto result = 0;
    auto pow3 = 1;
    for (auto it = sv.rbegin(); it != sv.rend(); ++it) {
        auto digit = *it - '0';
        if (0 > digit || 2 < digit) {
            return 0;
        }
        result += digit * pow3;
        pow3 *= 3;
    }
    return result;
}
