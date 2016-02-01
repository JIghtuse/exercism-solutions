def is_pangram(sentence):
    alphabet = set(chr(i) for i in range(ord('a'), ord('z') + 1))
    letters = set(sorted(sentence.lower()))
    return len(alphabet) == len(alphabet & letters)
