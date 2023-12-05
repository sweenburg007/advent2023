from pathlib import Path
from num2words import num2words
from typing import List

NUM_STRS = [num2words(x) for x in range(1, 10)]
NUM_CHARS = [str(x) for x in range(1, 10)]
MAX_DIGIT_LEN = max([len(x) for x in NUM_STRS])

with Path("./data/input_1.txt").open("r") as f:
    lines = f.read().split("\n")
    while "" in lines:
        lines.remove("")

print(f"{len(lines)=}")
print(f"{NUM_STRS=}")
print(f"{NUM_CHARS=}")
print(f"{MAX_DIGIT_LEN=}")

total = 0
buf: List[str] = []

for idx, line in enumerate(lines):
    print("--------------------------------------------------------------------------------")
    print(f"{idx}: cur line = {line}")

    # setup for first and second digit
    first_digit = None
    second_digit = None

    # loop over all the characters in the line
    for char in line:
        tmp_digit = None

        # check for buffer too big
        if len(buf) == MAX_DIGIT_LEN:
            buf.pop(0)

        # check to see if character is numer ic
        if char.isnumeric():
            # print("found numeric char!")
            buf = []
            tmp_digit = char

        else:
            # add character to buffer
            buf.append(char)

            # check buffer against number strings
            for didx, digit_str in enumerate(NUM_STRS):
                if len(digit_str) <= len(buf):
                    start = len(buf) - len(digit_str)
                    cmp_str = "".join(buf[start:])
                    # print(f"\t{digit_str=},{cmp_str=}")

                    if cmp_str == digit_str:
                        # print("\tFOUND number string")
                        tmp_digit = NUM_CHARS[didx]
                        buf = [buf[-1]]
                        break

        # updated test digits
        if tmp_digit is not None:
            if first_digit is None:
                first_digit = tmp_digit
            second_digit = tmp_digit

        # print(f"{char=}: {first_digit=}, {second_digit=}, {buf=}")

    cur_val =int(str( first_digit ) + str( second_digit ))
    total += cur_val
    print(
        f"{first_digit=},{second_digit=}, {cur_val=}, {total=}"
    )
