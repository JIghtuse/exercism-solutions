#include "raindrops.h"

std::string raindrops::convert(unsigned ndrops)
{
    auto is_factor = [ndrops](unsigned factor) { return ndrops % factor == 0; };

    std::string result;
    if (is_factor(3)) {
        return "Pling";
    } else {
        return std::to_string(ndrops);
    }
}
