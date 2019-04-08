def gcd(x, y):
    return x if y == 0 else gcd(y, x % y)


def solve(a):
    d0 = 0
    for i in range(len(a) - 1):
        if a[i] != a[i + 1]:
            d0 = gcd(a[i], a[i + 1])
            for j in range(i):
                d0 = a[i - j] // d0
            break
    d = d0 = a[0] // d0
    s = set([d])
    for e in a:
        d = e // d
        s.add(d)
    v = sorted(s)
    d = d0
    answer = ''
    for i in range(len(a) + 1):
        idx = v.index(d)
        answer += chr(idx + ord('A'))
        if i < len(a):
            d = a[i] // d
    return answer


t = int(input())
for i in range(t):
    _, l = [int(x) for x in input().strip().split(' ')]
    a = [int(x) for x in input().strip().split(' ', l)]
    print('Case #{}: {}'.format(i + 1, solve(a)))
