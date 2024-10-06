use advent_of_code::intcode::Program;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut comp = input.parse::<Program<i32>>().unwrap();
    let mut num_pulled = 0;
    comp.cache();
    for y in 0..50 {
        for x in 0..50 {
            comp.reset();
            comp.input([x, y]);
            let res = comp.next().unwrap();
            if res == 1 {
                num_pulled += 1;
            }
        }
    }
    Some(num_pulled)
}

fn square_fits(program: &mut Program<i64>, x: i64, y: i64) -> bool {
    [(0, 0), (99, 0), (0, 99), (100, 99)]
        .into_iter()
        .map(|(off_x, off_y)| {
            program.input([x + off_x, y + off_y]);
            let res = program.next().unwrap();
            program.reset();
            res
        })
        .all(|b| b == 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut comp = input.parse::<Program<i64>>().unwrap();
    comp.cache();
    let mut max_x = 5000;
    let mut max_y = 10000;
    let mut min_x;
    let mut min_y;
    let mut old_x = 0;
    let mut old_y = 0;
    loop {
        let mut y_done = false;
        let mut x_done = false;
        min_y = 0;
        min_x = 0;
        while !y_done {
            if square_fits(&mut comp, min_x, min_y) {
                max_y = min_y;
                max_x = min_x;
            } else if square_fits(&mut comp, max_x, (max_y + min_y) / 2) {
                max_y = (max_y + min_y) / 2;
            } else {
                min_y = (max_y + min_y) / 2;
            }
            y_done = max_y == min_y || max_y == min_y + 1;
        }
        while !x_done {
            if square_fits(&mut comp, min_x, min_y) {
                max_y = min_y;
                max_x = min_x;
            } else if square_fits(&mut comp, (max_x + min_x) / 2, max_y) {
                max_x = (max_x + min_x) / 2;
            } else {
                min_x = (max_x + min_x) / 2;
            }
            x_done = max_x == min_x || max_x == min_x + 1;
        }
        y_done = false;
        min_y = 0;
        while !y_done {
            if square_fits(&mut comp, min_x, min_y) {
                max_y = min_y;
                max_x = min_x;
            } else if square_fits(&mut comp, max_x, (max_y + min_y) / 2) {
                max_y = (max_y + min_y) / 2;
            } else {
                min_y = (max_y + min_y) / 2;
            }
            y_done = max_y == min_y || max_y == min_y + 1;
        }

        if old_x == min_x && old_y == min_y {
            break Some(10000 * max_x + max_y);
        }
        old_x = min_x;
        old_y = min_y;
    }
}
