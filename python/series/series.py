def slices(s, n):
    """Generates all the contiguous substrings of length `n` in `s`"""

    if n == 0 or len(s) < n:
        raise ValueError("Incorrect input")

    result = []
    for i in range(len(s) - n + 1):
        result.append([int(j) for j in s[i:i + n]])
    return result
