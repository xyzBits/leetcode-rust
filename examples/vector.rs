fn main() {
    let vec = vec![1, 2, 3, 4];

    let mut vec = Vec::with_capacity(3);

    for i in 0..8 {
        vec.push(i);
    }

    println!("{:?}", vec);


}