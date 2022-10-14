fn array_matrix_vector() {
    // Arreglos, matrices y vectores
    // # Arregos y matrices
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let first = days[0];
    let second = days[1];
    // let seventh = days[7]; ERROR index out of bounds: the length is 7 but the index is 7

    let bytes = [0; 5];
    println!("Array initialized with values {:?}, extract especific value array {} y {}, array initialized with type and length {:?}", days, first, second, bytes);

    // # Vectores
    let three_nums = vec![15, 3, 46];
    println!("Initital vector: {:?}", three_nums);

    let zeroes = vec![0; 5];
    // zeroes[0] = 1; ERROR cannot borrow as mutable
    // zeroes.push(1); ERROR cannot borrow as mutable
    println!("Zeroes: {:?}", zeroes);

    // ## Permite mutar el vector en cuanto a tamaño
    let mut fruit = Vec::new();
    fruit.push("Apple");
    fruit.push("Banana");
    fruit.push("Cherry");
    println!("Fruits: {:?}", fruit);
    println!("Pop off: {:?}", fruit.pop());
    println!("Fruits: {:?}", fruit);

    let mut index_vec = vec![15, 3, 46];
    let three = index_vec[1];
    println!("Vector: {:?}, three = {}", index_vec, three);

    index_vec[1] = index_vec[1] + 5;
    println!("Vector: {:?}", index_vec);

    // let beyond = index_vec[10]; ERROR thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 10'
    // println!("{}", beyond);
}
