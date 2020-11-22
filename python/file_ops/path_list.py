import os
import re
import argparse


def main():
    arglist = get_arguments()
    scan_dir(arglist)
    walk_dir(arglist)


def get_arguments():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        '--list', '-l', type=str,
        nargs='+',
        help="Provide number of files/paths to analyze"
    )
    return parser.parse_args().list


def scan_dir(arglist: []):
    for arg in arglist:
        with os.scandir(arg) as it:
            # Filter out files and directories starting with '.'
            it = list(filter(lambda x: re.match(r"[^\.]", x.name), it))
            for entry in it:
                if entry.is_file():
                    # Get filename with absolute path
                    print(os.path.abspath(entry))
                    # Get file extension (separated by '.')
                    print(entry.name.split('.')[1])


def walk_dir(arglist: []):
    for arg in arglist:
        for root, dirs, files in os.walk(arg):
            files = list(filter(lambda x: re.match(r"[^\.]", x), files))
            # Remove all directories
            del dirs[:]
            for file in files:
                print(os.path.abspath(root + "/" + file))
                print(file.split('.')[1])


if __name__ == "__main__":
    main()
