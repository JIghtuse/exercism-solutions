#include "food_chain.h"

using namespace std;

string const food_chain::verse(size_t n)
{
    string msg;
    switch (n) {
    case 1:
        msg = "I know an old lady who swallowed a fly.\n"
              "I don't know why she swallowed the fly. Perhaps she'll die.\n";
        break;
    default:
        break;
    }
    return msg;
}
