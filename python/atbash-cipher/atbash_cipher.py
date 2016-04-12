import string

DECODE_MAP = {
    chr(c): chr(ord('z') - c + ord('a')) for c in range(ord('a'), 1 + ord('z'))
}
ENCODE_MAP = {
    chr(c): chr(ord('z') - c + ord('a')) for c in range(ord('a'), 1 + ord('z'))
}


def decode(s):
    return ''.join(DECODE_MAP[c] for c in s if not c.isspace())


def encode(s):
    result = ''
    i = 0
    for c in s:
        if c in string.punctuation or c.isspace():
            continue

        # Insert space every 5 characters
        if i and i % 5 == 0:
            result += ' '
        i += 1

        c = c.lower()
        result += ENCODE_MAP.get(c, c)
    return result
