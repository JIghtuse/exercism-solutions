#include <cctype>
#include <sstream>
#include "phone_number.h"

using std::experimental::string_view;
using Size = std::string::size_type;

namespace {

constexpr const Size kValidNumberLength = 10;
constexpr const Size kAreaLength = 3;
constexpr const Size kExchangeLength = 3;

const char kInvalidNumber[] = "0000000000";

} // namespace

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
    if (1 + kValidNumberLength == number_.length() && '1' == number_.front()) {
        number_.erase(number_.begin());
    }

    // exercism: Why not to throw exception here?
    if (kValidNumberLength != number_.length()) {
        number_ = kInvalidNumber;
    }
}

std::string phone_number::number() const
{
    return number_;
}

std::string phone_number::area_code() const
{
    return number_.substr(0, kAreaLength);
}

phone_number::operator std::string() const
{
    std::ostringstream oss;
    oss << "(" << area_code() << ") "
        << number_.substr(kAreaLength, kExchangeLength)
        << "-"
        << number_.substr(kAreaLength + kExchangeLength);
    return oss.str();
}
