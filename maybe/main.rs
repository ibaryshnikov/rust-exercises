#![feature(try_trait)]

use std::fmt::Debug;
use std::ops::Try;

#[derive(Debug)]
struct NothingError;

#[derive(Debug)]
enum Maybe<T: Debug> {
    Just(T),
    Nothing,
}
use Maybe::{Just, Nothing};

impl<T: Debug> Try for Maybe<T> {
    type Ok = T;
    type Error = NothingError;
    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            Just(value) => Ok(value),
            Nothing => Err(NothingError)
        }
    }
    fn from_error(_e: Self::Error) -> Self {
        Nothing
    }
    fn from_ok(value: Self::Ok) -> Self {
        Just(value)
    }
}

fn get_data(i: i32) -> Maybe<i32> {
    if i > 5 {
        Just(i)
    } else {
        Nothing
    }
}

fn middle_fn(i: i32) -> Maybe<i32> {
    let inner = get_data(i)?;
    println!("{:?}", inner);
    Just(inner)
}

fn main() {
    let data1 = middle_fn(3);
    println!("{:?}", data1);
    let data2 = middle_fn(10);
    println!("{:?}", data2);
}
