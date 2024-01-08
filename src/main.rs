use std::sync::{Arc, Mutex, mpsc};
use std::{fmt, thread};
use std::time::Duration;
use rand::Rng;

#[derive(Debug)]
struct Prices {
    tobacco: u32,
    paper: u32,
    matches: u32,
}

impl fmt::Display for Prices {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Tobacco: {}, Paper: {}, Matches: {})", self.tobacco, self.paper, self.matches)
    }
}
fn main() {
    // prices of ingredients
    let prices = Arc::new(Mutex::new(Prices {
        tobacco: 0,
        paper: 0,
        matches: 0,
    }));

    // money of smokers
    let money_of_smoker_with_tobacco = Arc::new(Mutex::new(30));
    let money_of_smoker_with_paper = Arc::new(Mutex::new(30));
    let money_of_smoker_with_matches = Arc::new(Mutex::new(30));

    // create channels for sending money
    let (paper_to_tobacco, receive_tobacco) = mpsc::channel::<u32>();
    let matches_to_tobacco = paper_to_tobacco.clone();

    let (matches_to_paper, receive_paper) = mpsc::channel::<u32>();
    let tobacco_to_paper = matches_to_paper.clone();

    let (tobacco_to_matches, receive_matches) = mpsc::channel::<u32>();
    let paper_to_matches = tobacco_to_matches.clone();

    let prices1 = Arc::clone(&prices);
    let agent = thread::spawn(move || {
        loop {
            {
                let mut price = prices1.lock().unwrap();

                let mut rng = rand::thread_rng();
                // generate random prices
                price.tobacco = rng.gen_range(1..10);
                price.paper = rng.gen_range(1..10);
                price.matches = rng.gen_range(1..10);

                println!("\nCurrent prices: {}\n", price);
            }
            thread::sleep(Duration::from_millis(10000));
        }
    });

    let prices1 = Arc::clone(&prices);
    let smoker_with_tobacco = thread::spawn(move || {
        let mut money = money_of_smoker_with_tobacco.lock().unwrap();
        loop {
            while let Ok(payment) = receive_tobacco.try_recv() {
                println!("Tobacco received money: {}", payment);
                *money += payment;
            }

            {
                let prices = prices1.lock().unwrap();
                if prices.matches + prices.paper > *money {
                    println!("Tobacco couldn't buy ingredients");
                } else {
                    *money -= prices.paper + prices.matches;

                    println!("Tobacco bought ingredients for: {}, current money: {}", prices.paper + prices.matches, money);

                    // send money for ingredients
                    tobacco_to_paper.send(prices.paper).unwrap();
                    tobacco_to_matches.send(prices.matches).unwrap();
                }
            }
            thread::sleep(Duration::from_millis(5000));
        }
    });

    let prices1 = Arc::clone(&prices);
    let smoker_with_paper = thread::spawn(move || {
        loop {
            let mut money = money_of_smoker_with_paper.lock().unwrap();
            while let Ok(payment) = receive_paper.try_recv(){
                println!("Paper received money: {}", payment);
                *money += payment;
            }

            {
                let prices = prices1.lock().unwrap();
                if prices.tobacco + prices.matches > *money {
                    println!("Paper couldn't buy ingredients");
                } else {
                    *money -= prices.tobacco + prices.matches;

                    println!("Paper bought ingredients for: {}, current money: {}", prices.matches + prices.tobacco, money);

                    // send money for ingredients
                    paper_to_matches.send(prices.matches).unwrap();
                    paper_to_tobacco.send(prices.tobacco).unwrap();
                }
            }
            thread::sleep(Duration::from_millis(5000));
        }
    });

    let prices1 = Arc::clone(&prices);
    let smoker_with_matches = thread::spawn(move || {
        loop {
            let mut money = money_of_smoker_with_matches.lock().unwrap();

            while let Ok(payment) = receive_matches.try_recv() {
                println!("Matches received money: {}", payment);
                *money += payment;
            }

            {
                let prices = prices1.lock().unwrap();

                if prices.tobacco + prices.paper > *money {
                    println!("Matches couldn't buy ingredients");
                } else {
                    *money -= prices.tobacco + prices.paper;
                    println!("Matches bought ingredients for: {}, current money: {}", prices.paper + prices.tobacco, money);

                    // send money for ingredients
                    matches_to_paper.send(prices.paper).unwrap();
                    matches_to_tobacco.send(prices.tobacco).unwrap();
                }
            }
            thread::sleep(Duration::from_millis(5000));
        }
    });

    agent.join().unwrap();
    smoker_with_paper.join().unwrap();
    smoker_with_tobacco.join().unwrap();
    smoker_with_matches.join().unwrap();
}
