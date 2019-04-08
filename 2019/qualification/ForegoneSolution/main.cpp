#include <bits/stdc++.h>

using namespace std;

pair<string, string> solve(string n) {
    pair<string, string> answer("", "");
    for (int i = n.length() - 1; i >= 0; --i) {
        if (n[i] != '4') {
            answer.first = n[i] + answer.first;
            answer.second = '0' + answer.second;
        } else {
            answer.first = '3' + answer.first;
            answer.second = '1' + answer.second;
        }
    }
    while (answer.second[0] == '0') {
        answer.second.erase(answer.second.begin());
    }
    return answer;
}

int main() {
    int t;
    string n;
    cin >> t;
    for (int i = 0; i < t; ++i) {
        cin >> n;
        auto answer = solve(n);
        cout << "Case #" << i + 1 << ": ";
        cout << answer.first << " " << answer.second << endl;
    }
}
