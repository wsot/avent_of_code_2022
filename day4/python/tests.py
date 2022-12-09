import pytest

from . import day4


@pytest.mark.parametrize(
    "input,expected_result,label",
    (
        ("1-2,3-4", 0, "No overlap"),
        ("3-4,1-2", 0, "No overlap rev"),
        ("1-2,2-3", 0, "Ends touch"),
        ("2-3,1-2", 0, "Ends touch rev"),
        ("1-3,2-4", 0, "Overlap, not contained"),
        ("2-4,1-3", 0, "Overlap, not contained, rev"),
        ("1-2,1-2", 1, "Identical"),
        ("1-3,2-3", 1, "Overlap, end touch, contained"),
        ("2-3,1-3", 1, "Overlap, end touch, contained rev"),
        ("1-3,1-2", 1, "Overlap, start touch, contained"),
        ("1-2,1-3", 1, "Overlap, start touch, contained rev"),
        ("1-4,2-3", 1, "Fully contained"),
        ("2-3,1-4", 1, "Fully contained rev"),
    ),
)
def test_ranges(input, expected_result, label):
    assert day4.part_1([input]) == expected_result, label
