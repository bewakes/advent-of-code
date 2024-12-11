from operator import add, mul
from functools import reduce

def apply(ops, vals):
    return reduce(
        lambda a, pair: pair[0](a, pair[1]),
        zip(ops, vals[1:]),
        vals[0],
    )

def is_valid_entry(val, items):
    num_ops = len(items) - 1
    for i in range(0, 2**num_ops):
        operators = [(add if (i >> n) % 2 == 0 else mul) for n in range(num_ops)]
        if apply(operators, items) == val:
            return True
    return False


def main():
    with open('data/input') as f:
        sum = 0
        for line in f.readlines():
            test, vals = line.split(':', 1)
            test = int(test)
            vals = [int(x) for x in vals.strip().split()]
            if is_valid_entry(test, vals):
                sum+= test
        print(sum)


def test():
    test = 292
    vals = [11, 6, 16, 20]
    assert is_valid_entry(test, vals)

    test = 7290
    vals = [6, 8, 6, 15]
    assert not is_valid_entry(test, vals)


if __name__ == '__main__':
    test()
    main()
