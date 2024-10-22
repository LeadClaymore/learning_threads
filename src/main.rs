use std::thread;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
//use std::time::Duration;
use std::sync::mpsc;
fn main() {
    _shared_state();
}

fn _shared_state() {
    // this is an atomic data type (Arc) with a mutex and an i32 in it;
    // the arc is atomic meaning when it runs it does strong ordering when passing data between threads (see three)
    // the mutex is part of the atomic that locks the data to a thread when in use
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // I never knew that data within a scope could be reasigned and still used by the thread
        // what I mean is there will be 10 instances of counter 1 within each scope of the thread
        // but either way this clones the counter but its an atomic pointer so its the same one in use
        // each of the clones are going into the thread to be used
        let counter = Arc::clone(&counter);

        let handle = thread::spawn( move || {
            // you need to lock() the mutex so it wont be used by other threads
            // the unwrap is because its not 100% safe meaning it might panic
            // either way this creates a mutatable i32 that can be used by any number of threads
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        //push the thread handle for joining
        handles.push(handle);
    }

    // for each of the threads join them unwrap because its a result, it could fail
    for handle in handles {
        handle.join().unwrap();
    }
    //print the final value, cause mutexes you still have to lock it and unwrap
    println!("Result: {}", *counter.lock().unwrap());
}

// again guide from chatgpt
// this was meant to be passing data between threads
// but it came out as sending data back to the main thread
fn _passing_data_between_threads() {
    // the multi sender single reciever channels were gone over in rustlings, 
    // you can clone the sender if needed to have more threads send to the reciever
    // thats the next part of the guide, ima just skip it
    // also it complains before it knows what it will be sending, but after its fine
    let (tx, rx) = mpsc::channel();

    // same thread with the move to send tx into it
    thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        //this sends the messages in messages to rx
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    // this recieves the messages.
    // for each of the messages in rx it takes the sent data and prints it
    for received in rx {
        println!("Received: {}", received);
    }
}

// asked chat gpt for a guide on threads
fn _moving_data_to_thread() {
    // data to be sent
    let data = String::from("Hello thread");

    // spawn thread, move moves variables into the thread. Need to look this up more
    let handle = thread::spawn(move || {
        //this is to test if there are problems with threads taking longer then the main thread
        //there arnt at least like this, it waited the 10 then ended
        //thread::sleep(std::time::Duration::from_secs(10));

        //bc move we can access data
        println!("Data in thread: {}", data);
    });

    //test to see if the handle join causes or simply gets the thread
    //it printed then waited 10 sec so its doing the thread in the background while waiting
    //thread::sleep(std::time::Duration::from_secs(10));

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