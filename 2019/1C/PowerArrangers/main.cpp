#include <bits/stdc++.h>

using namespace std;

int main() {
    int t, f;
    cin >> t >> f;
    cerr << t << " " << f << endl;
    for (int test = 0; test < t; ++test) {
        string answer = "";
        char c;
        vector<int> v1[5], v2[5], v3[5];
        vector<int> vv;
        for (int i = 0; i < f; ++i) {
            if (i < 119) {
                cout << i * 5 + 1 << endl;
                cin >> c;
                v1[c - 'A'].push_back(i);
            } else if (i < 119 + 23) {
                if (i == 119) {
                    for (int i = 0; i < 5; ++i) {
                        if (v1[i].size() == 23) {
                            answer += 'A' + i;
                            vv = v1[i];
                        }
                    }
                }
                cout << (vv[i - 119]) * 5 + 2 << endl;
                cin >> c;
                v2[c - 'A'].push_back(vv[i - 119]);
            } else if (i < 119 + 23 + 5) {
                if (i == 119 + 23) {
                    for (int i = 0; i < 5; ++i) {
                        if (v2[i].size() == 5) {
                            answer += 'A' + i;
                            vv = v2[i];
                        }
                    }
                }
                cout << (vv[i - 119 - 23]) * 5 + 3 << endl;
                cin >> c;
                v3[c - 'A'].push_back(vv[i - 119 - 23]);
            } else if (i < 119 + 23 + 5 + 1) {
                if (i == 119 + 23 + 5) {
                    for (int i = 0; i < 5; ++i) {
                        if (v3[i].size() == 1) {
                            answer += 'A' + i;
                            vv = v3[i];
                        }
                    }
                }
                cout << (vv[i - 119 - 23 - 5]) * 5 + 4 << endl;
                cin >> c;
                int ii[5] = { 0 };
                for (char chr : answer) ++ii[chr - 'A'];
                ++ii[c - 'A'];
                for (int i = 0; i < 5; ++i) {
                    if (ii[i] == 0) {
                        answer += 'A' + i;
                        answer += c;
                    }
                }
            } else {
                cout << 1 << endl;
                cin >> c;
            }
        }
        cout << answer << endl;
        char result;
        cin >> result;
        if (result == 'N') {
            break;
        }
    }
}
