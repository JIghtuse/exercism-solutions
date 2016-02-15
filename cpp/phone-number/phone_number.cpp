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
