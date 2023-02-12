#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time

LETTERS = "etaonrishdlfcmugypwbvkjxzq"


def process(current, words):
    if len(current) >= 25:
        return current
    letter = [c for c in LETTERS if current.find(c) < 0][0]
    first, second = [], []
    for w in words:
        (first if w.find(letter) >= 0 else second).append(w)
    for word in first:
        found = process(current + word, [w for w in second if not share_letters(word, w)])
        if found:
            return found
    return None


def share_letters(w1, w2):
    for ch in w1:
        if w2.find(ch) >= 0:
            return True
    return False


def has_no_dup(s):
    for ch in s:
        if s.count(ch) > 1:
            return False
    return True


if __name__ == '__main__':
    start = time.time()
    with open("../words.txt") as file:
        lines = [line.rstrip() for line in file]
    words = [w for w in lines if has_no_dup(w)]
    result = process("", words)
    print(f"RESULT {result} found in {time.time() - start} s")
