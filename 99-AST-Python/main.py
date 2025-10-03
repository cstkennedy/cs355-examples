import ast
import sys


def main() -> None:
    source_filename = sys.argv[1]

    with open(source_filename, "r") as source_file:
        code = source_file.read()

    print(code)

    code_ast = ast.parse(code, type_comments=True)
    print(ast.dump(code_ast, indent=2))


if __name__ == "__main__":
    main()
