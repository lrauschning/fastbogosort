use std::str::FromStr;
use rip_shuffle::RipShuffleSequential;
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;


fn main(){
    let arr: Vec<isize> = std::env::args().skip(1).map(|x| isize::from_str(&x).expect("Input must be integers!")).collect();
    //println!("{:?}", arr);
    //println!("{}", issorted(&arr));

    let (send, recv) = channel();
    let mut t_killers = Vec::new();

    for _ in 0..8{
        let t_send = send.clone();
        let mut t_arr = arr.clone();

        // do not leave detached threads around
        // (should probably be fine anyway, but
        // cleaner this way)
        let (t_kill_sender, t_kill_recv) = channel(); 
        t_killers.push(t_kill_sender);
        thread::spawn(move || {
            while !issorted(&t_arr){
                t_arr.seq_shuffle(&mut rand::thread_rng());
                // shamelessly stolen from
                // https://stackoverflow.com/questions/26199926/how-to-terminate-or-suspend-a-rust-thread-from-another-thread
                match t_kill_recv.try_recv() {
                    Ok(_) | Err(TryRecvError::Disconnected) => {
                        return;
                    }
                    Err(TryRecvError::Empty) => {}
                }
            }
            t_send.send(t_arr);
        });
    }

    let sorted = recv.recv().unwrap();
    // kill threads
    for killer in t_killers {
        killer.send(());
    }

    println!("{:?}", sorted);
}

fn bogosort(mut arr:Vec<isize>) -> Vec<isize> {
    while !issorted(&arr){
        arr.seq_shuffle(&mut rand::thread_rng());
    }
    arr
}

/// fn to check if an array is sorted
fn issorted(arr: &Vec<isize>) -> bool{
    for i in 1..arr.len(){
        if arr[i] < arr[i-1]{
            return false;
        }
    }
    return true;
}
