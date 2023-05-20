use std::{env, fmt::Display, fs::File, io::Read, path};

fn max(a: i8, b: u8) -> i8 {
    let bi: i8 = b.try_into().unwrap();

    if a > bi {
        a
    } else {
        bi
    }
}

fn _print_mat<T: Display>(mat: &[Vec<T>]) {
    mat.iter().for_each(|row| {
        row.iter().for_each(|c| print!("{:02} ", c));
        println!();
    });
}

fn calculate_visibility(mat: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let r_len = mat.len();
    let c_len = mat[0].len();

    let mut visibility = vec![vec![0u8; r_len]; c_len];

    let mut rf_max = vec![-1_i8; r_len];
    let mut rb_max = vec![-1_i8; r_len];

    let mut cf_max = vec![-1_i8; c_len];
    let mut cb_max = vec![-1_i8; c_len];

    for r in 0..r_len {
        let rb = r_len - r - 1;
        for c in 0..c_len {
            let cb = c_len - c - 1;
            if rf_max[r] < mat[r][c].try_into().unwrap() {
                visibility[r][c] |= 0b00000001;
            } //visible from left  1
            if rb_max[r] < mat[r][cb].try_into().unwrap() {
                visibility[r][cb] |= 0b00000010;
            } //visible from right 2
            if cf_max[c] < mat[r][c].try_into().unwrap() {
                visibility[r][c] |= 0b00000100;
            } //visible from top   4
            if cb_max[c] < mat[rb][c].try_into().unwrap() {
                visibility[rb][c] |= 0b00001000;
            } //visible from bottom 8

            rf_max[r] = max(rf_max[r], mat[r][c]);
            rb_max[r] = max(rb_max[r], mat[r][cb]);

            cf_max[c] = max(cf_max[c], mat[r][c]);
            cb_max[c] = max(cb_max[c], mat[rb][c]);
        }
    }

    visibility
}

fn calculate_scenic_score(mat: &Vec<Vec<u8>>) -> Vec<Vec<u32>> {
    let r_len = mat.len();
    let c_len = mat[0].len();

    let mut scenic_score = vec![vec![0u32; r_len]; c_len];

    for r in 0..r_len {
        for c in 0..c_len {
            let mut lt = 0u32;
            let mut rt = 0u32;
            let mut t = 0u32;
            let mut b = 0u32;

            let mut ri: i64 = r as i64 - 1;
            while ri >= 0 {
                t += 1;
                if mat[ri as usize][c] >= mat[r][c] {
                    break;
                }
                ri -= 1;
            }

            let mut ri = r + 1;
            while ri < r_len {
                b += 1;
                if mat[ri][c] >= mat[r][c] {
                    break;
                }
                ri += 1;
            }

            let mut ci: i64 = c as i64 - 1;
            while ci >= 0 {
                lt += 1;
                if mat[r][ci as usize] >= mat[r][c] {
                    break;
                }
                ci -= 1;
            }

            let mut ci = c + 1;
            while ci < r_len {
                rt += 1;
                if mat[r][ci] >= mat[r][c] {
                    break;
                }
                ci += 1;
            }
            scenic_score[r][c] = lt * rt * t * b;
        }
    }

    scenic_score
}

fn soln1(mat: &Vec<Vec<u8>>) -> usize {
    let visibility = calculate_visibility(mat);

    return visibility.iter().flatten().filter(|x| **x > 0u8).count();
}

fn soln2(mat: &Vec<Vec<u8>>) -> u32 {
    let scenic_score = calculate_scenic_score(mat);

    return *scenic_score.iter().flatten().max().unwrap();
}

fn main() {
    let arg_v: Vec<String> = env::args().collect();

    let filepath = path::Path::new(&arg_v[1]);

    let mut infile = File::open(filepath).unwrap();
    let mut lines = String::new();

    infile.read_to_string(&mut lines).unwrap();
    let mat: Vec<Vec<u8>> = lines
        .split('\n')
        .map(|s| s.bytes().map(|b| b - b'0').collect())
        .collect();

    println!("#soln1 = {}", soln1(&mat));
    println!("#soln2 = {}", soln2(&mat));
}
