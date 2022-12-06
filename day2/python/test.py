import pytest

from . import main


@pytest.mark.parametrize("fn", (main.terse_part_2, main.verbose_part_2))
class TestyTestTest:
    def test_example(self, fn):
        lines = ("A Y\n", "B X\n", "C Z\n")
        assert fn(lines) == 12

    def test_harder(self, fn):
        lines = ("A Y\n", "B X\n", "C Z\n", "C Y\n", "C X\n", "B Z\n")
        assert fn(lines) == 29

    @pytest.mark.parametrize(
        "indata,score",
        (
            ("A X\n", 3),
            ("B X\n", 1),
            ("C X\n", 2),
            ("A Y\n", 4),
            ("B Y\n", 5),
            ("C Y\n", 6),
            ("A Z\n", 8),
            ("B Z\n", 9),
            ("C Z\n", 7),
        ),
    )
    def test_permutations(self, fn, indata, score):
        assert fn((indata,)) == score
