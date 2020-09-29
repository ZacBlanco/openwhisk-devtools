extern crate serde_json;

use serde_derive::{Deserialize, Serialize};
use serde_json::{Error, Value};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Input {
    p1: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Output {
    primes: Vec<u64>,
    notprimes: Vec<u64>,
}

fn isprime(val: &u64) -> bool {
    if (*val) % 2 == 0 {
        return false;
    }
    let cap = *val / 2;
    for i in 2..cap {
        if (*val) % i == 0 {
            return false;
        }
    }
    return true;
}

pub fn main(args: Value) -> Result<Value, Error> {
    let input: Input = serde_json::from_value(args)?;
    let mut primes = vec![];
    let mut notprimes = vec![];
    for i in 0..input.p1 {
        if isprime(&i) {
            primes.push(i);
        } else {
            notprimes.push(i);
        }
    }
    let output = Output {
        primes,
        notprimes,
    };
    serde_json::to_value(output)
}