import argparse


def get_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        'input', type=str,
        help="String to analyze"
    )
    return parser.parse_args().input


def find_unique_with_hash(input_str: str):
    char_dict: dict = {}

    pos = 0
    for char in input_str:
        if char in char_dict:
            char_dict[char][1] += 1
        else:
            char_dict[char] = [pos, 1]
        pos += 1

    for idx, ch in enumerate(char_dict):
        if char_dict[ch][1] == 1:
            print("Found at {}: {}".format(char_dict[ch][0], ch))
            return char_dict[ch][0]
    print("No unique character")
    return -1


def find_unique_with_array(input_str: str):
    char_array = [0] * 26

    for idx, char in enumerate(input_str):
        a_idx = ord(char) - ord('a')
        val = char_array[a_idx]
        if val != 0:
            char_array[a_idx] = -1
        else:
            char_array[a_idx] = idx + 1

    min_val = 0x7fffffff
    for idx in char_array:
        if char_array[idx] > 0:
            min_val = min(char_array[idx] - 1, min_val)

    return min_val if min_val != 0x7fffffff else -1


def main():
    print(find_unique_with_hash(get_arguments()))
    print(find_unique_with_array(get_arguments()))


if __name__ == "__main__":
    main()
