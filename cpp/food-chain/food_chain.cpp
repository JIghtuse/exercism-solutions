#include "food_chain.h"

#include <cassert>
#include <sstream>
#include <vector>

using namespace std;

struct Verse {
    string item;
    string desc;
};

vector<Verse> const verse_parts{
    Verse{ "fly", "I don't know why she swallowed the fly. Perhaps she'll die." },
    Verse{ "spider", "wriggled and jiggled and tickled inside her." },
    Verse{ "bird", "How absurd to swallow a bird!" },
    Verse{ "cat", "Imagine that, to swallow a cat!" },
    Verse{ "dog", "What a hog, to swallow a dog!" },
    Verse{ "goat", "Just opened her throat and swallowed a goat!" },
    Verse{ "cow", "I don't know how she swallowed a cow!" },
    Verse{ "horse", "She's dead, of course!" },
};

string const food_chain::verse(size_t n)
{
    assert(n <= verse_parts.size());

    ostringstream out;
    out << "I know an old lady who swallowed a " << verse_parts[n - 1].item << ".\n";
    if (n == 2)
        out << "It ";
    out << verse_parts[n - 1].desc << '\n';

    if (n == 1 || n == verse_parts.size())
        return out.str();

    while (--n) {
        out << "She swallowed the " << verse_parts[n].item
            << " to catch the " << verse_parts[n - 1].item;
        if (n == 2)
            out << " that " << verse_parts[n - 1].desc;
        else
            out << ".";
        out << '\n';
    }
    out << verse_parts[0].desc << '\n';
    return out.str();
}

string const food_chain::verses(size_t begin, size_t end)
{
    ostringstream out;
    for (size_t i = begin; i != end + 1; ++i) {
        out << verse(i) << '\n';
    }
    return out.str();
}

string const food_chain::sing()
{
    return verses(1, verse_parts.size());
}
