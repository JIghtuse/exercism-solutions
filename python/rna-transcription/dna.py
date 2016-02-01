import sys

def get_table(set1, set2):
    if sys.version_info < (3,):
        from string import maketrans
        return maketrans(set1, set2)
    else:
        return { ord(chars[0]) : chars[1] for chars in zip(set1, set2) }


def to_rna(strand):
    return strand.translate(get_table("GCTA", "CGAU"))
