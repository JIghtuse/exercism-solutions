from collections import OrderedDict

NUMBERS = {0: "zero",
           1: "one",
           2: "two",
           3: "three",
           4: "four",
           5: "five",
           6: "six",
           7: "seven",
           8: "eight",
           9: "nine",
           10: "ten",
           11: "eleven",
           12: "twelve",
           13: "thirteen",
           14: "fourteen",
           15: "fifteen",
           16: "sixteen",
           17: "seventeen",
           18: "eighteen",
           19: "nineteen",
           20: "twenty",
           30: "thirty",
           40: "forty",
           50: "fifty",
           60: "sixty",
           70: "seventy",
           80: "eighty",
           90: "ninety",
           100: "hundred"}


TEN_POWERS = OrderedDict([(1000000000, "billion"),
                          (1000000, "million"),
                          (1000, "thousand")])


def say_up_to_hundreds(n, need_and=False):
    s = ""

    hundreds = n // 100
    if hundreds:
        if need_and:
            s += " "
        s += "{} {}".format(NUMBERS[hundreds], NUMBERS[100])
        n %= 100

    if (need_and or s) and n:
        s += " and "

    if n > 19:
        tens = n // 10
        s += NUMBERS[tens * 10]
        n %= 10
        if n:
            s += "-"
    if n > 0:
        s += NUMBERS[n]
    return s


def say(n):
    if n < 0 or n >= 1e12:
        raise AttributeError

    if n == 0:
        return NUMBERS[n]

    s = ""
    while n >= 1000:
        for ten_power in TEN_POWERS:
            power = n // ten_power
            if power:
                s = ' '.join([s, say_up_to_hundreds(power), TEN_POWERS[ten_power]])
                n %= ten_power
    s += say_up_to_hundreds(n, s)
    return s.strip()
