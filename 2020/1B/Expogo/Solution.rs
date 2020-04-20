use std::io::stdin;

fn read_usize() -> usize {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().parse().unwrap()
}

fn read_tuple_i64() -> (i64, i64) {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    let v: Vec<i64> = buf.trim().split(' ').map(|s| s.parse().unwrap()).collect();
    (v[0], v[1])
}

fn solve(xy: (i64, i64)) -> String {
    let mut xy = xy;
    let mut answer = String::new();
    let mut n = {
        let mut n: i64 = 1;
        while n * 2 < xy.0.abs() + xy.1.abs() {
            n <<= 1;
        }
        n
    };
    while n > 0 {
        if xy.0.abs() > xy.1.abs() {
            xy.0 += if xy.0 > 0 {
                answer.push('E');
                -n
            } else {
                answer.push('W');
                n
            };
        } else {
            xy.1 += if xy.1 > 0 {
                answer.push('N');
                -n
            } else {
                answer.push('S');
                n
            };
        }
        n >>= 1;
    }
    if xy == (0, 0) {
        answer.chars().rev().collect()
    } else {
        "IMPOSSIBLE".to_string()
    }
}

fn main() {
    let t = read_usize();
    for case in 0..t {
        let xy = read_tuple_i64();
        println!("Case #{}: {}", case + 1, solve(xy));
    }
}
