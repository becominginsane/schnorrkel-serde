use schnorrkel::{Keypair, PublicKey};

fn main() {
    let keypair = Keypair::generate();

    let public = serde_json::to_value(keypair.public).expect("unable to serialize PublicKey");

    println!("public_key: {:?}", public);

    let public: PublicKey =
        serde_json::from_value(public).expect("unable to deserialize PublicKey");

    println!("public_key: {:x?}", public.to_bytes());
}
