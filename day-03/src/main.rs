use std::{fs::File, io::Read, env, path};

  fn priority(x:&char)->usize{(if x.is_lowercase() {*x as u8-b'a'+1}else{*x as u8-b'A'+27}).into()}

fn intersect(first:&String,second:&String)->String{
  let mut item_count:Vec<i32> = Vec::new();
  item_count.resize(53, 0);
  first.chars().

  first.chars().for_each(|c|item_count[priority(&c)]=item_count[priority(&c)]+1);

  let ans:String= second.chars().filter(|c|{
    let index = priority(&c);
    if item_count[index] >0{
      item_count[index]=item_count[index]-1;
      return true;
    }

    return false;
  }).collect();

  
  return ans;

}

fn intersect3(first:&String,second:&String,third:&String)->String{
  return intersect(&first, &intersect(&second,&third))
}

fn soln1(back_packs:&Vec<String>)->u64{
  let commons:Vec<String> = back_packs.iter()
  .map(|x|x.split_at((x.len()+1)/2))
  .map(|(first,second)|intersect(&first.to_string(), &second.to_string())).collect();


  let prio:Vec<usize>= commons.iter()
    .map(|s|{
      s.chars()
        .map(|x|priority(&x))
        .take(1)
        .sum::<usize>()
    }).collect();


   return prio.iter().sum::<usize>().try_into().unwrap();
  
}

fn soln2(back_packs:&Vec<String>)-> u64{
  let commons:Vec<String> = back_packs.chunks(3).map(|x|intersect3(&x[0], &x[1],&x[2])).collect();

  return commons.iter().map(|x|priority(&x.chars().nth(0).unwrap())).sum::<usize>().try_into().unwrap();
}

fn main() {
    let arg_v:Vec<String> = env::args().collect();

    
    let filepath = path::Path::new(&arg_v[1]);

    let mut  infile = File::open(filepath).unwrap();
    let mut lines = String::new();

    infile.read_to_string(&mut lines).unwrap();

    let back_packs:Vec<String> =lines.split("\n").map(|x|x.to_string()).collect();


    println!("#soln1 = {}",soln1(&back_packs));
    println!("#soln2 = {}",soln2(&back_packs));

}
