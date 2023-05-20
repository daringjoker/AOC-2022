use std::{path, fs::File, io::Read, env,};

#[derive(Debug)]
struct Instr{
    num:u32,
    from:u32,
    to:u32
}

fn parse_stacks(stack_repr:String)->Vec<Vec<char>>{
    let lines=stack_repr.split("\n").map(|x|x.to_string()).collect::<Vec<String>>();
    let idx = lines.iter().nth_back(0).unwrap();
    let num_stacks = idx.split(' ').filter(|x|x.trim().len()>0).count();

    // println!("num of stacks ={} and idx line = {}",num_stacks,idx);

    let mut stacks:Vec<Vec<char>> = Vec::new();
    stacks.resize(num_stacks+1, Vec::new());

    let body:Vec<&String>= lines.iter().rev().skip(1).collect();

    for dat in body.iter(){
        dat.chars().collect::<Vec<char>>().chunks(4).enumerate().for_each(|(i,c)|{
            if c[1].is_alphabetic() {
                stacks[i+1].push(c[1]);
            }
        });
    }

    // println!("{:?}",stacks);

    return stacks;
}

fn soln1(stacks:&mut Vec<Vec<char>>,instr_vec:&mut Vec<Instr>)->String{
    for instr in instr_vec{
        for x in 0..instr.num{
            let last_chr=stacks[instr.from as usize].pop().unwrap();
            stacks[instr.to as usize].push(last_chr);
        }
    }

return stacks.iter().filter(|x|x.len()>0).map(|s|s.iter().nth_back(0).unwrap().to_string()).collect();
}

fn soln2(stacks:&mut Vec<Vec<char>>,instr_vec:&mut Vec<Instr>)->String{
    for instr in instr_vec{
        let mut tmp_stack:Vec<char>=Vec::new();
        for _ in 0..instr.num{
            let last_chr=stacks[instr.from as usize].pop().unwrap();
            tmp_stack.push(last_chr);
        }
        tmp_stack.reverse();
        stacks[instr.to as usize].append(&mut tmp_stack);
    }

return stacks.iter().filter(|x|x.len()>0).map(|s|s.last().unwrap().to_string()).collect();
}

fn main() {
    let arg_v:Vec<String> = env::args().collect();


    let filepath = path::Path::new(&arg_v[1]);

    let mut  infile = File::open(filepath).unwrap();
    let mut lines = String::new();

    infile.read_to_string(&mut lines).unwrap();

    let data:Vec<String> = lines.split("\n\n").map(|x|x.to_string()).collect();
    let stacks_data= &data[0];
    let mut instrs:Vec<Instr>= data[1].split("\n").map(|s|{
        let  toks:Vec<&str> =s.split(" ").collect();
        let n:u32= toks[1].parse::<u32>().unwrap();
        let f:u32= toks[3].parse::<u32>().unwrap();
        let t:u32= toks[5].parse::<u32>().unwrap();
        return Instr{
            num:n,
            from:f,
            to:t
        };
    }).collect();

    
    let mut stacks = parse_stacks(stacks_data.to_string());
    

    println!("#soln1 = {}",soln1(&mut stacks.clone(),&mut instrs));
    println!("#soln2 = {}",soln2(&mut stacks,&mut instrs));

}