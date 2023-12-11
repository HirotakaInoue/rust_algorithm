use rand;
mod bubble_sort;
mod selection_sort;

const SIZE: usize = 10;

fn main() {
    bubble_sort_execute();
    selection_sort_execute();
}

fn gen_random_array() -> [i32; SIZE] {
    let mut arr: [i32; SIZE] = [0; SIZE];
    for i in 0..SIZE {
        arr[i] = rand::random::<i32>();
    }
    arr
}

fn bubble_sort_execute() {
    println!("bubble sort execute");
    let mut arr: [i32; SIZE] = gen_random_array();
    println!("bef sort :{:?}", arr);
    bubble_sort::execute(&mut arr);
    println!("aft sort :{:?}", arr);
}

fn selection_sort_execute() {
    println!("selection sort execute");
    let mut arr: [i32; SIZE] = gen_random_array();
    println!("bef sort :{:?}", arr);
    selection_sort::execute(&mut arr);
    println!("aft sort :{:?}", arr);
}
