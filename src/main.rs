
use std::{env, io};
use std::collections::{VecDeque, LinkedList};
use std::time::Instant;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: queue_test num_values");
    } else {
        let num_values = args[1].parse::<usize>().unwrap();
        test(&mut VecDeque::new(), num_values, "VecDeque         ");
        test(&mut LinkedList::new(), num_values, "LinkedList       ");
        test(&mut VecDeque::with_capacity(num_values), num_values, "VecDeque Reserved");
    }
    Ok(())
}

fn test(queue: &mut dyn Q, num_values: usize, label: &str) {
    let mut op_times = Vec::new();
    let start = Instant::now();
    for i in 0..num_values {
        let op_start = Instant::now();
        queue.enqueue(i);
        op_times.push(since(op_start));
    }

    while !queue.is_empty() {
        let op_start = Instant::now();
        queue.dequeue();
        op_times.push(since(op_start));
    }

    let total_time = mu_s_to_s(since(start));
    let median_op = mu_s_to_s(median(&op_times).unwrap());
    let mean_op = mean(&(op_times.iter().map(|t| mu_s_to_s(*t)).collect()));

    println!("For {}: total time: {} s; max single op: {} s; median single op: {}; mean single op: {}",
             label, total_time,
             mu_s_to_s(*(op_times.iter().max().unwrap())),
             median_op, mean_op.unwrap());
}

fn mu_s_to_s(mu_s: u128) -> f64 {
    mu_s as f64 / 1_000_000.0
}

fn since(start: Instant) -> u128 {
    Instant::now().duration_since(start).as_micros()
}

fn median<T:Copy+Ord>(nums: &Vec<T>) -> Option<T> {
    let mut sorted = nums.clone();
    sorted.sort();
    sorted.get(sorted.len() / 2).map(|n| *n)
}

fn mean(nums: &Vec<f64>) -> Option<f64> {
    if nums.len() > 0 {
        Some(nums.iter().sum::<f64>() / nums.len() as f64)
    } else {
        None
    }
}

trait Q {
    fn enqueue(&mut self, value: usize);
    fn dequeue(&mut self) -> Option<usize>;
    fn is_empty(&self) -> bool;
}

impl Q for VecDeque<usize> {
    fn enqueue(&mut self, value: usize) {
        self.push_back(value);
    }

    fn dequeue(&mut self) -> Option<usize> {
        self.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl Q for LinkedList<usize> {
    fn enqueue(&mut self, value: usize) {
        self.push_back(value);
    }

    fn dequeue(&mut self) -> Option<usize> {
        self.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}