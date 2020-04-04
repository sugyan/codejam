use std::io::stdin;

fn read_usize() -> usize {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().parse().unwrap()
}

fn read_vec_i32() -> Vec<i32> {
    let mut buf: String = String::new();
    stdin().read_line(&mut buf).ok();
    buf.trim().split(' ').map(|s| s.parse().unwrap()).collect()
}

fn main() {
    let t = read_usize();
    for case in 0..t {
        let n = read_usize();
        let mut matrix: Vec<Vec<i32>> = Vec::new();
        for i in 0..n {
            matrix.push(read_vec_i32());
        }
        let (mut k, mut c, mut r) = (0, 0, 0);
        for i in 0..n {
            k += matrix[i][i];
        }
        for i in 0..n {
            let mut v: Vec<usize> = vec![0; n];
            for j in 0..n {
                let m = matrix[i][j] as usize - 1;
                if v[m] > 0 {
                    c += 1;
                    break;
                }
                v[m] += 1;
            }
        }
        for i in 0..n {
            let mut v: Vec<usize> = vec![0; n];
            for j in 0..n {
                let m = matrix[j][i] as usize - 1;
                if v[m] > 0 {
                    r += 1;
                    break;
                }
                v[m] += 1;
            }
        }
        println!("Case #{}: {} {} {}", case + 1, k, c, r);
    }
}
