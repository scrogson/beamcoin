use crate::atoms;
use mpsc::{Receiver, Sender};
use rustler::{Atom, Encoder, Env, Error, LocalPid, OwnedEnv, ResourceArc, Term};
use scoped_pool::Pool;
use sha2::Digest;
use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex};

static DIFFICULTY: usize = 6;

lazy_static::lazy_static! {
    static ref POOL: Pool = Pool::new(num_cpus::get());
}

#[derive(Debug)]
struct Solution(u64, String);

pub struct Channel(Mutex<Sender<Message>>);

enum Message {
    Mine(LocalPid),
}

pub fn load(env: Env, _: Term) -> bool {
    rustler::resource!(Channel, env);
    true
}

#[rustler::nif]
pub fn start() -> Result<(Atom, ResourceArc<Channel>), Error> {
    let (tx, rx) = mpsc::channel::<Message>();

    std::thread::spawn(move || thread_pool(rx));

    let resource = ResourceArc::new(Channel(Mutex::new(tx)));
    Ok((atoms::ok(), resource))
}

#[rustler::nif]
pub fn mine(env: Env, resource: ResourceArc<Channel>) -> Result<Atom, Error> {
    let lock = resource.0.lock().unwrap();
    lock.send(Message::Mine(env.pid())).unwrap();

    Ok(atoms::ok())
}

#[rustler::nif]
pub fn stop(resource: ResourceArc<Channel>) -> Result<Atom, Error> {
    drop(resource);

    Ok(atoms::ok())
}

fn thread_pool(mailbox: Receiver<Message>) {
    let mut env = OwnedEnv::new();

    loop {
        match mailbox.recv() {
            Ok(Message::Mine(pid)) => {
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
                    Ok(Solution(i, hash)) => {
                        env.send_and_clear(&pid, |env| (atoms::ok(), i, hash).encode(env))
                    }
                    Err(_) => {
                        let msg =
                            "Worker threads disconnected before the solution was found".to_owned();
                        env.send_and_clear(&pid, |env| (atoms::error(), msg).encode(env))
                    }
                }
            }
            Err(_) => {
                println!("sender dropped...exiting");
                POOL.shutdown();
                break;
            }
        }
    }
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
