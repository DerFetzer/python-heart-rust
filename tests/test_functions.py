import python_heart_rust

import pytest


@pytest.fixture(scope="module")
def bible():
    return python_heart_rust.get_and_open_bible()


def test_python(benchmark, bible):
    word_counter = benchmark(python_heart_rust.get_word_counter_dict_py, bible)
    assert word_counter["der"] == 18727

def test_rust(benchmark, bible):
    word_counter = benchmark(python_heart_rust.get_word_counter_dict_rs, bible)
    assert word_counter["der"] == 18727

def test_rust_parallel(benchmark, bible):
    word_counter = benchmark(python_heart_rust.get_word_counter_dict_parallel_rs, bible)
    assert word_counter["der"] == 18727
