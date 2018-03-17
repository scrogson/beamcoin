#![feature(iterator_step_by)]

extern crate easy_hash;
#[macro_use] extern crate lazy_static;
extern crate scoped_pool;
#[macro_use] extern crate rustler;

use easy_hash::{Sha256, Hasher, HashResult};
use scoped_pool::Pool;
use rustler::{Env, Term, Error, Encoder, thread};

use std::sync::Arc;
use std::sync::mpsc::{Sender, channel};
use std::sync::atomic::{AtomicBool, Ordering};

mod atoms;

const BASE: usize = 42;
const THREADS: usize = 4;
static DIFFICULTY: &'static str = "000000";

lazy_static! {
    static ref POOL: Pool = Pool::new(4);
}

rustler_export_nifs! {
    "Elixir.Beamcoin",
    [("native_mine", 0, mine)],
    None
}

struct Solution(usize, String);

fn mine<'a>(caller: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    thread::spawn::<thread::ThreadSpawner, _>(caller, move |env| {
        let is_solved = Arc::new(AtomicBool::new(false));
        let (sender, receiver) = channel();

        for i in 0..THREADS {
            let sender_n = sender.clone();
            let is_solved = is_solved.clone();

            POOL.spawn(move || {
                search_for_solution(i, sender_n, is_solved);
            });
        }

        match receiver.recv() {
            Ok(Solution(i, hash)) => (atoms::ok(), i, hash).encode(env),
            Err(_) => (atoms::error(), "Worker threads disconnected before the solution was found".to_owned()).encode(env),
        }
    });

    Ok(atoms::ok().encode(caller))
}

fn verify_number(number: usize) -> Option<Solution> {
    let hash: String = Sha256::hash((number * BASE).to_string().as_bytes()).hex();
    if hash.ends_with(DIFFICULTY) {
        Some(Solution(number, hash))
    } else {
        None
    }
}

fn search_for_solution(start: usize, sender: Sender<Solution>, is_solved: Arc<AtomicBool>) {
    let mut i = 0;
    for number in (start..).step_by(THREADS) {
        if let Some(solution) = verify_number(number) {
            is_solved.store(true, Ordering::Relaxed);
            match sender.send(solution) {
                Ok(_) => {},
                Err(_) => println!("Receiver has stopped listening, dropping worker number {}", start)
            }
            return;
        } else if i % 1000 == 0 && is_solved.load(Ordering::Relaxed) {
            return;
        }

        i += 1;
    }
}
