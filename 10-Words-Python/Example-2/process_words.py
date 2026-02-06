"""
Demonstrate generator expressions in Python using...

  1. a generator function with filter function calls
  2. a generator with conditions
"""

import sys
from typing import Generator, TextIO

from qwlist import Lazy


def contains_chars(word: str, banned: list[str] | None = None) -> bool:
    """
    Determine if a "word" (more aptly "token") contains a banned symbol.

    If no banned list is provided... default to ['.' and '+']

    Returns:
        True if the "word" contains a banned symbol and false otherwise
    """

    if not banned:
        banned = [".", "+"]

    for char in banned:
        if char in word:
            return True

    return False


def main() -> None:
    word_fname = sys.argv[1]

    with open(word_fname, "r") as word_file:
        banned = [".", "+"]

        words = (
            Lazy(word_file)
            .map(lambda word: word.strip())
            .filter(lambda word: word)
            .filter(lambda word: len(word) < 5)
            .filter(lambda word: Lazy(banned).all(lambda symbol: symbol not in word))
        )

        for word in words:
            print(f"|{word}|")


if __name__ == "__main__":
    main()
