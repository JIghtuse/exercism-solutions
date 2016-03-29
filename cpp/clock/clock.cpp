#include "clock.h"

namespace date_independent {

clock::clock(Hours h, Minutes m)
    : hours{ h }
    , minutes{ m }
{
}

clock clock::at(Hours h, Minutes m)
{
    return { h, m };
}

clock::operator std::string() const
{
    std::string repr{"00:00"};
    auto stringify = [&repr](int n, int offset) {
        if (n > 9) {
            repr[offset] = n / 10 + '0';
        }
        repr[offset + 1] = n % 10 + '0';
    };
    stringify(hours, 0);
    stringify(minutes, 3);
    return repr;
}

} // namespace date_independent
