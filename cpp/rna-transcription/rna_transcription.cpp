#include <algorithm>
#include <stdexcept>
#include "rna_transcription.h"

using namespace std;
using std::experimental::string_view;

char transcription::to_rna(char dna_nucleotide)
{
    switch (dna_nucleotide) {
    case 'C':
        return 'G';
    case 'G':
        return 'C';
    case 'A':
        return 'U';
    case 'T':
        return 'A';
    default:
        throw invalid_argument{ "No such nucleotide" };
    }
}

string transcription::to_rna(string_view strand)
{
    string result;
    transform(strand.begin(), strand.end(), back_inserter(result),
        static_cast<char (*)(char)>(to_rna));
    return result;
}
