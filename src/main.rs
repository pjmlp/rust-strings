/*
 * rust-strings, A solution to an Hacherrank exercise.
 * Copyright (C) 2018, Paulo Pinto
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software Foundation,
 * Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301  USA
 */

extern crate num;

use std::io::{self, BufRead, Stdin};

use num::{pow, BigUint, Zero, One};

// Reads the number of test_cases
fn test_cases(stdin: &Stdin) ->  u32 {
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    
    if let Err(_) = handle.read_line(&mut buffer) {
        return 0
    }
    
    match buffer.trim().parse::<u32>() {
        Ok(num) if num >= 1 && num <= 50 => num,
        Ok(_) => 0,
        Err(_) => 0
    }
}


fn elems(stdin: &Stdin) -> Vec<u64> {
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    
    if let Err(_) = handle.read_line(&mut buffer) {
        return vec!();
    }

    //.filter(|x| x > &0).collect();
    let v: Vec<u64> = buffer.trim().split(" ").map(|x| 
        match u64::from_str_radix(x, 10) {
            Ok(num) => num,
            Err(_) => 0
        }).filter(|x| x > &0).collect();
    v
}

fn string_sum(stdin: &Stdin, m : u64, k : u64) -> BigUint {
    let mut buffer = String::new();
    let mut handle = stdin.lock();
    
    if let Err(_) = handle.read_line(&mut buffer) {
        return Zero::zero();
    }
    
    let mut words = 0;
    let mut pwr : BigUint = One::one();
    let mut sum : BigUint = Zero::zero();
    for ch in buffer.trim().chars() {
       if !ch.is_whitespace() {
          let base = BigUint::new(vec!(((ch as u8) as u32)));
          
          pwr = pwr * pow(base, m as usize);
       } else {
          words += 1;       
          sum += pwr;
          pwr = One::one();

          //println!("{}", sum.to_string());
       }
    }
    if words + 1 == k {
       sum += pwr;
    }
    //println!("{}", sum.to_string());
    sum
}


/// checks if the expoent is within the valid range
fn valid_expoent(m : u64) -> bool {
    m >= 2 && m <= 10000000000
}

/// checks if the amount of strings within the valid range
fn valid_string_num(m : u64) -> bool {
    m >= 2 && m <= 1000000
}

fn main() {
    let stdin = io::stdin();
    let count = test_cases(&stdin);

    let denomintator = &BigUint::parse_bytes(b"2", 10).unwrap();
    for _ in 0..count {
        let v = elems(&stdin);
        
        if v.len()  == 2 {
            if valid_expoent(v[0]) && valid_string_num(v[1]) {
                if string_sum(&stdin, v[0], v[1]) % denomintator == Zero::zero() {
                    println!("EVEN")
                } else {
                    println!("ODD")
                }
            }
        }
    }
}