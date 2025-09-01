// use rand::Rng;
use rand::seq::SliceRandom;

fn main() {

    for i in 0..20 {
        let alphbet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let mut message = String::new();

        for _ in 0..(i + 1) {
            message.push_str(alphbet);
        }

        // println!("{}", message);


        // let mut rng = rand::rng();
        // let mut bytes = "".to_string().into_bytes();
        // bytes.shuffle(&mut rng);
        // let str = String::from_utf8(bytes).unwrap();



    }
}
