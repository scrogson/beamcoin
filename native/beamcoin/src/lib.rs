#[macro_use]
extern crate lazy_static;
extern crate num_cpus;
extern crate scoped_pool;
extern crate sha2;
#[macro_use]
extern crate rustler;

use rustler::{thread, Encoder, Env, Error, Term};
use scoped_pool::Pool;
use sha2::Digest;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};

mod atoms;

static DIFFICULTY: usize = 6;
lazy_static! {
    static ref POOL: Pool = Pool::new(num_cpus::get());
}

rustler_export_nifs! {
    "Elixir.Beamcoin",
    [("native_mine", 0, mine)],
    None
}

#[derive(Debug)]
struct Solution(u64, String);

fn mine<'a>(caller: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    thread::spawn::<thread::ThreadSpawner, _>(caller, move |env| {
        let is_solved = Arc::new(AtomicBool::new(false));
        let (sender, receiver) = mpsc::channel();

        for i in 0..num_cpus::get() {
            let sender_n = sender.clone();
            let is_solved = is_solved.clone();

            POOL.spawn(move || {
                search_for_solution(i as u64, num_cpus::get() as u64, sender_n, is_solved);
            });
        }

        match receiver.recv() {
            Ok(Solution(i, hash)) => (atoms::ok(), i, hash).encode(env),
            Err(_) => (
                atoms::error(),
                "Worker threads disconnected before the solution was found".to_owned(),
            ).encode(env),
        }
    });

    Ok(atoms::ok().encode(caller))
}

fn verify_number(number: u64) -> Option<Solution> {
    let number_bytes: [u8; 8] = unsafe { mem::transmute::<u64, [u8; 8]>(number) };
    let hash = sha2::Sha256::digest(&number_bytes);

    let top_idx = hash.len() - 1;
    let trailing_zero_bytes = DIFFICULTY / 2; // Hex chars are 16 bits, we have
                                              // 8 bits. /2 is conversion.
    let mut jackpot = true;
    for i in 0..trailing_zero_bytes {
        jackpot &= hash[top_idx - i] == 0;
    }

    if jackpot {
        Some(Solution(number, format!("{:X}", hash)))
    } else {
        None
    }
}

fn search_for_solution(
    mut number: u64,
    step: u64,
    sender: mpsc::Sender<Solution>,
    is_solved: Arc<AtomicBool>,
) -> () {
    let id = number;
    while !is_solved.load(Ordering::Relaxed) {
        if let Some(solution) = verify_number(number) {
            if let Ok(_) = sender.send(solution) {
                is_solved.store(true, Ordering::Relaxed);
                break;
            } else {
                println!("Worker {} has shut down without finding a solution.", id);
            }
        }
        number += step;
    }
}
