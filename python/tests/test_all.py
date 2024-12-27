import pytest
import z2edit


def test_sum_as_string():
    assert z2edit.sum_as_string(1, 1) == "2"
