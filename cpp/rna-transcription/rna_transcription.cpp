#include <stdexcept>
#include "rna_transcription.h"

using namespace std;

char transcription::to_rna(char c)
{
    switch (c) {
    case 'C':
        return 'G';
    case 'G':
        return 'C';
    case 'A':
        return 'U';
    case 'T':
        return 'A';
    default:
        throw invalid_argument{"No such nucleotide"};
    }
}
