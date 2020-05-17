use std::io::stdin;

fn read_usize() -> usize {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().parse().unwrap()
}

fn read_tuple_u64() -> (u64, u64) {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    let v: Vec<u64> = buf.trim().split(' ').map(|s| s.parse().unwrap()).collect();
    (v[0], v[1])
}

fn solve(lr: (u64, u64)) -> (u64, u64, u64) {
    let mut lr = lr;
    let mut answer = 1;
    loop {
        if lr.1 > lr.0 {
            if answer > lr.1 {
                return (answer - 1, lr.0, lr.1);
            }
            lr.1 -= answer;
        } else {
            if answer > lr.0 {
                return (answer - 1, lr.0, lr.1);
            }
            lr.0 -= answer;
        }
        answer += 1;
    }
}

fn main() {
    let t = read_usize();
    for case in 0..t {
        let lr = read_tuple_u64();
        let answer = solve(lr);
        println!("Case #{}: {} {} {}", case + 1, answer.0, answer.1, answer.2);
    }
}
