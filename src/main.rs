mod dyn_array;
use dyn_array::DynArray;
use std::cmp;

fn main() {
    let dims = get_dims();
    let mut array = DynArray::new([dims.0, dims.1], '.');

    let (w, h) = (array.dims()[0], array.dims()[1]);
    for i in 0..cmp::min(w, h) {
        array[[i, i]] = '@';
        array[[w - i - 1, i]] = '#';
        array[[i, h - i - 1]] = '$';
        array[[w - i - 1, h - i - 1]] = '&';
    }

    print_arr(&array);
}

fn get_dims() -> (usize, usize) {
    use std::io::stdin;
    let mut input = String::new();
    
    println!("Please a width for the board: ");
    stdin().read_line(&mut input).unwrap();
    let w = input.trim().parse().expect("Failed to parse width");

    input.clear();
    println!("Please a height for the board: ");
    stdin().read_line(&mut input).unwrap();
    let h = input.trim().parse().expect("Failed to parse height");

    (w, h)
}

fn print_arr(arr: &DynArray<char, 2>) {
    for y in 0..arr.dims()[1] {
        for x in 0..arr.dims()[0] {
            print!("{} ", arr[[x, y]]);
        }
        println!();
    }
}
