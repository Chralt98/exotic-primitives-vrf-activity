use rand::{Rng, rngs::OsRng};
use schnorrkel::Keypair;
use schnorrkel::Signature;

fn main() {
    let keypair_player_0: Keypair = Keypair::generate_with(OsRng);
    let keypair_player_1: Keypair = Keypair::generate_with(OsRng);
}
