#include "beer_song.h"
#include <sstream>

using namespace std;

struct Bottles {
    explicit Bottles(size_t n, const std::string& what="beer")
        : what{what}
        , n{n}
    {}
    const char* one() const { return n == 1 ? "it" : "one"; }
    void stream(std::ostream& out, bool capitalized) const
    {
        switch (n) {
        case 0:
            out << (capitalized ? 'N' : 'n') << "o more bottles of " << what;
            break;
        case 1:
            out << "1 bottle of " << what;
            break;
        default:
            out << n << " bottles of " << what;
            break;
        }
    }
    std::string what;
    size_t n;
};

ostream& operator<<(ostream& out, const struct Bottles& bottles)
{
    bottles.stream(out, false);
    return out;
}

string beer::verse(size_t n)
{
    auto bottles = Bottles{n};

    ostringstream out;
    bottles.stream(out, true);
    out << " on the wall, " << bottles << ".\n";

    if (!bottles.n)
        out << "Go to the store and buy some more, "
            << Bottles{99} << " on the wall.\n";
    else
        out << "Take " << bottles.one() << " down and pass it around, "
            << Bottles{n - 1} << " on the wall.\n";

    return out.str();
}

string beer::sing(size_t from, size_t down_to)
{
    ostringstream out;
    size_t i;
    for (i = from; i > down_to; --i)
        out << verse(i) << '\n';
    out << verse(i);
    return out.str();
}
