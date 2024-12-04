// Used to silence warnings of unused variables
#![allow(unused_variables)]
use simple_types :: {print_difference, print_array, ding, on_off, print_distance};

fn main() {
    let coords: (f32, f32) = (6.3, 15.0);
    let series = [1, 1, 2, 3, 5, 8, 13];
    let mess = ([3,2], 3.14, [(false, -3), (true, -100)], 5, "candy");

    // print the differnce
    let (coords_1, coords_2) = coords;
    print_difference(coords_1, coords_2);
    // this can also be done "print_difference(coords.0, coords.1)"

    // print array out of coords
    let coords_array: [f32; 2] = coords.into();
    print_array(coords_array);

    // Get a "Ding"
    ding(series[6]);

    // Get Lights on
    on_off(mess.2[1].0);

    // organize the mess
    // created a library to house all my functions. 

    print_distance(coords.0, coords.1);
}
