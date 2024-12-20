// Used to silence warnings of unused variables
#![allow(unused_variables)]

fn main() {
    let width = 4;
    let height = 7;
    let debth = 10;

    let area = area_of(width, height);
    let volume = volume_of(width, height, debth);

    
    println!("Area is {}", area);
    println!("Volume is {}", volume);

}

fn area_of(x: i32, y: i32) -> i32{
    x * y
}

fn volume_of(x: i32, y: i32, z: i32) -> i32{
    x * y * z
}