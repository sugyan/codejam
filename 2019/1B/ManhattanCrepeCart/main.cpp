#include <bits/stdc++.h>

using namespace std;

int X[100005], Y[100005];

int main() {
    int t, p, q;
    cin >> t;
    for (int i = 0; i < t; ++i) {
        cin >> p >> q;
        for (int j = 0; j <= q; ++j) {
            X[j] = 0;
            Y[j] = 0;
        }
        for (int j = 0; j < p; ++j) {
            int x, y;
            char d;
            cin >> x >> y >> d;
            switch (d) {
            case 'N':
                ++Y[y + 1];
                break;
            case 'E':
                ++X[x + 1];
                break;
            case 'W':
                ++X[0];
                --X[x];
                break;
            case 'S':
                ++Y[0];
                --Y[y];
                break;
            }
        }
        pair<int, int> answer = { 0, 0 };
        for (int j = 1; j <= q; ++j) {
            X[j] += X[j - 1];
            Y[j] += Y[j - 1];
            if (X[j] > X[answer.first])  answer.first  = j;
            if (Y[j] > Y[answer.second]) answer.second = j;
        }
        cout << "Case #" << i + 1 << ": " << answer.first << " " << answer.second << endl;
    }
}
