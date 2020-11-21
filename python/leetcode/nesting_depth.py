import argparse
import re
from itertools import accumulate


def get_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'input', type=str,
        help='Input string to be analyzed for nested parantheses'
    )
    return parser.parse_args().input


def get_depth_naive(input_str: str):
    depth = 0
    max_depth = 0
    for char in input_str:
        if char == '(':
            depth += 1
        elif char == ')':
            depth -= 1

        if depth < 0:
            raise Exception("Parantheses imbalanced")

        if depth > max_depth:
            max_depth = depth

    return max_depth

    return 0


def get_depth_compact(input_str: str):
    return  max(
            accumulate(
            map(
                lambda x: int(1 if x == "(" else -1 if x == ")" else 0),
                input_str)))


def main():
    input_str: str = get_arguments()
    print("Detected depths is {}".format(get_depth_naive(input_str)))
    print("Detected depths is {}".format(get_depth_compact(input_str)))



if __name__ == "__main__":
    main()
