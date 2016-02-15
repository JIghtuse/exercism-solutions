#include <cctype>
#include <sstream>
#include "phone_number.h"

using std::experimental::string_view;

phone_number::phone_number(string_view sv)
    : number_{}
{
    std::ostringstream oss;
    for (auto c : sv)
        if (isdigit(c))
            oss << c;
    number_ = oss.str();

    if (11 == number_.length() && '1' == number_[0])
        number_.erase(0, 1);
}

std::string phone_number::number() const
{
    return number_;
}
