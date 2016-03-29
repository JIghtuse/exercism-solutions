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
    operator std::string() const;

private:
    Hours hours;
    Minutes minutes;
};

} // namespace date_independent
