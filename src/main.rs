#![feature(test)]
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
    println!("{:?}", par_bogosort(&arr, 8));
}

fn par_bogosort(arr:&Vec<isize>, no_threads:usize) -> Vec<isize>{
    let (send, recv) = channel();
    let mut threads = Vec::new();

    for _ in 0..no_threads{
        let t_send = send.clone();
        let t_arr = arr.clone();
        threads.push(thread::spawn(move || {
            if let Err(x) = t_send.send(bogosort(t_arr)) {
                println!("Error in sending result: {:?}!", x);
            }
        }));
    }

    // wait for one of the workers to find the right solution, print it
    let sorted = recv.recv().unwrap();

    // cancel threads using libc
    // should be fine not killing them, but cleaner this way
    // using pthread_kill kills the entire process for some reason
    unsafe {
        for thread in threads {
            libc::pthread_cancel(thread.into_pthread_t());
        }
    }
    sorted
}

fn bogosort<T: Ord>(mut arr:Vec<T>) -> Vec<T> {
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


#[cfg(test)]
mod tests{
    extern crate test;
    use test::Bencher;
    use super::*;

    #[test]
    fn issorted_true(){
        let arr = vec!(1, 2, 3);
        assert!(issorted(&arr));
    }

    #[test]
    fn issorted_rand(){
        let arr = vec!(2, 1, 3);
        assert!(!issorted(&arr));
    }

    #[test]
    fn issorted_inv(){
        let arr = vec!(3, 2, 1);
        assert!(!issorted(&arr));
    }

    #[test]
    fn bogosort_sorted(){
        let arr = vec!(1, 2, 3);
        assert_eq!(bogosort(arr.clone()), arr)
    }

    #[test]
    fn bogosort_unsorted(){
        let arr = vec!(3, 1, 2);
        assert_eq!(bogosort(arr.clone()), vec!(1, 2, 3))
    }

    #[test]
    fn par_bogosort_sorted(){
        let arr = vec!(1, 2, 3);
        assert_eq!(par_bogosort(&arr, 4), arr)
    }

    #[test]
    fn par_bogosort_unsorted(){
        let arr = vec!(3, 1, 2);
        assert_eq!(par_bogosort(&arr, 4), vec!(1, 2, 3))
    }

    #[bench]
    fn par10(b: &mut Bencher){
        let arr = vec!(9, 8, 7, 6, 5, 4, 3, 2, 1, 0);
        b.iter(|| par_bogosort(&arr, 8))
    }

    #[bench]
    fn par5(b: &mut Bencher){
        let arr = vec!(4, 3, 2, 1, 0);
        b.iter(|| par_bogosort(&arr, 8))
    }

    #[bench]
    fn par3(b: &mut Bencher){
        let arr = vec!(2, 1, 0);
        b.iter(|| par_bogosort(&arr, 8))
    }
}
