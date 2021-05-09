import argparse


def get_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'input', type=int,
        nargs='+',
        help="Sorted number array in ascending order (may contain duplicates)"
    )
    return parser.parse_args().input


def remove_duplicate(input_array: []):

    cur_pos = 0
    input_size = len(input_array)

    for item in input_array:
        # Check if index lookahead is a valid operation
        if cur_pos < input_size - 2:
            # Just increment for ascending values (no duplicate)
            if input_array[cur_pos] < input_array[cur_pos + 1]:
                cur_pos = cur_pos + 1
            # Otherwise, check if the item received by loop is greater than
            # current position
            elif item > input_array[cur_pos]:
                input_array[cur_pos + 1] = item
                cur_pos = cur_pos + 1

    # Return slice of input array containing no duplicates
    return input_array[0 : cur_pos + 1]


def main():
    print("Input: {}\n".format(get_arguments()))
    print("Output: {}\n".format(remove_duplicate(get_arguments())))


if __name__ == "__main__":
    main()
