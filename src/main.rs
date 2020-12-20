use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use rand::Rng;

mod consumer_producer;
use consumer_producer::ConsumerProducer;
use consumer_producer::Capacity;

fn test() {
    let mut buff = consumer_producer::CyclicBuffer::<u8>::new(5);

    println!("CyclicBuffer size is {}", buff.buff_full_capacity());

    let mut  i = 0;
    loop {

        println!("iteration {}", i);
        match buff.write(5) {
            Ok(_) => {
                buff.get_data().into_iter().for_each(|el| {
                        println!("element {}", el)
                    }
                );
                i += 1;
            },
            Err(Capacity::Full) => {
                println!("buffer is full!");
                break;
            },
            Err(_) => panic!("shouldn't have reached here!")
        }
    }

    match i {
        4 => {},
        _ => panic!("amount of writes {} expected 4", i)
    }

    loop {
        match buff.read() {
            Ok(val) => println!("read value {}", val),
            Err(Capacity::Empty) => {
                println!("All values Read");
                break;
            },
            Err(_) => panic!("shouldn't have reached here!")
        }
    }

}

fn multi_thread() {
    let mut buff = consumer_producer::CyclicBuffer::<u32>::new(5);

    let (p_tx, p_rx) = mpsc::sync_channel(0);
    let (c_tx, c_rx) = mpsc::sync_channel(0);

    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        for _ in 1..10 {
            let val = rng.gen::<u32>();
            p_tx.send(val).unwrap();
        }
        //println!("val is {}", val);
    });

    thread::spawn(move || {
        for _ in 1..10 {
            let val = p_rx.recv();
            
            loop {
                match buff.write(val) {
                    Ok(_) => {
                        println!("value {} written to buffer")
                    }
                    Capacity::Full => {
                        println!("buffer is full ")
                    }
                }
            }


        }
    })

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
fn main() {
    test()
}
