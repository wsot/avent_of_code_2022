import typing as t


def main() -> int:
    print(f"Part 1: {part_1(input())}")
    print(f"Part 2: {part_2(input())}")
    return 0


def input() -> t.Iterable[str]:
    with open("input.txt", "rt") as f:
        yield from (line.strip() for line in f)


def part_1(input: t.Iterable[str]) -> int:
    total = 0

    for line in input:
        if not line:
            continue
        first, second = line.split(",")
        ordered_pairs = sorted(
            tuple(
                (
                    tuple(int(x) for x in first.split("-")),
                    tuple(int(x) for x in second.split("-")),
                )
            ),
            key=lambda x: (x[0], -x[1]),
        )
        if ordered_pairs[1][1] <= ordered_pairs[0][1]:
            total += 1

    return total


def part_2(input: t.Iterable[str]) -> int:
    total = 0

    for line in input:
        if not line:
            continue
        first, second = line.split(",")
        ordered_pairs = sorted(
            tuple(
                (
                    tuple(int(x) for x in first.split("-")),
                    tuple(int(x) for x in second.split("-")),
                )
            ),
        )
        if ordered_pairs[1][0] <= ordered_pairs[0][1]:
            total += 1

    return total


if __name__ == "__main__":
    exit(main())
