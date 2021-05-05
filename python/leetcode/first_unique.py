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

    for pos, char in enumerate(input_str):
        if char in char_dict:
            char_dict[char][1] += 1
        else:
            char_dict[char] = [pos, 1]

    for ch, info in char_dict.items():
        if info[1] == 1:
            print("Found at {}: {}".format(info[0], ch))
            return info[0]
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
    for idx, count in enumerate(char_array):
        if count > 0:
            min_val = min(count - 1, min_val)

    return min_val if min_val != 0x7fffffff else -1


def main():
    print(find_unique_with_hash(get_arguments()))
    print(find_unique_with_array(get_arguments()))


if __name__ == "__main__":
    main()
