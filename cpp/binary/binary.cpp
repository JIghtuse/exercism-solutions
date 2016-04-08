#include "binary.h"

int binary::convert(std::experimental::string_view s)
{
    int res = 0;
    int power_of_two = 1;
    for (auto it = s.rbegin(); it != s.rend(); ++it, power_of_two *= 2) {
        if (*it == '1') {
            res += power_of_two;
        } else if (*it != '0') {
            // Unexpected symbol, return immediately
            return 0;
        }
    }
    return res;
}
