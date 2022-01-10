def multiplier(factor: int):
    return lambda x: x * factor


multiply_by_two = multiplier(2)

for i in range(2, 8):
    print(f"The double of {i} is {multiply_by_two(i)}.")
