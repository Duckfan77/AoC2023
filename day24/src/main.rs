use std::ops::{Add, Mul};

use itertools::Itertools;

use num::rational::Ratio;
type Rational128 = Ratio<i128>;

fn main() {
    let text = include_str!("../input");

    println!("Part 1:");
    part1(&text);

    println!("\nPart 2:");
    part2(&text);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Vec3 {
    x: Rational128,
    y: Rational128,
    z: Rational128,
}

impl Mul<Rational128> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Rational128) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

struct Ray {
    a: Vec3,
    v: Vec3,
}

impl Ray {
    fn from_line(line: &str) -> Self {
        let ((ax, ay, az), (vx, vy, vz)) = line
            .split_once(" @ ")
            .map(|(base, vec)| {
                (
                    base.split(", ")
                        .map(|d| d.trim().parse::<i128>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                    vec.split(", ")
                        .map(|d| d.trim().parse::<i128>().unwrap())
                        .collect_tuple()
                        .unwrap(),
                )
            })
            .unwrap();

        Self {
            a: Vec3 {
                x: Rational128::from_integer(ax),
                y: Rational128::from_integer(ay),
                z: Rational128::from_integer(az),
            },
            v: Vec3 {
                x: Rational128::from_integer(vx),
                y: Rational128::from_integer(vy),
                z: Rational128::from_integer(vz),
            },
        }
    }

    fn intersection_point_r2(&self, other: &Self) -> Option<Vec3> {
        if self.a == other.a {
            return Some(self.a);
        }

        let dx = other.a.x - self.a.x;
        let dy = other.a.y - self.a.y;
        let det = other.v.x * self.v.y - other.v.y * self.v.x;

        if det != 0.into() {
            let u = (dy * other.v.x - dx * other.v.y) / det;
            let v = (dy * self.v.x - dx * self.v.y) / det;
            if u >= 0.into() && v >= 0.into() {
                let result = self.a + self.v * u;
                return Some(result);
            }
        }

        None
    }
}

fn passes_r2(point: Vec3, min: Rational128, max: Rational128) -> bool {
    min <= point.x && point.x <= max && min <= point.y && point.y <= max
}

const MIN: i128 = 200000000000000;
const MAX: i128 = 400000000000000;

fn part1(text: &str) {
    let rays = text.lines().map(|line| Ray::from_line(line)).collect_vec();

    let min = Rational128::from_integer(MIN);
    let max = Rational128::from_integer(MAX);

    println!(
        "{}",
        rays.iter()
            .combinations(2)
            .filter(|rays| rays[0]
                .intersection_point_r2(rays[1])
                .is_some_and(|point| passes_r2(point, min, max)))
            .count()
    );
}

fn part2(text: &str) {}
