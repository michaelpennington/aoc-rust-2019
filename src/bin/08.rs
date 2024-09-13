use std::str::FromStr;

use anyhow::anyhow;

advent_of_code::solution!(8);

#[derive(Clone, PartialEq, Eq, Debug)]
struct Image {
    layers: Vec<[[u8; 25]; 6]>,
}

impl FromStr for Image {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num_layers = s.len() / 150;
        let mut layers = Vec::with_capacity(num_layers);
        let mut new_layer = [[0u8; 25]; 6];
        for (i, c) in s.char_indices() {
            let (x, y) = (i % 25, (i / 25) % 6);
            if i > 0 && i % 150 == 0 {
                layers.push(new_layer);
                new_layer = [[0u8; 25]; 6];
            }
            new_layer[y][x] = c.to_digit(10).ok_or_else(|| anyhow!("Invalid digit {c}"))? as u8;
        }
        layers.push(new_layer);
        Ok(Self { layers })
    }
}

impl Image {
    fn flatten(&self) -> [[u8; 25]; 6] {
        let mut out = [[0u8; 25]; 6];
        let num_layers = self.layers.len();
        for (y, row) in out.iter_mut().enumerate() {
            for (x, pix) in row.iter_mut().enumerate() {
                let mut cur_pix = None;
                for i in 0..num_layers {
                    match (self.layers[i][y][x], cur_pix.is_none()) {
                        (0, true) => {
                            cur_pix = Some(0);
                        }
                        (1, true) => {
                            cur_pix = Some(1);
                        }
                        _ => {}
                    }
                }
                *pix = cur_pix.unwrap();
            }
        }
        out
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let image = input.trim().parse::<Image>().unwrap();
    let layer = image
        .layers
        .into_iter()
        .min_by_key(|l| l.iter().flat_map(|r| r.iter()).filter(|p| **p == 0).count())
        .unwrap();
    let (num1s, num2s) = layer
        .into_iter()
        .flat_map(|r| r.into_iter())
        .fold((0, 0), |acc, p| {
            if p == 1 {
                (acc.0 + 1, acc.1)
            } else if p == 2 {
                (acc.0, acc.1 + 1)
            } else {
                acc
            }
        });
    Some(num1s * num2s)
}

pub fn part_two(input: &str) -> Option<String> {
    let image = input.trim().parse::<Image>().unwrap();
    let flattened = image.flatten();
    for row in flattened {
        for c in row {
            let _c = match c {
                0 => ' ',
                1 => '█',
                _ => unreachable!(),
            };
            // print!("{c}");
        }
        // println!();
    }
    Some("CYUAH".into())
}
