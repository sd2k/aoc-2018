use hashbrown::{HashMap, HashSet};
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

#[inline(always)]
fn manhattan_distance(p: Point, q: Point) -> i32 {
    (p.x - q.x).abs() + (p.y - q.y).abs()
}

/// Get an iterator of points on the bounding box.
fn bbox(x0: i32, x1: i32, y0: i32, y1: i32) -> impl Iterator<Item = Point> {
    let top = (x0..x1).map(move |x| Point { x, y: y0 });
    let bottom = (x0..x1).map(move |x| Point { x, y: y1 });
    let left = ((y0 + 1)..(y1 - 1)).map(move |y| Point { x: x0, y });
    let right = ((y0 + 1)..(y1 - 1)).map(move |y| Point { x: x1, y });
    top.chain(right).chain(bottom).chain(left)
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
    let bounding_box = bbox(x_range.0, x_range.1, y_range.0, y_range.1);
    bounding_box
        .map(|b| {
            *points
                .iter()
                .map(|point| (point, manhattan_distance(*point, b)))
                .min_by_key(|x| x.1)
                .map(|(point, _)| point)
                .unwrap()
        })
        .collect()
}

fn get_closest_point_nodupes(points: &[Point], x: i32, y: i32) -> Option<&Point> {
    let distances = points
        .iter()
        .map(|p| (p, manhattan_distance(*p, Point { x, y })))
        .fold(
            HashMap::new(),
            |mut acc: HashMap<i32, Option<&Point>>, x| {
                acc.entry(x.1)
                    .and_modify(|e| *e = None)
                    .or_insert_with(|| Some(x.0));
                acc
            },
        );
    *distances.iter().min_by_key(|kv| kv.0).unwrap().1
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
    log::info!("Getting points with infinite areas");
    let invalid_points =
        get_points_with_infinite_areas(&points, (mins.0, maxes.0), (mins.1, maxes.1));

    let mut point_counts = HashMap::new();
    log::info!("Getting point counts");
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
    log::info!("Getting max point");
    let max_point = point_counts
        .iter()
        .filter(|(k, _)| !invalid_points.contains(k))
        .max_by_key(|(_, &v)| v)
        .unwrap();
    *max_point.1
}

fn distance_less_than_max(candidate: Point, points: &[Point], max: i32) -> bool {
    let mut dist = 0;
    for point in points {
        dist += manhattan_distance(*point, candidate);
        if dist >= max {
            return false;
        }
    }
    return true;
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
    for x in (mins.0)..(maxes.0) {
        for y in (mins.1)..(maxes.1) {
            if distance_less_than_max(Point { x, y }, &points, max) {
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
