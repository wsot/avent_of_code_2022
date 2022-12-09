# A = Rock, B = Paper, C = Scissors
LATTER_WINS = {
    ("A", "B"),
    ("B", "C"),
    (
        "C",
        "A",
    ),
}
SUBSTITUTIONS = {"A": "X", "B": "Y", "C": "Z"}
SCORES = {"A": 1, "B": 2, "C": 3}
WIN_SCORE = 6


def verbose_part_1(lines) -> int:
    my_score: int = 0

    outcome_map = {}
    for theirs in ("A", "B", "C"):
        for mine in ("A", "B", "C"):
            if theirs == mine:
                outcome_map[f"{theirs} {SUBSTITUTIONS[mine]}"] = (
                    SCORES[theirs] + 3,
                    SCORES[mine] + 3,
                )
            elif (theirs, mine) in LATTER_WINS:
                outcome_map[f"{theirs} {SUBSTITUTIONS[mine]}"] = (
                    SCORES[theirs],
                    SCORES[mine] + WIN_SCORE,
                )
            else:
                outcome_map[f"{theirs} {SUBSTITUTIONS[mine]}"] = (
                    SCORES[theirs] + WIN_SCORE,
                    SCORES[mine],
                )

    for line in lines:
        line = line.strip()
        outcome = outcome_map[line]
        my_score += outcome[1]

    return my_score


def verbose_part_2(lines) -> int:
    my_score: int = 0

    outcome_map = {}
    for theirs in ("A", "B", "C"):
        for outcome in ("X", "Y", "Z"):
            if outcome == "X":
                score = 1 + (SCORES[theirs] - 2) % 3
            elif outcome == "Y":
                score = 4 + (SCORES[theirs] - 1) % 3
            elif outcome == "Z":
                score = 7 + (SCORES[theirs]) % 3
            else:
                raise Exception("wha?")

            outcome_map[f"{theirs} {outcome}"] = score

    for line in lines:
        line = line.strip()
        my_score += outcome_map[line]

    return my_score


def terse_part_1(lines) -> int:
    return sum(
        1 + x[1] + (3 if x[0] == x[1] else (6 if (((x[1] - x[0]) % 3) == 1) else 0))
        for x in (
            (ord(inner_line[0]) - ord("A"), ord(inner_line[2]) - ord("X"))
            for inner_line in (line[:-1] for line in lines)
        )
    )


def terse_part_2(lines) -> int:
    results = (
        (ord(x[0]) - ord("A"), ord(x[1]) - ord("X"))
        for x in (line.strip().split(" ") for line in lines)
    )
    return sum(1 + ((theirs + (wld - 1)) % 3) + (3 * wld) for theirs, wld in results)


def main() -> int:
    with open("input.txt", "rt") as f:
        lines = f.readlines()

    print(terse_part_1(lines))
    print(verbose_part_1(lines))
    print("part 2")
    print(verbose_part_2(lines))
    print(terse_part_2(lines))
    return 0


if __name__ == "__main__":
    exit(main())
