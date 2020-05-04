use std::{
    thread,
    sync::{Arc, Mutex},
    time::Duration
};
use padlock;

#[derive(Clone, Debug)]
struct Person {
    age: u8,
    name: String
}

fn main() {

    let people = Arc::new(Mutex::new(Vec::<Person>::new()));
    let mut threads = Vec::<thread::JoinHandle<()>>::new();

    // Write in one thread
    let people_clone = Arc::clone(&people);
    threads.push(thread::spawn(move || {

        for i in 0..10 {

            padlock::mutex_lock(&people_clone, |lock| {

                lock.push(Person {
                    age: i * 10,
                    name: format!("Name {}", i)
                });

            });

            thread::sleep(Duration::from_millis(500));

        }

    }));

    // Read from another
    let people_clone = Arc::clone(&people);
    threads.push(thread::spawn(move || {

        for _ in 0..6 {

            padlock::mutex_lock(&people_clone, |lock| {

                for person in lock {
                    println!("{:?}", person);
                }

            });

            thread::sleep(Duration::from_secs(1));

        }

    }));

    for t in threads {
        drop(t.join());
    }

}
