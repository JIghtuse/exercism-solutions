#include <sstream>
#include "beer_song.h"

using namespace std;

struct Bottles {
    size_t n;
};

ostream& operator<<(ostream& out, const struct Bottles& bottles)
{
    switch (bottles.n) {
    case 0:
        out << "no more bottles of beer";
        break;
    case 1:
        out << "1 bottle of beer";
        break;
    default:
        out << bottles.n << " bottles of beer";
        break;
    }
    return out;
}

string beer::verse(size_t n)
{
    auto bottles = Bottles{n};

    ostringstream out;
    out << bottles << " on the wall, " << bottles << ".\n";
    out << "Take " << (bottles.n == 1 ? "it" : "one")
        << " down and pass it around, "
        << Bottles{n - 1} << " on the wall.\n";

    return out.str();
}
