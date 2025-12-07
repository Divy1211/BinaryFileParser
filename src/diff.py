from itertools import zip_longest

# ls1 = [
#     "test0",
#     "test1",
#     "test2",
#     "test3",
#     "test4",
#     "test5",
# ]
#
# ls2 = [
#     "test1",
#     "test3",
#     "test4.3",
#     "test4.4",
#     "test5",
#     "test6",
# ]
#
# ls3 = [
#     "test0",
#     "test2",
#     "test3",
#     "test4.7",
#     "test53",
#     "test6",
#     "test7",
# ]

ls1 = [
    "1",
    "2",
    "3",
]
ls2 = [
    "1",
    "2",
    "1",
]
ls3 = [
    "1",
    "2",
    "4",
    "2",
    "1",
]

def change(off: int, mod: tuple[int, str, str], ls: list[str]) -> int:
    i, c, e = mod
    if c == "!":
        ls[i-off] = e
        return off
    if c == "+":
        ls.insert(i-off, e)
        return off - 1

    ls.pop(i-off)
    return off + 1

def lcs(a: list[str] | str, b: list[str] | str) -> list[str] | str:
    n, m = len(a), len(b)

    dp = [[0] * (m + 1) for _ in range(n + 1)]

    for i in range(n):
        for j in range(m):
            if a[i] == b[j]:
                dp[i + 1][j + 1] = dp[i][j] + 1
            else:
                dp[i + 1][j + 1] = max(dp[i][j + 1], dp[i + 1][j])

    res = []
    i, j = n, m
    while i > 0 and j > 0:
        if a[i - 1] == b[j - 1]:
            res.append(a[i - 1])
            i -= 1
            j -= 1
        elif dp[i - 1][j] >= dp[i][j - 1]:
            i -= 1
        else:
            j -= 1

    return res[::-1]

def diff(ls1, ls2) -> list[str]:
    ls_lcs = lcs(ls1, ls2)
    start1, start2 = 0, 0

    result = []
    for txt in ls_lcs + [None]:
        if txt is None:
            idx1, idx2 = len(ls1), len(ls2)
        else:
            idx1, idx2 = (ls1.index(txt), ls2.index(txt))
        for ele1, ele2 in zip_longest(enumerate(ls1[start1:idx1], start1), ls2[start2:idx2]):
            if ele1 is None:
                result.append((idx1, "+", ele2))
            elif ele2 is None:
                i, ele1 = ele1
                result.append((i, "-", ele1))
            else:
                i, ele1 = ele1
                result.append((i, "!", ele2))
        start1, start2 = idx1 + 1, idx2 + 1
    return result

def merge(ls1, ls2, ls3) -> list[str]:
    d1 = diff(ls1, ls2)
    d2 = diff(ls1, ls3)

    print(d1, d2)

    merged = ls1[:]

    i1 = iter(d1)
    i2 = iter(d2)
    e1 = next(i1, None)
    e2 = next(i2, None)

    off = 0
    while e1 is not None or e2 is not None:
        if e2 is None or e1 is not None and e1[0] < e2[0]: 
            print("change", e1)
            off = change(off, e1, merged)
            e1 = next(i1, None)
        elif e1 is None or e2 is not None and e2[0] < e1[0]:
            print("change", e2)
            off = change(off, e2, merged)
            e2 = next(i2, None)
        else:
            if e1 == e2:
                print("change", e1)
                off = change(off, e1, merged)
            elif e1[1] in {"-", "!"} and e2[1] in {"-", "!"}:
                print("conflict:", e1, e2)
            else:
                e1, e2 = sorted([e1, e2], key = lambda x: x[1], reverse = True)
                print("change", e1)
                off = change(off, e1, merged)
                print("change", e2)
                off = change(off, e2, merged)
            e1 = next(i1, None)
            e2 = next(i2, None)

    return merged

# ls4 = merge(ls1, ls2, ls3)
#
# print(*ls4, sep = "\n")
