use rand;
mod bubble_sort;

fn main() {
    bubble_sort_execute();
}

fn gen_random_array() -> [i32;10]{
    let mut arr = [0;10];
    for i in 0..10{
        arr[i] = rand::random::<i32>();
    }
    arr
}

fn bubble_sort_execute(){
    let mut arr = gen_random_array();
    println!("bef sort :{:?}",arr);
    bubble_sort::execute(&mut arr);
    println!("aft sort :{:?}",arr);
}
