use std::{path, fs::File, io::Read, env,};

fn soln1(line:&str)->u32{
    return (line.chars().zip(line.chars().skip(1)).zip(line.chars().skip(2).zip(line.chars().skip(3))).take_while(|((a,b),(c,d))|{
       let mut intr=  vec![a,b,c,d];
       intr.sort();
       intr.dedup();
       return intr.len()<4;
    }).count() + 4) as u32;
}

fn soln2(line:&str)->u32{
    let mut stack:Vec<char>=  line.chars().take(13).collect();
    return 14 + line.chars().skip(13).take_while(|c|{
        stack.push(*c);
        let mut intr = stack.clone();
        stack.remove(0);
        intr.sort();
        intr.dedup();
        return intr.len()<14;
    }).count() as u32;
}

fn main() {
    let arg_v:Vec<String> = env::args().collect();


    let filepath = path::Path::new(&arg_v[1]);

    let mut  infile = File::open(filepath).unwrap();
    let mut lines = String::new();

    infile.read_to_string(&mut lines).unwrap();

    
    

    println!("#soln1 = {}",soln1(&lines));
    println!("#soln2 = {}",soln2(&lines));

}