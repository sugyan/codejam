#include <bits/stdc++.h>

using namespace std;

string solve(int n, string p) {
    string answer;
    for (int i = 0, l = p.length(); i < l; ++i) {
        answer += p[i] == 'S' ? 'E' : 'S';
    }
    return answer;
}

int main() {
    int t, n;
    string p;
    cin >> t;
    for (int i = 0; i < t; ++i) {
        cin >> n >> p;
        cout << "Case #" << i + 1 << ": " << solve(n, p) << endl;
    }
}
