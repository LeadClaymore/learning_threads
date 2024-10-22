use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn main() {
    _moving_data_to_thread();
}

// asked chat gpt for a guide on threads
fn _moving_data_to_thread() {
    // data to be sent
    let data = String::from("Hello thread");

    // spawn thread, move moves variables into the thread. Need to look this up more
    let handle = thread::spawn(move || {
        //bc move we can access data
        println!("Data in thread: {}", data);
    });

    // joining the thread gets its stuff
    handle.join().unwrap();
}

//rustanomicon 8.3
fn _three() {
    // man the book did not lie about its posible insanity this stuff is insane.
    // I did not know the existance of weak vs strong ordering
    // the gist is some platforms (x86) are strongly ordered so they have optimizations to strong ordering code
    // and some (arm) are weakly ordered meaning that they are optimized for weak ordering code
    // strong ordering code is like mutexes that simply lock other threads from using the data so it cant have data races
    // also strongly ordering is like setting up a path for the threads to order to, rather then letting them pick
    // where as weak ordering is allows theads to be reordered as they like
}

//rustanomoicon 8.2
fn _two() {
    // this messes with raw pointers and tbh I dont want to / dont want to yet
    // but the general concept is:
    // send = you can pass it to a thread
    // sync = you can pass it to multible threads
    // you normaly cant assign sync or send because things are either send or sync by default
    // and declaring something to be send or sync that was not is unsafe
    // their example is a struct with a sync and send example that they use through raw pointers
}

//rustanomocon 8.1
//tbh I thought there would be more code analysis then reading, this did not need to be code
fn _one() {
    let data = vec![1, 2, 3, 4];
    let idx = Arc::new(AtomicUsize::new(0));
    let other_idx = idx.clone();

    thread::spawn(move || {
        other_idx.fetch_add(10, Ordering::SeqCst);
    });

    println!("{}", data[idx.load(Ordering::SeqCst)]);
}