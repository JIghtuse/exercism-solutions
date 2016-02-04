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

string capitalize(const struct Bottles& bottles)
{
    ostringstream out;
    out << bottles;
    auto result = out.str();
    result[0] = toupper(result[0]);
    return result;
}

string beer::verse(size_t n)
{
    auto bottles = Bottles{n};

    ostringstream out;
    out << capitalize(bottles) << " on the wall, " << bottles << ".\n";

    if (!bottles.n)
        out << "Go to the store and buy some more, "
            << Bottles{99} << " on the wall.\n";
    else
        out << "Take " << (bottles.n == 1 ? "it" : "one")
            << " down and pass it around, "
            << Bottles{n - 1} << " on the wall.\n";

    return out.str();
}

string beer::sing(size_t from, size_t down_to)
{
    ostringstream out;
    size_t i;
    for (i = from; i != down_to; --i)
        out << verse(i) << '\n';
    out << verse(i);
    return out.str();
}
