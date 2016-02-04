#ifndef BEER_SONG_H
#define BEER_SONG_H

#include <string>

namespace beer {
    std::string verse(size_t n);
    std::string sing(size_t from, size_t down_to);
}

#endif // BEER_SONG_H
