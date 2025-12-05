from pathlib import Path
from typing import Tuple

inputs = Path("inputs/day-01.txt").read_text()


def solution() -> Tuple[int, int]:
    calories: list[int] = list(
        map(
            lambda elf: sum(map(lambda i: int(i.strip()), elf.splitlines())),
            inputs.split("\n\n"),
        )
    )

    calories.sort(reverse=True)

    return calories[0], sum(calories[0:3])


part1, part2 = solution()
print(f"Part 1: {part1} / Part 2: {part2}")
