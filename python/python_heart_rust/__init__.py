from .python_heart_rust import *

import collections
import requests
import os.path

__doc__ = python_heart_rust.__doc__
if hasattr(python_heart_rust, "__all__"):
    __all__ = python_heart_rust.__all__

BIBLE_URL: str = "https://www.sermon-online.com/download/german/MartinLuther-1912/Martin_Luther_Uebersetzung_1912.txt"
BIBLE_FILE: str = "bible.txt"


def get_and_open_bible() -> list[str]:
    if not os.path.exists(BIBLE_FILE):
        bible_text = requests.get(BIBLE_URL).content
        with open(BIBLE_FILE, "wb") as f:
            f.write(bible_text)

    with open(BIBLE_FILE, "r", encoding="latin-1") as f:
        lines = f.readlines()
        return list(map(__extract_text_from_line, lines[1:]))


def get_word_counter_dict_py(lines: list[str]) -> dict[str, int]:
    word_counter = collections.defaultdict(int)
    for line in lines:
        filtered_line = "".join(
            filter(lambda c: str.isalpha(c) or str.isspace(c), line)
        ).lower()
        for word in filtered_line.split():
            word_counter[word] += 1
    return word_counter


def __extract_text_from_line(line: str) -> str:
    try:
        return line.split("§", 1)[1].split(" ", 1)[1].split(" - ")[0].strip()
    except IndexError:
        return ""
