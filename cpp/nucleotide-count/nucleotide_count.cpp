#include "nucleotide_count.h"

using namespace std;
using std::experimental::string_view;

map<char, int> dna::counter::nucleotide_counts() const
{
    return map<char, int>{
        { 'A', 0 },
        { 'T', 0 },
        { 'C', 0 },
        { 'G', 0 },
    };
}
