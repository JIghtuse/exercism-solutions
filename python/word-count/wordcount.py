import re
from collections import Counter

def word_count(phrase):
    return Counter(w.lower() for w in re.split('[_\W]+', phrase) if w)
