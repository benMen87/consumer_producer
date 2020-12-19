mod consumer_producer;

use consumer_producer::ConsumerProducer;
use consumer_producer::Capacity;

fn main() {
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
