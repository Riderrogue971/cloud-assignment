use std::env;
use std::cmp;

pub struct Vector {}

#[derive(Debug, PartialEq, Eq)]
pub struct Interval {
    pub start: i32,
    pub end: i32,
}

impl Interval {
    #[inline]
    pub fn new(start: i32, end: i32) -> Self {
        Interval { start, end }
    }
}

impl Vector {

    pub fn overlap(intervals: Vec<Interval>) -> Vec<Interval> {
        let mut output: Vec<Interval> = Vec::new();
        let mut inter: Vec<Interval> = Vec::new();
        for var in intervals {
            inter.push(var)
        }
        inter.sort_by(|a, b| a.start.cmp(&b.start));

        for interval in inter {
            if output.len() == 0 {
                output.push(interval);
            } else {
                let last = output.last_mut().unwrap();
                if (interval.start >= last.start && interval.start <= last.end) || (interval.start >= last.start && interval.end <= last.end) { // 1-9 and 7-8
                    let interval_new: Interval = Interval::new(cmp::min(last.start, interval.start), cmp::max(last.end, interval.end));
                    
                    output.last_mut().unwrap().start = interval_new.start;
                    output.last_mut().unwrap().end = interval_new.end;
                } else {
                    output.push(interval);
                }
            }
        }
        
        return output;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let intervals = &args[1..];

    let mut input: Vec<Interval> = Vec::new();
    let mut temp: i32 = 0;

    for (pos, e) in intervals.iter().enumerate() {
        if let 1=pos%2{
            input.push(Interval::new(temp, e.parse::<i32>().unwrap()))
        }
        else {
            temp = e.parse::<i32>().unwrap();
        }
    }
    println!("{:?}", Vector::overlap(input));
}
