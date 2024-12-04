const STARTING_MISSLES: i32 = 8;
const READY_AMT: i32 = 2;


fn main() {
    let mut missles = STARTING_MISSLES;
    
    println!("Firing {} of my {} missles...", READY_AMT, missles);

    missles = missles - READY_AMT;

    println!("{} missles left", missles);
}
