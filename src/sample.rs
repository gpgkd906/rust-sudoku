fn slice_sample () {
    let vec = vec![
        0, 1, 2, 3, 4, 6, 7, 8, 9,
        10, 11, 12, 13, 14, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 26, 27, 28, 29,
        30, 31, 32, 33, 34, 36, 37, 38, 39,
    ];
    let vec = &vec[..10];
    
    println!("{:?}", vec);
}