use std::{collections::hash_set::HashSet, env, fs::File, io::Read, path, vec};
#[derive(Clone, PartialEq, Eq, Hash, Debug, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn add(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn sub(&self, other: &Pos) -> Pos {
        Pos {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn abs(&self) -> Pos {
        Pos {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    fn into_diff(&self) -> Pos {
        let abs_delta = self.abs();
        let mut movements = self.clone();
        if abs_delta.x != 2 && abs_delta.y != 2 {
            movements = Pos { x: 0, y: 0 };
        }
        if abs_delta.x == 2 {
            movements.x = movements.x / 2;
        }
        if abs_delta.y == 2 {
            movements.y = movements.y / 2;
        }
        movements
    }
}

fn get_follower_position_for(leader_position: &Pos, follower_position: &Pos) -> Pos {
    let diff = &leader_position.sub(follower_position).into_diff();
    follower_position.add(diff)
}

fn update_by_command(command: &str, old_pos: &Pos) -> Pos {
    match command {
        "U" => Pos {
            x: old_pos.x,
            y: old_pos.y + 1,
        },
        "D" => Pos {
            x: old_pos.x,
            y: old_pos.y - 1,
        },
        "L" => Pos {
            x: old_pos.x - 1,
            y: old_pos.y,
        },
        "R" => Pos {
            x: old_pos.x + 1,
            y: old_pos.y,
        },
        &_ => panic!("unexpected command found"),
    }
}

fn calc_unique_position_for(commands: &Vec<(&str, u64)>, follower_count: u32) -> u64 {
    let mut objs = vec![Pos { x: 0, y: 0 }; (follower_count + 1) as usize];
    let mut positions: HashSet<Pos> = HashSet::new();

    commands.iter().for_each(|(command, unit)| {
        (0..*unit).for_each(|_| {
            objs.clone().iter().enumerate().for_each(|(i, follower)| {
                objs[i] = if i == 0 {
                    update_by_command(command, objs.first().unwrap())
                } else {
                    get_follower_position_for(&objs[i - 1], follower)
                };
            });
            positions.insert(*objs.last().unwrap());
        });
    });

    return positions.len() as u64;
}

fn soln1(commands: &Vec<(&str, u64)>) -> u64 {
    calc_unique_position_for(commands, 1)
}

fn soln2(commands: &Vec<(&str, u64)>) -> u64 {
    calc_unique_position_for(commands, 9)
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
    let commands: Vec<(&str, u64)> = lines
        .split('\n')
        .map(|x| {
            let y: Vec<&str> = x.split(" ").collect();
            return (y[0], y[1].to_string().parse().unwrap());
        })
        .collect();

    println!("#soln1 = {}", soln1(&commands));
    println!("#soln2 = {}", soln2(&commands));
}
