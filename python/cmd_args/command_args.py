import argparse

def getArguments():
    parser = argparse.ArgumentParser(
        description='''Parse list of files in path.'''
    )
    parser.add_argument(
        '--path', '-p', type=str,
        help='Path to files',
        required=True,
        default='.'
    )
    parser.add_argument(
        '--type', '-t', type=str,
        choices=['dir', 'file']
    )
    parser.add_argument(
        '--file-list', '-l', nargs='+', type=str,
        dest='list',
        help='List of files'
    )
    return parser.parse_args()


if __name__ == "__main__":
    args = getArguments()
    print(args)
    print(args.path)
    print(args.list)

