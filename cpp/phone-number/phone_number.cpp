#include <cctype>
#include "phone_number.h"

using std::experimental::string_view;

phone_number::phone_number(string_view sv)
    : oss{}
{
    for (auto c : sv)
        if (isdigit(c))
            oss << c;
}

std::string phone_number::number() const
{
    return oss.str();
}
