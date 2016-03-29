#include "clock.h"
#include <stdexcept>

namespace {

constexpr unsigned kMPH{60};
constexpr unsigned kHPD{24};

} // namespace

namespace date_independent {

clock::clock(Hours h, Minutes m)
    : m_hours{ h }
    , m_minutes{ m }
{
    if (m_hours >= kHPD || m_minutes >= kMPH) {
        throw std::domain_error{ "incorrect input" };
    }
}

clock clock::at(Hours h, Minutes m)
{
    return { h, m };
}

clock clock::plus(Minutes m) const
{
    auto total_minutes = m_hours * kMPH + m_minutes + m;
    return { (total_minutes / kMPH) % kHPD, total_minutes % kMPH };
}

clock clock::minus(Minutes m) const
{
    auto total_minutes = (kHPD + m_hours) * kMPH + m_minutes - m;
    return { (total_minutes / kMPH) % kHPD, total_minutes % kMPH };
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
    auto stringify = [&repr](unsigned n, unsigned offset) {
        const unsigned base{10};
        if (n >= base) {
            repr[offset] = n / 10 + '0';
        }
        repr[offset + 1] = n % 10 + '0';
    };
    stringify(m_hours, 0);
    stringify(m_minutes, 3);
    return repr;
}

} // namespace date_independent
