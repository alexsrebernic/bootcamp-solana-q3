fn main() {
    let mut array = [1, 2, 3, 4,];
    // MY PART
    // add your code here 
    let array_length = array.len();
    let mut count = 0usize;

    while count < array_length / 2 {
        let temp = array[count];
      array[count] = array[array_length - 1 - count];
      array[array_length - 1 - count] = temp;
      count += 1;

    }
    println!("{:?}", array);
}