use std::{env, fs::File, io::Read, path};

fn render_pixel(position: i64, cy: u64) -> char {
    let cycle = cy % 40;
    let char = if (position as i128 - cycle as i128 + 1).abs() <= 1 {
        '█'
    } else {
        ' '
    };
    char
}

fn soln1(commands: &Vec<&str>) -> i128 {
    let mut x: i64 = 1;
    let mut cycle: u64 = 1;
    let mut next_catch: u64 = 20;
    let mut ans: i128 = 0;
    let mut screen: String = String::new();
    commands.iter().for_each(|s| {
        let mut next_x = x;
        let command_tokens: Vec<&str> = s.split(" ").collect();
        match command_tokens[..] {
            ["noop"] => {
                screen.push(render_pixel(x, cycle));
                cycle = cycle + 1;
            }
            ["addx", val] => {
                screen.push(render_pixel(x, cycle));
                screen.push(render_pixel(x, cycle + 1));
                cycle = cycle + 2;
                next_x = x + val.parse::<i64>().unwrap();
            }
            _ => {
                dbg!(command_tokens, x, cycle);
            }
        }
        if next_catch < cycle {
            ans = ans + (next_catch as i128) * x as i128;
            next_catch = next_catch + 40;
        }

        x = next_x;
    });
    let scr_prn: Vec<String> = screen
        .chars()
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|x| x.iter().collect::<String>())
        .collect();

    println!("#soln2 = \n{}", scr_prn.join("\n"));
    return ans;
}

fn main() {
    let arg_v: Vec<String> = env::args().collect();

    let filepath = if arg_v.len() > 1 {
        path::Path::new(&arg_v[1])
    } else {
        path::Path::new("input.txt")
    };

    let mut infile = File::open(filepath).unwrap();
    let mut lines = String::new();

    infile.read_to_string(&mut lines).unwrap();
    let commands: Vec<&str> = lines.strip_suffix("\n").unwrap().split('\n').collect();

    println!("#soln1 = {}", soln1(&commands));
}
