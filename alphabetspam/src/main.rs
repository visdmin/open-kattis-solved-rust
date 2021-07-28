/*
    Problem:  https://open.kattis.com/problems/alphabetspam
    CPU time: 0.00 s
*/


use std::io;
use std::io::prelude::*;
use std::process;

fn main()
{
    let stdin = io::stdin();
    let line  = match stdin.lock().lines().next() {
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
    };

    if line.is_empty() {
        process::exit(0);
    }

    let mut results: [i32; 4] = [0; 4];
    for c in line.chars() {
        match c {
            '_' => {
                results[0] += 1;
            },
            'a'..='z' => {
                results[1] += 1;
            },
            'A'..='Z' => {
                results[2] += 1;
            },
            _        => {
                results[3] += 1;
            }
        }
    }

    for result_idx in 0..results.len() {
        println!(
            "{}",
            (f64::from(results[result_idx]) / f64::from(line.len() as i32))
        );
    }
}
