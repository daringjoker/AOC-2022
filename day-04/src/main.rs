use std::{fs::File, io::Read, env, path, cmp::{max_by, min_by}};

#[derive(Clone, Copy)]
struct Section{
    from:u32,
    to:u32,
}

impl Section {    
    fn size(&self)->u32{
        return self.to-self.from;
    }

    fn contains(&self,point:u32)->bool{
        return point>=self.from && point<=self.to;
    }

    fn overlaps(&self,other:&Section)->bool{
        return other.contains(self.from) || other.contains(self.to)||self.contains(other.from)||self.contains(other.to);
    }
}

fn soln1(sec_list:&Vec<(Section,Section)>)->u32{
     return sec_list.iter().filter(|(sec1,sec2)|{
        let bigger= max_by(sec1, sec2, |x1,x2|x1.size().cmp(&x2.size()));
        let smaller = min_by(sec1, sec2, |x1,x2|x1.size().cmp(&x2.size()));

        return (bigger.from<=smaller.from) && (bigger.to>=smaller.to);
    }).count().try_into().unwrap();
}


fn soln2(sec_list:&Vec<(Section,Section)>)->u32{
     return sec_list.iter().filter(|(sec1,sec2)|{
       return sec1.overlaps(&sec2);
    }).count().try_into().unwrap();
}

fn main() {
    let arg_v:Vec<String> = env::args().collect();

    
    let filepath = path::Path::new(&arg_v[1]);

    let mut  infile = File::open(filepath).unwrap();
    let mut lines = String::new();

    infile.read_to_string(&mut lines).unwrap();

    let section_list:Vec<(Section,Section)> =lines.split("\n").map(|x|{
    let sections:Vec<Section> = x.split(",").map(|x|{
       let ep:Vec<u32> = x.split("-").map(|x|x.parse().unwrap()).collect();
       return Section{
        from:ep[0],
        to:ep[1]
       }
    }).collect();

    return (sections[0],sections[1])

    }).collect();


    println!("#soln1 = {}",soln1(&section_list));
    println!("#soln2 = {}",soln2(&section_list));

}
