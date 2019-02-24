use rustler::{Env, Term};

mod atoms;
mod miner;

rustler::rustler_export_nifs! {
    "Elixir.Beamcoin.Native",
    [
        ("start", 0, miner::start),
        ("mine", 1, miner::mine),
        ("stop", 1, miner::stop)
    ],
    Some(load)
}

fn load<'a>(env: Env<'a>, _: Term<'a>) -> bool {
    rustler::resource_struct_init!(miner::Channel, env);
    true
}
