use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// Define a special value that will signal termination
const TERMINATION_SIGNAL: i32 = -1;

fn main() {
    // Number of items to produce
    const ITEM_COUNT: usize = 20;
    
    // TODO: Create a channel for sending numbers
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    
    // TODO: Create 2 producer threads
    let mut handles = vec![];
    let num_producers = 2;
    
    for id in 0..num_producers {
        let tx_clone = tx.clone();
        let items_per_producer = ITEM_COUNT / num_producers;
        
        handles.push(thread::spawn(move || {
            producer(id, tx_clone, items_per_producer);
        }));
    }
    
    // TODO: Create 3 consumer threads
    let num_consumers = 3;
    
    for id in 0..num_consumers {
        let rx_clone = Arc::clone(&rx);
        
        handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }
    
    // TODO: Wait for all threads to finish

    // Wait for producers to finish first
    for handle in handles.drain(0..num_producers) {
        handle.join().unwrap();
    }

    // Send termination signals (one per consumer)
    for _ in 0..num_consumers {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for consumers to finish
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("All items have been produced and consumed!");
}

// TODO: Implement producer function
fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    // TODO: Generate random numbers and send them to the channel
    // When finished, producer should NOT send termination signal
    let mut rng = rand::thread_rng();
    
    for i in 0..item_count {
        let num = rng.gen_range(0..100);
        println!("Producer {} generated item {}: {}", id, i, num);
        
        tx.send(num).unwrap();
        
        thread::sleep(Duration::from_millis(100));
    }
    
    println!("Producer {} finished", id);
}

// TODO: Implement consumer function
fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    // TODO: Receive numbers from the channel and process them
    // Break the loop when receiving the termination signal
    loop {
        let value = {
            let lock = rx.lock().unwrap();
            lock.recv().unwrap()
        };
        
        if value == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal", id);
            break;
        }
        
        println!("Consumer {} processing value {}", id, value);
        thread::sleep(Duration::from_millis(150));
    }
    
    println!("Consumer {} exiting", id);
}
