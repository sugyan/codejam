#include <bits/stdc++.h>

using namespace std;

int main() {
    int t;
    cin >> t;
    for (int test = 0; test < t; ++test) {
        int a;
        cin >> a;
        vector<string> c(a);
        set<int> s;
        for (int i = 0; i < a; ++i) {
            cin >> c[i];
            s.insert(i);
        }
        string answer;
        int idx = 0;
        while (true) {
            if (answer.length() > 500) break;
            unordered_map<char, set<int>> um;
            for (int i : s) {
                char cc = c[i][idx % c[i].length()];
                if (um.find(cc) == um.end()) {
                    um[cc] = set<int>();
                }
                um[cc].insert(i);
            }
            if (um.size() == 0) {
                break;
            }
            if (um.size() == 1) {
                switch (um.begin()->first) {
                case 'R':
                    answer += 'P';
                    break;
                case 'P':
                    answer += 'S';
                    break;
                case 'S':
                    answer += 'R';
                    break;
                }
                break;
            }
            if (um.size() == 2) {
                pair<char, set<int>> p1 = *um.begin();
                um.erase(p1.first);
                pair<char, set<int>> p2 = *um.begin();
                if (p1.first == 'P') {
                    if (p2.first == 'R') {
                        answer += 'P';
                        s = p1.second;
                    } else {
                        answer += 'S';
                        s = p2.second;
                    }
                }
                if (p1.first == 'R') {
                    if (p2.first == 'P') {
                        answer += 'P';
                        s = p2.second;
                    } else {
                        answer += 'R';
                        s = p1.second;
                    }
                }
                if (p1. first == 'S') {
                    if (p2.first == 'R') {
                        answer += 'R';
                        s = p2.second;
                    } else {
                        answer += 'S';
                        s = p1.second;
                    }
                }
            }
            if (um.size() == 3) {
                answer = "IMPOSSIBLE";
                break;
            }
            ++idx;
        }
        cout << "Case #" << test + 1 << ": ";
        cout << answer << endl;
    }
}
