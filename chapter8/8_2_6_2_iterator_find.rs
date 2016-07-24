fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("Find 2 in vec1: {:?}", vec1.iter().find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", vec2.into_iter().find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&&x| x == 2));
}
