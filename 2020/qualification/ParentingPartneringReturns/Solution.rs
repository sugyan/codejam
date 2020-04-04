use std::io::stdin;

fn read_usize() -> usize {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().parse().unwrap()
}

fn read_tuple_usize() -> (usize, usize) {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    let v: Vec<usize> = buf.trim().split(' ').map(|s| s.parse().unwrap()).collect();
    (v[0], v[1])
}

fn solve(v: Vec<(usize, usize)>) -> String {
    let mut v: Vec<(usize, usize, usize)> = (0..)
        .zip(v.iter())
        .map(|e| ((e.1).0, (e.1).1, e.0))
        .collect();
    v.sort();
    let mut answer: Vec<char> = vec!['J'; v.len()];
    let (mut i, mut t) = (0, 0);
    loop {
        while i < v.len() && t > v[i].0 {
            i += 1;
        }
        if i == v.len() {
            break;
        }
        answer[v[i].2] = 'C';
        t = v[i].1;
    }
    t = 0;
    for i in 0..v.len() {
        if answer[v[i].2] == 'C' {
            continue;
        }
        if v[i].0 < t {
            return "IMPOSSIBLE".to_string();
        }
        t = v[i].1;
    }
    answer.iter().collect::<String>()
}

fn main() {
    let t = read_usize();
    for case in 0..t {
        let n = read_usize();
        let mut v: Vec<(usize, usize)> = Vec::new();
        for _ in 0..n {
            v.push(read_tuple_usize());
        }
        println!("Case #{}: {}", case + 1, solve(v));
    }
}
