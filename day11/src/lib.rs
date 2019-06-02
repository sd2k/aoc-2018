use std::ops::RangeInclusive;

use itertools::Itertools;
use rayon::prelude::*;
use summed_area_table::{SummedAreaTable, SummedAreaTableSource, VecSource};

#[derive(Clone, Copy, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
struct Serial(usize);

fn power_level(coords: Coordinates, serial: Serial) -> isize {
    let rack_id = coords.x + 10;
    let power_level = rack_id * coords.y;
    let power_level = power_level + serial.0;
    let power_level = power_level * rack_id;
    let power_level = (power_level / 100) % 10;
    power_level as isize - 5
}

type Axis = RangeInclusive<usize>;

fn powers(x_range: Axis, y_range: Axis, serial: Serial) -> Vec<isize> {
    let all_coords: Vec<(usize, usize)> = x_range.cartesian_product(y_range).collect();
    all_coords
        .into_par_iter()
        .map(|(x, y)| power_level(Coordinates { x, y }, serial))
        .collect()
}

fn max_corner(
    x_range: &[usize],
    y_range: &[usize],
    summed: &SummedAreaTable,
    window: usize,
) -> (Coordinates, isize) {
    x_range
        .windows(window)
        .map(|xs| {
            y_range
                .windows(window)
                .map(|ys| {
                    // Subtract one because index in x_range / y_range start at 1
                    let top_left = (xs[0] - 1, ys[0] - 1);
                    let bottom_right = (xs[window - 1] - 1, ys[window - 1] - 1);
                    let sum = summed.get_sum(top_left, bottom_right);
                    (Coordinates { x: xs[0], y: ys[0] }, sum as isize)
                })
                .collect::<Vec<(Coordinates, isize)>>()
        })
        .flatten()
        .max_by_key(|el| el.1)
        .unwrap()
}

pub fn part1(serial: usize, width: usize, window: usize) -> (usize, usize) {
    let serial = Serial(serial);
    let x = 1..=width;
    let y = x.clone();
    let powers = powers(x.clone(), y.clone(), serial);
    let summed = VecSource::new(&powers, width, width).calculate_full_summed_area_table();
    let x: Vec<usize> = x.collect();
    let y: Vec<usize> = y.collect();
    let coordinates = max_corner(&x, &y, &summed, window).0;
    (coordinates.y, coordinates.x)
}

pub fn part2(serial: usize, width: usize) -> (usize, usize, usize) {
    let serial = Serial(serial);
    let x = 1..=width;
    let y = x.clone();
    let powers = powers(x.clone(), y.clone(), serial);
    let summed = VecSource::new(&powers, width, width).calculate_full_summed_area_table();
    let x: Vec<usize> = x.collect();
    let y: Vec<usize> = y.collect();
    let (coordinates, window) = (1..(width + 1))
        .into_par_iter()
        .map(|window| (max_corner(&x, &y, &summed, window), window))
        .max_by_key(|el| (el.0).1)
        .unwrap();
    (coordinates.0.y, coordinates.0.x, window)
}

#[cfg(test)]
mod tests {

    use super::{part1, part2, power_level, Coordinates, Serial};

    #[test]
    fn test_power_level() {
        assert_eq!(power_level(Coordinates { x: 3, y: 5 }, Serial(8)), 4);
        assert_eq!(power_level(Coordinates { x: 33, y: 45 }, Serial(18)), 4);
        assert_eq!(power_level(Coordinates { x: 122, y: 79 }, Serial(57)), -5);
        assert_eq!(power_level(Coordinates { x: 217, y: 196 }, Serial(39)), 0);
        assert_eq!(power_level(Coordinates { x: 101, y: 153 }, Serial(71)), 4);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(18, 300, 3), (33, 45));
        assert_eq!(part1(42, 300, 3), (21, 61));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(18, 300), (90, 269, 16));
        assert_eq!(part2(42, 300), (232, 251, 12));
    }
}
