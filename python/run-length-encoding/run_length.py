import re
from itertools import groupby


def encode(s):
    res = ""
    for char, group in groupby(s):
        length = len(list(group))
        if length > 1:
            res += str(length)
        res += char
    return res


def decode(s):
    nums = (int(n) if n else 1 for n in re.split('\D', s))
    chars = ''.join(n for n in re.split('\d', s) if n)

    return ''.join(char * number for (number, char) in zip(nums, chars))
