#include "clock.h"
#include <stdexcept>

namespace date_independent {

clock::clock(Hours h, Minutes m)
    : m_hours{ h }
    , m_minutes{ m }
{
    if (m_hours > 23 || m_minutes > 59) {
        throw std::domain_error{ "incorrect input" };
    }
}

clock clock::at(Hours h, Minutes m)
{
    return { h, m };
}

clock clock::plus(Minutes m) const
{
    auto total_minutes = m_hours * 60 + m_minutes + m;
    return { (total_minutes / 60) % 24, total_minutes % 60 };
}

clock clock::minus(Minutes m) const
{
    auto total_minutes = (24 + m_hours) * 60 + m_minutes - m;
    return { (total_minutes / 60) % 24, total_minutes % 60 };
}

bool operator==(const clock& a, const clock& b)
{
    return a.hours() == b.hours()
        && a.minutes() == b.minutes();
}

bool operator!=(const clock& a, const clock& b)
{
    return !(a == b);
}

clock::operator std::string() const
{
    std::string repr{ "00:00" };
    auto stringify = [&repr](int n, int offset) {
        if (n > 9) {
            repr[offset] = n / 10 + '0';
        }
        repr[offset + 1] = n % 10 + '0';
    };
    stringify(m_hours, 0);
    stringify(m_minutes, 3);
    return repr;
}

} // namespace date_independent
