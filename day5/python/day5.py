import typing as t


def main() -> int:
    input_iter = input()
    print(move_stacks_part_1(input_iter, build_stacks(input_iter)))
    input_iter = input()
    print(move_stacks_part_2(input_iter, build_stacks(input_iter)))
    return 0


def input() -> t.Iterable[str]:
    with open("input.txt", "rt") as f:
        for line in f:
            yield line


def build_stacks(input: t.Iterable[str]) -> list[list[str]]:
    input_iter = iter(input)
    line = next(input_iter)
    stack_count = len(line) // 4
    stacks: list[list[str]] = [[] for _ in range(stack_count)]

    while True:
        for idx in range(stack_count):
            char = line[idx * 4 + 1]
            if char != " ":
                stacks[idx].insert(0, char)
        line = next(input_iter)
        if line == "\n":
            break

    return stacks


def move_stacks_part_1(input: t.Iterable[str], stacks: list[list[str]]) -> str:
    for line in input:
        if not line.strip():
            break
        _, count, _, col_from, _, col_to = line.strip().split(" ")

        for _ in range(int(count)):
            stacks[int(col_to) - 1].append(stacks[int(col_from) - 1].pop())

    return "".join(col[-1] for col in stacks)


def move_stacks_part_2(input: t.Iterable[str], stacks: list[list[str]]) -> str:
    for line in input:
        if not line.strip():
            break
        _, count, _, col_from, _, col_to = line.strip().split(" ")

        stacks[int(col_to) - 1].extend(stacks[int(col_from) - 1][-int(count) :])
        del stacks[int(col_from) - 1][-int(count) :]

    return "".join(col[-1] for col in stacks)


if __name__ == "__main__":
    exit(main())
