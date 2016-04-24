def sum_of_multiples(n, numbers):
    multiples = set()
    for number in numbers:
        if number:
            for multiple in range(number, n, number):
                multiples.add(multiple)
    return sum(multiples)
