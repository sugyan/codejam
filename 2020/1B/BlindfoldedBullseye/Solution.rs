use std::io::stdin;

fn read_vec_i64() -> Vec<i64> {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().split(' ').map(|s| s.parse().unwrap()).collect()
}

fn read_string() -> String {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().to_string()
}

fn main() {
    let c: i64 = 10i64.pow(9);
    let v = read_vec_i64();
    for _ in 0..v[0] {
        let x = ({
            let (mut l, mut r) = (-c, -c + 100);
            while l < r {
                let m = (l + r) / 2;
                println!("{} {}", m, 0);
                let ret = read_string();
                if ret == "HIT" {
                    r = m - 1;
                } else {
                    l = m;
                }
            }
            l
        } + {
            let (mut l, mut r) = (c - 100, c);
            while l < r {
                let m = (l + r) / 2;
                println!("{} {}", m, 0);
                let ret = read_string();
                if ret == "HIT" {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            r
        }) / 2;
        let y = ({
            let (mut l, mut r) = (-c, -c + 100);
            while l < r {
                let m = (l + r) / 2;
                println!("{} {}", 0, m);
                let ret = read_string();
                if ret == "HIT" {
                    r = m - 1;
                } else {
                    l = m;
                }
            }
            l
        } + {
            let (mut l, mut r) = (c - 100, c);
            while l < r {
                let m = (l + r) / 2;
                println!("{} {}", 0, m);
                let ret = read_string();
                if ret == "HIT" {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            r
        }) / 2;
        'case: for xx in -5..6 {
            for yy in -5..6 {
                println!("{} {}", x + xx, y + yy);
                let ret = read_string();
                if ret == "CENTER" {
                    break 'case;
                }
            }
        }
    }
}
