use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().trim().parse().unwrap();
        Ok(Point { x, y })
    }
}

fn manhattan_distance(p: Point, q: Point) -> i32 {
    (p.x - q.x).abs() + (p.y - q.y).abs()
}

/// Determine which points have finite areas and are therefore valid candidates.
///
/// This is done by getting the bounding box and determining which points
/// don't touch it.
fn get_points_with_infinite_areas(
    points: &[Point],
    x_range: (i32, i32),
    y_range: (i32, i32),
) -> HashSet<Point> {
    let closest_to_top = ((x_range.0)..(x_range.1)).map(|x| {
        *points
            .iter()
            .map(|point| (point, manhattan_distance(*point, Point { x, y: y_range.0 })))
            .min_by_key(|x| x.1)
            .map(|(point, _)| point)
            .unwrap()
    });
    let closest_to_bottom = ((x_range.0)..(x_range.1)).map(|x| {
        *points
            .iter()
            .map(|point| (point, manhattan_distance(*point, Point { x, y: y_range.1 })))
            .min_by_key(|x| x.1)
            .map(|(point, _)| point)
            .unwrap()
    });
    let closest_to_left = ((y_range.0)..(y_range.1)).map(|y| {
        *points
            .iter()
            .map(|point| (point, manhattan_distance(*point, Point { x: x_range.0, y })))
            .min_by_key(|x| x.1)
            .map(|(point, _)| point)
            .unwrap()
    });
    let closest_to_right = ((y_range.0)..(y_range.1)).map(|y| {
        *points
            .iter()
            .map(|point| (point, manhattan_distance(*point, Point { x: x_range.1, y })))
            .min_by_key(|x| x.1)
            .map(|(point, _)| point)
            .unwrap()
    });
    closest_to_top
        .chain(closest_to_right)
        .chain(closest_to_bottom)
        .chain(closest_to_left)
        .collect::<HashSet<Point>>()
}

fn get_closest_point_nodupes(points: &[Point], x: i32, y: i32) -> Option<&Point> {
    let distances = points
        .iter()
        .map(|p| (p, manhattan_distance(*p, Point { x, y })))
        .fold(HashMap::new(), |mut acc: HashMap<i32, Vec<&Point>>, x| {
            acc.entry(x.1)
                .and_modify(|e| e.push(x.0))
                .or_insert_with(|| vec![x.0]);
            acc
        });
    let closest_points = distances.iter().min_by_key(|kv| kv.0).unwrap().1;
    match closest_points.len() {
        1 => Some(closest_points[0]),
        _ => None,
    }
}

pub fn part1(input: &[&str]) -> i32 {
    let points: Vec<Point> = input.iter().map(|l| Point::from_str(l).unwrap()).collect();
    let mins = (
        points.iter().map(|p| p.x).min().unwrap(),
        points.iter().map(|p| p.y).min().unwrap(),
    );
    let maxes = (
        points.iter().map(|p| p.x).max().unwrap(),
        points.iter().map(|p| p.y).max().unwrap(),
    );
    let invalid_points =
        get_points_with_infinite_areas(&points, (mins.0, maxes.0), (mins.1, maxes.1));

    let mut point_counts = HashMap::new();
    for x in (mins.0)..(maxes.0) {
        for y in (mins.1)..(maxes.1) {
            let closest_point = get_closest_point_nodupes(&points, x, y);
            if let Some(closest_point) = closest_point {
                point_counts
                    .entry(closest_point)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }
    let max_point = point_counts
        .iter()
        .filter(|(k, _)| !invalid_points.contains(k))
        .max_by_key(|(_, &v)| v)
        .unwrap();
    *max_point.1
}

fn total_distance_to_all_points(candidate: Point, points: &[Point]) -> i32 {
    points
        .iter()
        .map(|p| manhattan_distance(*p, candidate))
        .sum()
}

pub fn part2(input: &[&str], max: i32) -> i32 {
    let points: Vec<Point> = input.iter().map(|l| Point::from_str(l).unwrap()).collect();
    let mins = (
        points.iter().map(|p| p.x).min().unwrap(),
        points.iter().map(|p| p.y).min().unwrap(),
    );
    let maxes = (
        points.iter().map(|p| p.x).max().unwrap(),
        points.iter().map(|p| p.y).max().unwrap(),
    );
    let mut size = 0;
    for x in (mins.0 - (max / 2))..(maxes.0 + (max / 2)) {
        for y in (mins.1 - (max / 2))..(maxes.1 + (max / 2)) {
            if total_distance_to_all_points(Point { x, y }, &points) < max {
                size += 1;
            }
        }
    }
    size
}

#[cfg(test)]
mod tests {

    use super::{part1, part2};

    #[test]
    fn test_part1() {
        let input = &["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
        assert_eq!(part1(input), 17);
    }

    #[test]
    fn test_part2() {
        let input = &["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
        assert_eq!(part2(input, 32), 16);
    }
}
