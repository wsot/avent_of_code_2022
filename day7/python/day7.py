import pathlib
import typing as t

INCLUSION_THRESHOLD = 100000
REQUIRED_SPACE = 30000000
TOTAL_SPACE = 70000000


def main() -> int:
    check_assumptions(data())
    print(f"Part 1: {part1(data())}")
    target_size = REQUIRED_SPACE + part_2_get_total_size(data()) - TOTAL_SPACE
    print(f"Part 2: {part_2_find_target(data(), target_size)}")
    return 0


def data() -> t.Iterable[str]:
    with pathlib.Path("input.txt").open() as f:
        lineiter = iter(f)
        assert next(lineiter) == "$ cd /\n"  # We don't care about the first line
        for stripped_line in (line.strip() for line in lineiter):
            if not stripped_line:
                continue
            yield stripped_line


def part1(data: t.Iterable[str]) -> int:
    # Cumulates the size as directories are exited
    size_cumulator = 0

    size: list[int] = [0]

    for line in data:
        if line.startswith("$ cd .."):
            dir_size = size.pop(0)
            if dir_size <= INCLUSION_THRESHOLD:
                size_cumulator += dir_size
            size[0] += dir_size
        elif line.startswith("$ cd "):
            size.insert(0, 0)
        elif line.startswith(
            (
                "dir",
                "$",
            )
        ):
            continue
        else:
            size[0] += int(line.split(" ", 1)[0])

    return size_cumulator


def part_2_get_total_size(data: t.Iterable[str]) -> int:
    total = 0
    for line in data:
        if line.startswith(("$", "dir")):
            continue
        total += int(line.split(" ", 1)[0])

    return total


def part_2_find_target(data: t.Iterable[str], target_size: int) -> int:
    # Don't need to cumulate the totals this time
    # just need to record size of dir that has size as close
    # as possible to target size while being above it

    current_winner_size = 0

    size: list[int] = [0]

    for line in data:
        if line.startswith("$ cd .."):
            dir_size = size.pop(0)
            size[0] += dir_size
            if (current_winner_size == 0 and dir_size > target_size) or (
                target_size <= dir_size < current_winner_size
            ):
                current_winner_size = dir_size
        elif line.startswith("$ cd "):
            size.insert(0, 0)
        elif line.startswith(
            (
                "dir",
                "$",
            )
        ):
            continue
        else:
            size[0] += int(line.split(" ", 1)[0])

    return current_winner_size


def check_assumptions(data: t.Iterable[str]) -> None:
    # array of sets
    # when hit a `dir` add name to the set
    # when hit a `cd <name>` remove the name from the set,
    # and push a new thing onto the array
    # when doing a `cd ..` if the set at the last level
    # of the array is not empty, throw up
    dirs: list[list[str]] = [[]]
    for line in data:
        if line.startswith("dir "):
            dirs[0].append(line.split(" ", 1)[1])
        elif line.startswith("$ cd .."):
            assert not dirs.pop(0)
        elif line.startswith("$ cd "):
            assert dirs[0].pop(0) == line.split(" ", 3)[2]
            dirs.insert(0, [])


if __name__ == "__main__":
    exit(main())
