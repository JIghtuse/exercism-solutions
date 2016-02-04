#include <sstream>
#include "beer_song.h"

using namespace std;

string beer::verse(size_t)
{
    ostringstream out;
    out << "8 bottles of beer on the wall, 8 bottles of beer.\n";
    out << "Take one down and pass it around, 7 bottles of beer on the wall.\n";

    return out.str();
}
