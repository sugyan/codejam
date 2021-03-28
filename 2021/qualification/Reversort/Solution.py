t = int(input())
for case in range(t):
    n = int(input())
    ls = list(map(int, input().split()))
    cost = 0
    for i in range(n - 1):
        pos = ls.index(min(ls[i:]))
        cost += pos - i + 1
        ls = ls[:i] + list(reversed(ls[i : pos + 1])) + ls[pos + 1 :]
    print(f"Case #{case + 1}: {cost}")
