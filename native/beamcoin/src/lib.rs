use rustler::Term; // Required to be in scope for the export macro.

mod atoms;
mod miner;

rustler::rustler_export_nifs! {
    "Elixir.Beamcoin.Native",
    [
        ("start", 0, miner::start),
        ("mine", 1, miner::mine),
        ("stop", 1, miner::stop)
    ],
    Some(miner::load)
}
