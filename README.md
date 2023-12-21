# Entro Sort
## Description
Entro Sort is an in-place, linearithmic, unstable sorting algorithm.

## Code Example
The following code demonstrates an example implementation included in this package.

``` rust
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
```

To test the Cargo package, execute the following command.

``` console
cargo test
```

## Purchase
[EntroCraft](https://entrocraft.com/) maintains this open-source package with the permissive MIT license.

It's provided as a convenient code evaluation tool for the [premium Entro Sort library written in C](https://entrocraft.com/dungeon/sorting-algorithms/entro-sort/).

Converting code in this package from Rust to another programming language is discouraged and may be subject to either [purchasing a license](https://entrocraft.com/dungeon/sorting-algorithms/entro-sort/#license) for the premium library in C or attributing other OSI licenses.

Developers who don't use the C programming language can still [purchase credits](https://entrocraft.com/pricing/), learn C and support package maintenance.
