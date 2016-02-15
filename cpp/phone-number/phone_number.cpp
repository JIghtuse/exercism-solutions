#include <algorithm>
#include <cctype>
#include <sstream>
#include "phone_number.h"

using std::experimental::string_view;

std::string extract_digits(string_view sv)
{
    std::ostringstream oss;
    for (auto c : sv)
        if (isdigit(c))
            oss << c;
    return oss.str();
}

phone_number::phone_number(string_view sv)
    : number_{extract_digits(sv)}
{
    const auto valid_length = 10;

    if (1 + valid_length == number_.length() && '1' == number_[0])
        number_.erase(0, 1);

    // exercism: Why not to throw exception here?
    if (valid_length != number_.length()) {
        number_.resize(10);
        std::fill(number_.begin(), number_.end(), '0');
    }
}

std::string phone_number::number() const
{
    return number_;
}

std::string phone_number::area_code() const
{
    return number_.substr(0, 3);
}

phone_number::operator std::string() const
{
    std::ostringstream oss;
    oss << "(" << area_code() << ") "
        << number_.substr(3, 3) << "-"
        << number_.substr(6);
    return oss.str();
}
