t = int(input())
for case in range(t):
    n, c = map(int, input().split())
    if c < n - 1 or n * (n + 1) // 2 - 1 < c:
        print(f"Case #{case + 1}: IMPOSSIBLE")
    else:
        answer = list(range(1, n + 1))
        d = c - n + 1
        s = []
        i = n - 1
        while d > 0:
            if d >= i:
                d -= i
                s.append(i)
            i -= 1
        for j in reversed(s):
            answer = answer[: -j - 1] + list(reversed(answer[-j - 1 :]))
        print(f"Case #{case + 1}: {' '.join(map(str, answer))}")
