use std::{str::FromStr};

#[derive(Debug)]
struct Point{
    x: u64,
    y: u64
}

#[derive(Debug)]
enum Fold {
    X(u64),
    Y(u64)
}

impl FromStr for Point {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        println!("{}:{}", x,y);
        return Ok(Point{
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        });
    }
}


impl FromStr for Fold {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value) = s.split_once("fold along")
        .unwrap().
        1
        .split_once("=")
        .unwrap();
        return match axis {
            "x" => Ok(Fold::X(value.parse().unwrap())),
            "y" => Ok(Fold::Y(value.parse().unwrap())),
            _ => unreachable!(),
        };
    }
}

fn main() {
   // env::set_var("RUST_BACKTRACE", "1");
    let (points, folds) = include_str!("./sample.in")
    .trim()
    .split_once("\n\n")
    .unwrap();
    let points: Vec<Point> = points.lines()
    .map(str::parse)
    .map(Result::unwrap)
    .collect();
    let folds: Vec<Point> = folds.lines()
    .map(str::parse)
    .map(Result::unwrap)
    .collect();
    for p in points {
        println!("x : {:?} - y : {:?}", p.x,p.y);
    }

    for f in folds {
        println!("{:?}", f);
    }
}
