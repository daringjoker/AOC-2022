use std::{fs::File, io::Read, env, path, collections::HashMap};

struct RoundPlay {
    opp:String,
    my:String,
}

fn calc_score1(round:&RoundPlay)->u32{
    let mut verdict=" ";
    if round.opp=="A" && round.my =="X"{verdict="D";}
    if round.opp=="A" && round.my =="Y"{verdict="W";}
    if round.opp=="A" && round.my =="Z"{verdict="L";}
    if round.opp=="B" && round.my =="X"{verdict="L";}
    if round.opp=="B" && round.my =="Y"{verdict="D";}
    if round.opp=="B" && round.my =="Z"{verdict="W";}
    if round.opp=="C" && round.my =="X"{verdict="W";}
    if round.opp=="C" && round.my =="Y"{verdict="L";}
    if round.opp=="C" && round.my =="Z"{verdict="D";}

    let sel_score:u32=(round.my.as_bytes()[0]-"X".as_bytes()[0] +1).into();

    let win_score = match verdict{
        "D"=>3,
        "W"=>6,
        "L"=>0,
        &_=>0
    };

    return sel_score+win_score;
}


fn calc_score2(round:&RoundPlay)->u32{
    let mut choose=1;
    if round.opp=="A" && round.my =="X"{choose=3;}
    if round.opp=="A" && round.my =="Y"{choose=1;}
    if round.opp=="A" && round.my =="Z"{choose=2;}
    if round.opp=="B" && round.my =="X"{choose=1;}
    if round.opp=="B" && round.my =="Y"{choose=2;}
    if round.opp=="B" && round.my =="Z"{choose=3;}
    if round.opp=="C" && round.my =="X"{choose=2;}
    if round.opp=="C" && round.my =="Y"{choose=3;}
    if round.opp=="C" && round.my =="Z"{choose=1;}


    let win_score = match round.my.as_str(){
        "X"=>0,
        "Y"=>3,
        "Z"=>6,
        &_=>0
    };

    return choose+win_score;
}
fn soln1(rounds:&Vec<RoundPlay>)->u32{
    return rounds.iter().map(|x|calc_score1(&x)).sum();
}

fn soln2(rounds:&Vec<RoundPlay>)->u32{
    return rounds.iter().map(|x|calc_score2(&x)).sum();
}

fn main() {
    let argV:Vec<String> = env::args().collect();
    
    let filepath = path::Path::new(&argV[1]);

    let mut  infile = File::open(filepath).unwrap();
    let mut lines = String::new();

    infile.read_to_string(&mut lines).unwrap();

    let strategy:Vec<RoundPlay> = lines.split("\n")
                                        .map(|s|s.split(' ').map(|x|x.to_string()).collect())
                                        .map(|x:Vec<String>|RoundPlay{opp:x[0].to_string(),my:x[1].to_string()})
                                        .collect();
    
    
    println!("#soln1 = {}",soln1(&strategy));
    println!("#soln2 = {}",soln2(&strategy));
}
