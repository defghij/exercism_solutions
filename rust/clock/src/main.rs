use clock::Clock;

pub fn main(){
    let clock = Clock::new(2, 20).add_minutes(-3000);
    println!("{:?}", clock );
}
