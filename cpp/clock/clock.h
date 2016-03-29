#pragma once

#include <string>

namespace date_independent {

using Hours = unsigned;
using Minutes = unsigned;

class clock {
public:
    clock(Hours, Minutes);
    static clock at(Hours h, Minutes m=0);
    clock plus(Minutes);
    clock minus(Minutes);
    operator std::string() const;

    Hours hours() const { return m_hours; }
    Minutes minutes() const { return m_minutes; }
private:
    Hours m_hours;
    Minutes m_minutes;
};

bool operator==(const clock& a, const clock& b);
bool operator!=(const clock& a, const clock& b);

} // namespace date_independent
