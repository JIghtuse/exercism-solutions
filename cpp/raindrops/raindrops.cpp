#include "raindrops.h"
#include <algorithm>
#include <map>

using Factor = unsigned;
using Voice = std::string;
using FW = std::pair<Factor, Voice>;

const std::map<Factor, Voice> voices{
    { 3, "Pling" },
    { 5, "Plang" },
    { 7, "Plong" },
};

std::string raindrops::convert(unsigned ndrops)
{
    auto append_voice = [ndrops](const std::string& a, const FW& factor_voice) {
        return ndrops % factor_voice.first == 0 ? a + factor_voice.second : a;
    };
    auto melody = std::accumulate(voices.begin(), voices.end(), std::string{}, append_voice);
    return !melody.empty() ? melody : std::to_string(ndrops);
}
