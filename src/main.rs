use std::str::FromStr;
use rip_shuffle::RipShuffleSequential;
use std::sync::mpsc::channel;
use std::thread;
use std::os::unix::thread::JoinHandleExt; // only works on unix because Rust doesn't support
                                          // killing threads on other platforms.


fn main(){
    let arr: Vec<isize> = std::env::args().skip(1).map(|x| isize::from_str(&x).expect("Input must be integers!")).collect();
    //println!("{:?}", arr);
    //println!("{}", issorted(&arr));

    let (send, recv) = channel();
    let mut threads = Vec::new();

    for _ in 0..8{
        let t_send = send.clone();
        let t_arr = arr.clone();
        threads.push(thread::spawn(move || {
            if let Err(_) = t_send.send(bogosort(t_arr)) {
                panic!("Error in sending result!");
            }
        }));
    }

    // wait for one of the workers to find the right solution, print it
    println!("{:?}", recv.recv().unwrap());

    // cancel threads using libc
    // should be fine not killing them, but cleaner this way
    // using pthread_kill kills the entire process for some reason
    unsafe {
        for thread in threads {
            libc::pthread_cancel(thread.into_pthread_t());
        }
    }
}

fn bogosort(mut arr:Vec<isize>) -> Vec<isize> {
    while !issorted(&arr){
        arr.seq_shuffle(&mut rand::thread_rng());
    }
    arr
}

/// fn to check if an array is sorted
fn issorted(arr: &Vec<impl Ord>) -> bool{
    for i in 1..arr.len(){
        if arr[i] < arr[i-1]{
            return false;
        }
    }
    return true;
}
