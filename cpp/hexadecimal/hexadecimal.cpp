#include "hexadecimal.h"
#include <cctype>

int hexadecimal::convert(std::experimental::string_view sv)
{
    if (sv.empty()) {
        return 0;
    }
    char c = sv[0];
    if (isdigit(c)) {
        return c - '0';
    }
    if ('a' <= c && c <= 'h') {
        return c - 'a' + 10;
    }
    return 0;
}
