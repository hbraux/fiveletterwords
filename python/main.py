#!/usr/bin/env python3
# -*- coding: utf-8 -*-
import time

LETTERS = "etaonrishdlfcmugypwbvkjxzq"


def process(current, words):
    if len(current) >= 20:
        return current
    letter = [c for c in LETTERS if current.find(c) < 0][0]
    first, second = [], []
    for w in words:
        (first if w.find(letter) >= 0 else second).append(w)
    for word in first:
        filtered = [w for w in second if not share_letters(word, w)]
        found = process(current, filtered)
        if found:
            return found
    return ""


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
    lines = open("../words.txt").readlines()
    words = [w for w in lines if has_no_dup(w)]
    result = process("", words)
    print(f"RESULT {result} found in {time.time() - start} s")
