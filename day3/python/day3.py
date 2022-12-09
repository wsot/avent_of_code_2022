import itertools
import pathlib
import typing as t


def main() -> int:
    with pathlib.Path("input.txt").open() as f:
        input = [line.strip() for line in f]
    print(f"Part 1: {part_1(input)}")
    print(f"Part 2: {part_2(input)}")
    return 0


def part_1(input: t.Iterable[str]) -> int:
    """
    Let's call this _uglyquest_.
    """
    return sum(
        (
            val - 65 + 27 if (val - 65 < 26) else val - 96
            for val in (
                ord(
                    (
                        set(line[: len(line) // 2]).intersection(
                            set(line[len(line) // 2 :])
                        )
                    ).pop()
                )
                for line in input
            )
        )
    )


def part_2(input: t.Iterable[str]) -> int:
    """
    I feel like I could probably wrangle this into a generator
    """
    total = 0
    i = iter(input)
    while vals := list(itertools.islice(i, 3)):
        print(vals)
        val = ord(
            set(vals[0]).intersection(set(vals[1])).intersection(set(vals[2])).pop()
        )
        total += val - 65 + 27 if (val - 65 < 26) else val - 96

    return total


if __name__ == "__main__":

    exit(main())
