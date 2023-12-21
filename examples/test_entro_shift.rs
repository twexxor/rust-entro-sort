extern crate entro_sort;

use entro_sort::entro_sort::*;

fn main() {
    let mut input1: [u8;10] = [24, 132, 22, 217, 185, 9, 65, 245, 88, 148];
    let mut input2: [u8;10] = [24, 132, 22, 217, 185, 9, 65, 245, 88, 148];
    let mut i: usize = 0;

    entro_sort_ascending(&mut input1);

    while i != input1.len() {
        println!("{}", input1[i]);
        i += 1;
    }

    entro_sort_descending(&mut input2);
    i = 0;

    while i != input2.len() {
        println!("{}", input2[i]);
        i += 1;
    }
}
