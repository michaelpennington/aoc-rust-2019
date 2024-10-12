use advent_of_code::intcode::{NonBlockProgram, Program};

advent_of_code::solution!(23);
#[derive(Debug)]
struct Network {
    computers: Vec<NonBlockProgram<i64>>,
}

impl Network {
    fn new(program: Program<i64>, num: usize) -> Self {
        let mut computers = Vec::with_capacity(num);
        for id in 0..num {
            let mut program = program.clone();
            program.input(std::iter::once(id as i64));
            computers.push(program.into());
        }
        Self { computers }
    }

    fn calc_pt1(mut self) -> Option<i64> {
        loop {
            for i in 0..self.computers.len() {
                let mut msg = Vec::new();
                loop {
                    match self.computers[i].next() {
                        Some(Ok(v)) => {
                            msg.push(v);
                            if msg.len() == 3 {
                                let addr = msg[0] as usize;
                                let (x, y) = (msg[1], msg[2]);
                                if addr == 255 {
                                    return Some(y);
                                } else {
                                    self.computers[addr].program.input([x, y]);
                                }
                                msg.clear();
                            }
                        }
                        Some(Err(true)) => break,
                        Some(Err(false)) => {}
                        None => break,
                    }
                }
            }
        }
    }

    fn calc_pt2(mut self) -> Option<i64> {
        let mut next_nat = None;
        let mut last_y = -1;
        loop {
            let mut all_idle = true;
            for i in 0..self.computers.len() {
                let mut msg = Vec::new();
                loop {
                    match self.computers[i].next() {
                        Some(Ok(v)) => {
                            all_idle = false;
                            msg.push(v);
                            if msg.len() == 3 {
                                let addr = msg[0] as usize;
                                let (x, y) = (msg[1], msg[2]);
                                if addr == 255 {
                                    next_nat = Some((x, y));
                                } else {
                                    self.computers[addr].program.input([x, y]);
                                }
                                msg.clear();
                            }
                        }
                        Some(Err(true)) => break,
                        Some(Err(false)) => {}
                        None => break,
                    }
                }
            }
            if all_idle {
                if let Some(n) = next_nat.take() {
                    if n.1 == last_y {
                        return Some(last_y);
                    }
                    last_y = n.1;
                    self.computers[0].program.input([n.0, n.1]);
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut comp = input.parse::<Program<i64>>().unwrap();
    comp.set_default_input(-1);
    let network = Network::new(comp, 50);
    network.calc_pt1()
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut comp = input.parse::<Program<i64>>().unwrap();
    comp.set_default_input(-1);
    let network = Network::new(comp, 50);
    network.calc_pt2()
}
