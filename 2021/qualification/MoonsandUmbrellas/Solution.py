t = int(input())
for case in range(t):
    xs, ys, s = input().split(" ")
    x = int(xs)
    y = int(ys)
    s = s.replace(r"?", "")
    copyrights = 0
    for i in range(len(s) - 1):
        if s[i] == "C" and s[i + 1] == "J":
            copyrights += x
        if s[i] == "J" and s[i + 1] == "C":
            copyrights += y
    print(f"Case #{case + 1}: {copyrights}")
