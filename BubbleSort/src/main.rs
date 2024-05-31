use rand::prelude::*;

fn bubble_sort(array: &mut [i32]){
    for i in (0..array.len()).rev() {
        for j in (1..i).rev(){
            if array[j] < array[j-1] {    
                (array[j], array[j-1]) = (array[j-1], array[j]);
            }
        }
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut array = [0;12];

    for i in 0..array.len(){
        array[i] = rng.gen_range(0..1000);
    }

    println!("Array unsorted: {:?}", array);

    bubble_sort(&mut array);

    println!("Array sorted: {:?}", array);
}
