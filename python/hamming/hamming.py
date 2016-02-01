def distance(strand1, strand2):
    return sum(n[0] != n[1] for n in zip(strand1, strand2))
