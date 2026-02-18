import random

TEST_TERMS = 100


def get_random_formula() -> str:
    numbers = [0]
    while any(round(n, 2) == 0 for n in numbers):
        numbers = [random.uniform(-10, 10) for _ in range(TEST_TERMS)]
    operators = [random.choice(["+", "-", "*", "/"]) for _ in range(TEST_TERMS - 1)]
    formula = " ".join(
        str(round(numbers[i], 2)) + " " + operators[i] for i in range(TEST_TERMS - 1)
    )
    formula += " " + str(round(numbers[TEST_TERMS - 1], 2))
    return formula + "\n"


if __name__ == "__main__":
    with open("input.txt", "w") as f:
        f.writelines(get_random_formula() for _ in range(10_000))
