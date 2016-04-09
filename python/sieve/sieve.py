def sieve(n):
    numbers = range(2, n + 1)
    pos = 0
    while pos != len(numbers):
        prime = numbers[pos]
        numbers = [i for i in numbers if i == prime or i % prime != 0]
        pos += 1

    return numbers
