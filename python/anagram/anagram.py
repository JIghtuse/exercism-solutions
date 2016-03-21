def detect_anagrams(word, inputs):
    def lower_sorted(s):
        s = s.lower()
        return ''.join(sorted(s))

    inputs = [inp for inp in inputs if inp.lower() != word.lower()]
    word = lower_sorted(word)

    return [inp for inp in inputs if lower_sorted(inp) == word]
