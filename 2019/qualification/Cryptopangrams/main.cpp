#include <bits/stdc++.h>
#include <boost/multiprecision/cpp_int.hpp>

using namespace std;
namespace np = boost::multiprecision;

np::cpp_int gcd(np::cpp_int x, np::cpp_int y) {
    return y == 0 ? x : gcd(y, x % y);
}

string solve(np::cpp_int a[], int l) {
    np::cpp_int d, d0;
    for (int i = 0; i < l - 1; ++i) {
        if (a[i] == a[i + 1]) {
            continue;
        }
        d0 = gcd(a[i], a[i + 1]);
        while (i > 0) {
            d0 = a[i--] / d0;
        }
        break;
    }
    d = d0 = a[0] / d0;
    set<np::cpp_int> s { d };
    for (int i = 0; i < l; ++i) {
        d = a[i] / d;
        s.insert(d);
    }
    vector<np::cpp_int> v(s.begin(), s.end());
    string answer = "";
    d = d0;
    for (int i = 0; i < l + 1; ++i) {
        for (int j = 0, ll = v.size(); j < ll; ++j) {
            if (v[j] == d) {
                answer += 'A' + j;
                if (i < l) {
                    d = a[i] / d;
                }
                break;
            }
        }
    }
    return answer;
}

int main() {
    int t, n, l;
    cin >> t;
    for (int i = 0; i < t; ++i) {
        cin >> n >> l;
        np::cpp_int a[l];
        for (int j = 0; j < l; ++j) {
            string s;
            cin >> s;
            a[j] = np::cpp_int(s);
        }
        cout << "Case #" << i + 1 << ": " << solve(a, l) << endl;
    }
}
