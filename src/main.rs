mod linked_list;
mod block;
mod mresult;
mod addresses;

use std::str::FromStr;
use addresses::Address;
use num_bigint::BigUint;

fn main() {
    println!("Hello, world!");

    let mut address = Address::new();
    address.generate_private_key();

    println!("Address: {:?}", address.address());

    // let x = BigUint::from_str("55066263022277343669578718895168534326250603453777594175500187360389116729240").unwrap();
    // let y = BigUint::from_str("32670510020758816978083085130507043184471273380659243275938904335757337482424").unwrap();
    
    // let p = BigUint::from_str("115792089237316195423570985008687907853269984665640564039457584007908834671663").unwrap();

    // let x_cube = &x * &x * &x;
    // let y_square = &y * &y;

    // println!("(X cube - 7) % p: {}", (x_cube.clone() - 7u8) % p.clone());
    // println!("Y square % p: {}", y_square.clone() % p.clone());

    // let result = (x_cube + (BigUint::from_str("1").unwrap() * 7u8) - y_square) % &p;
    // println!("Result = {}", result);
}
