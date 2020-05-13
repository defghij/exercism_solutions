mod lib;
use lib::factors;


fn main(){  
    let f: Vec<u64> = factors(2);
    for x in f.iter() {
        println!("{}", x);
    }
}
