use std::cmp::Ordering;
use std::io::stdin;

fn read_usize() -> usize {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().parse().unwrap()
}

fn read_string() -> String {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn solve(s: String) -> String {
    let v: Vec<usize> = s.chars().map(|c| (c as u8 - b'0') as usize).collect();
    let mut answer: String = String::new();
    let mut prev = 0;
    for n in v.iter() {
        match n.cmp(&prev) {
            Ordering::Greater => {
                for _ in 0..*n - prev {
                    answer.push('(');
                }
            }
            Ordering::Less => {
                for _ in 0..prev - *n {
                    answer.push(')');
                }
            }
            Ordering::Equal => {}
        }
        answer += &n.to_string();
        prev = *n;
    }
    for _ in 0..prev {
        answer.push(')');
    }
    answer
}

fn main() {
    let t = read_usize();
    for case in 0..t {
        let s = read_string();
        println!("Case #{}: {}", case + 1, solve(s));
    }
}
