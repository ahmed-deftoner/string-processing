use std::str::FromStr;

struct Point{
    x: u64,
    y: u64
}

enum Fold {
    X(u64),
    Y(u64)
}

impl FromStr for Point {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();
        return Ok(Point{
            x: x.parse().unwrap(),
            y: y.parse().unwrap()
        });
    }
}


impl FromStr for Fold {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (axis, value) = s.split_once("fold along").unwrap().1.split_once("=").unwrap();
        return match axis {
            "x" => Ok(Fold::X(value.parse().unwrap())),
            "y" => Ok(Fold::Y(value.parse().unwrap())),
            _ => unreachable!()
        };
    }
}

fn main() {
    
}
