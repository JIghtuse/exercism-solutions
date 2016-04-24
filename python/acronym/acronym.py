def abbreviate(s):
    prev = ''
    abbr = ''
    for c in s:
        if prev in " -" or prev.islower() and c.isupper():
            abbr += c.upper()
        prev = c
    return abbr
