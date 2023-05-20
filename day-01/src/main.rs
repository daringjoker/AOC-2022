use std::{env, fs::File, io::Read, path};
fn soln1(elfGroup: &Vec<Vec<i64>>) -> i64 {
    return elfGroup.iter().map(|x| x.iter().sum()).max().unwrap();
}

fn soln2(elfGroup: &Vec<Vec<i64>>) -> i64 {
    let mut elfCarry: Vec<i64> = elfGroup.iter().map(|x| x.iter().sum()).collect();

    elfCarry.sort();

    return elfCarry.iter().rev().take(3).sum();
}

fn main() {
    let argV: Vec<String> = env::args().collect();

    let filepath = path::Path::new(&argV[1]);

    let mut infile = File::open(filepath).unwrap();
    let mut lines = String::new();

    infile.read_to_string(&mut lines).unwrap();

    let elfStore: Vec<Vec<i64>> = lines
        .split("\n\n")
        .map(|group| {
            group
                .split("\n")
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let ans1 = soln1(&elfStore);
    let ans2 = soln2(&elfStore);

    println!("#soln1 = {ans1}");
    println!("#soln2 = {ans2}");
}
