/*
    Problem:  https://open.kattis.com/problems/polygonarea
    CPU time: 0.00 s
*/


use std::io;
use std::io::prelude::*;
use std::process;
use crate::io::Error;


struct Point
{
    x: i32,
    y: i32
}

impl Point
{
    fn new(x: i32, y: i32) -> Point {
        Point {
            x: x,
            y: y
        }
    }
}

enum Orientation
{
    CounterClockwise,
    Clockwise,
}

impl Orientation
{
    fn to_string(&self) -> String
    {
        match self {
            Orientation::Clockwise        => String::from("CW"),
            Orientation::CounterClockwise => String::from("CCW")
        }
    }
}

struct Polygon<'a>
{
    points : &'a Vec<Point>
}

impl<'a> Polygon<'a>
{
    fn new(points: &'a Vec<Point>) -> Polygon<'a>
    {
        Polygon {
            points: points,
        }
    }

    fn get_area_and_orientation(&self) -> (Orientation, f64)
    {

        let mut temp  = 0.0;
        for idx in 0..self.points.len() {
            let s_idx = if (idx + 1) < self.points.len() {
                idx + 1
            } else {
                0
            };

            temp = temp + f64::from(self.points[idx].x * self.points[s_idx].y);
            temp = temp - f64::from(self.points[idx].y * self.points[s_idx].x);
        }

        let area = temp / 2.0;

        let orientation = if area > 0.0 {
            Orientation::CounterClockwise
        } else {
            Orientation::Clockwise
        };

        return (orientation, area.abs());
    }
}

fn iter_line(item: &Option<Result<String, Error>>) -> String {
    match item {
        Some(item) => {
            match item {
                Ok(string)  => string.to_string(),
                Err(err) => {
                    println!("Input error: (err {})", err);
                    process::exit(1);
                }
            }
        },
        None => String::from("")
    }
}

fn main()
{
    let stdin         = io::stdin();
    let mut iterator  = stdin.lock().lines();
    let mut points    = Vec::<Point>::new();
    let mut line: String;

    loop {
        line = iter_line(&iterator.next());

        if line.is_empty() {
            process::exit(0);
        }

        let point_cnt = match line.trim().parse::<i32>() {
            Ok(val)   => {
                match val {
                    0 => process::exit(0),
                    _ => val
                }
            },
            Err(_err) => process::exit(0)
        };

        for _point_idx in 0..point_cnt {
            line = iter_line(&iterator.next());

            let point_pair: Vec<i32> = line.split_whitespace().map(|num| num.parse().unwrap()).collect();

            if point_pair.len() < 2 {
                process::exit(1);
            }

            points.push(Point::new(point_pair[0], point_pair[1]));
        }

        let polygon = Polygon::new(&points);

        let (orientation, area) = polygon.get_area_and_orientation();
        println!("{} {:.1}", orientation.to_string(), area);
        points.clear();
    }
}
