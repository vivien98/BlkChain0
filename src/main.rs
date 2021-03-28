use ring::digest::{digest};
use serde::{Serialize,Deserialize};

#[allow(non_snake_case)]
#[derive(Serialize,Deserialize,Debug)]
struct NameHash{
	name: String,
	hash: String,
}

#[allow(non_snake_case)]
fn main() {
    let my_name = String::from("Viraj");
    let encoded = my_name.as_bytes();
    println!("{}",my_name );
    print!("{:?} \n",encoded );
    let hash = digest(&ring::digest::SHA256,encoded);
    println!("{:?}",hash );
    let hashInHex = hex::encode(hash);
    println!("hashInHex is {}",hashInHex );

    let namehash: NameHash = NameHash{name : my_name, hash : hashInHex};

    let serialEnc = bincode::serialize(&namehash).unwrap();
    let serialDec: NameHash = bincode::deserialize(&serialEnc).unwrap();

    println!("{:?}",serialEnc);
    println!("{:?}",serialDec );

}
