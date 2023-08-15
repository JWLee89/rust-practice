#[derive(Debug)]
struct Counter {
    length: usize,
    count: usize,
}

impl Counter {
    fn new(length: usize) -> Counter {
        Counter {
            count: 0,
            length
        }
    }
}

impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < self.length {
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let c = Counter::new(10);
    for i in c {
        println!("Ukang: {}", i);
    }
    let powers_of_2: Vec<usize> = Counter::new(8).map(|n| 2usize.pow(n as u32)).collect();
    println!("Powers of 2: {:?}", powers_of_2);
}
