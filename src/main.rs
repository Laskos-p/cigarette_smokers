use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;
use rand::Rng;

#[derive(Debug)]
struct Prices {
    tobacco: u32,
    paper: u32,
    matches: u32,
}

fn main() {

    // prices of ingredients [tobacco, paper, matches]
    let prices = Arc::new(Mutex::new(Prices {
        tobacco: 0,
        paper: 0,
        matches: 0,
    }));

    // money of smokers
    let money_of_smoker_with_tobacco = Arc::new(Mutex::new(100));
    let money_of_smoker_with_paper = Arc::new(Mutex::new(100));
    let money_of_smoker_with_matches = Arc::new(Mutex::new(100));

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

                // generate random prices
                let mut rng = rand::thread_rng();
                price.tobacco = rng.gen_range(1..10);
                price.paper = rng.gen_range(1..10);
                price.matches = rng.gen_range(1..10);
                println!("Current prices: {:?}", price);
            }
            thread::sleep(Duration::from_millis(2000));
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

                    println!("Tobacco bought ingredients, current money: {}", money);

                    // send money for ingredients
                    tobacco_to_paper.send(prices.paper).unwrap();
                    tobacco_to_matches.send(prices.matches).unwrap();
                    println!("Smoker tobacco sent money: {}", prices.paper + prices.matches);

                    // print!("Smoker tobacco is smoking");
                }
            }
            thread::sleep(Duration::from_millis(2000));

        }
    });

    let prices1 = Arc::clone(&prices);
    let smoker_with_paper = thread::spawn(move || {
        loop {
            let mut money = money_of_smoker_with_paper.lock().unwrap();
            while let Ok(payment) = receive_paper.try_recv(){
                // println!("Smoker paper received money: {}", payment);
                *money += payment;
            }

            {
                let prices = prices1.lock().unwrap();
                if prices.tobacco + prices.matches > *money {
                    println!("Paper couldn't buy ingredients");
                } else {
                    *money -= prices.tobacco + prices.matches;

                    println!("Paper bought ingredients, current money: {}", money);

                    // send money for ingredients
                    paper_to_matches.send(prices.matches).unwrap();
                    paper_to_tobacco.send(prices.tobacco).unwrap();

                    println!("Paper sent money for tobacco: {}", prices.tobacco);
                }
            }

            thread::sleep(Duration::from_millis(2000));
        }
    });

    let prices1 = Arc::clone(&prices);
    let smoker_with_matches = thread::spawn(move || {
        loop {
            let mut money = money_of_smoker_with_matches.lock().unwrap();

            while let Ok(payment) = receive_matches.try_recv() {
                // println!("Smoker matches received money: {}", payment);
                *money += payment;
            }

            // *money += receive_matches.recv().unwrap();
            {
                let prices = prices1.lock().unwrap();

                if prices.tobacco + prices.paper > *money {
                    println!("Matches couldn't buy ingredients");
                } else {
                    *money -= prices.tobacco + prices.paper;

                    println!("Matches bought ingredients, current money: {}", money);
                    // send money for ingredients
                    matches_to_paper.send(prices.paper).unwrap();
                    matches_to_tobacco.send(prices.tobacco).unwrap();
                    println!("Matches sent money for tobacco: {}", prices.tobacco)

                }
            }
            thread::sleep(Duration::from_millis(2000));
        }
    });

    agent.join().unwrap();
    smoker_with_paper.join().unwrap();
    smoker_with_tobacco.join().unwrap();
    smoker_with_matches.join().unwrap();

}
