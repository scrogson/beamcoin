mod atoms;
mod miner;

rustler::init! {
    "Elixir.Beamcoin.Native",
    [
        miner::start,
        miner::mine,
        miner::stop
    ],
    load = miner::load
}
