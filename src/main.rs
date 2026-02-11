/* use std::{fs, path::PathBuf};

use anyhow::{Context, Result, anyhow}; 

enum Option2<T> {
    None,
    Some(T)
}

impl<T> Option2<T> {
    fn is_some(&self) -> bool {
        return match self {
            Option2::None => false,
            Option2::Some(_) => true,
        }
     }  

     fn unwrap(self) -> T {
        match self {
            Option2::Some(value) => value,
            Option2::None => panic!("called unwrap on None"),
        }
     }
}
fn error_me(throw: bool) ->  Result<()> {
    if throw {
        return Err(anyhow!("this should never be true"));
    }

    std::fs::read(PathBuf::from("/foo")).context("Hey unable to do this")?;

    return Ok(())
} */

fn get_input() -> &'static str {
    return "forward 5
down 5
forward 8
up 3
down 8
forward 2";
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

fn parse_line(line: &str) -> Point {
    let (dir, amount) = line.split_once(" ").expect("must contain a whitespace");
    let amount = str::parse(amount).expect("second arg must be an integer");

    if dir == "forward" {
        return Point { x: amount, y: 0 };
    } else if dir == "up" {
        return Point { x: 0, y: amount };
    }

    return Point { x: 0, y: amount };
}

fn main() {
    /* let foo = Option2::Some(5);
 
    if foo.is_some() {
        let value = foo.unwrap();
    }

    if error_me(true).is_ok() {

    } */

   let result = get_input()
    .lines()
    .map(parse_line)
    .fold(Point {x:0 , y:0}, |mut acc,point| {
        acc.x += point.x;
        acc.y += point.y;
        return acc;
    });

    println!("{:?}", result);

}
