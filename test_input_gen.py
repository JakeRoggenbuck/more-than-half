# #!/usr/bin/env python3
# Copyright (c) 2023 Jake Roggenbuck
import random
import math

SMALLEST_VALUE = 0
LARGEST_VALUE = 99

import argparse

def parser():
    parse = argparse.ArgumentParser()

    parse.add_argument("-c", "--count", help="Amount of input values you want", required=True)
    return parse.parse_args()

def main():
    args = parser()
    count = int(args.count)

    to_populate = random.randint(SMALLEST_VALUE, LARGEST_VALUE)
    half_size = math.floor((count - 1) / 2.0)

    array = [to_populate]*(half_size + 2)
    for x in range(half_size):
        value = random.randint(SMALLEST_VALUE, LARGEST_VALUE)
        array.append(value)

    random.shuffle(array)

    with open("out.txt", "w") as file:
        # Write the size in the header
        file.write(str(2 * half_size + 2) + "; <- amount of values\n")
        for x in array:
            file.write(str(x) + "\n")

if __name__ == "__main__":
    main()
