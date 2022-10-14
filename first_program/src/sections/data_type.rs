fn data_type() {
    // Seteo de tipo de dato
    let number: u32 = 14;
    println!("The number is {}.", number);

    let number_f_64 = 4.0; // Inferido f64 por defecto para numeros flotantes
    let number_f_32: f32 = 5.0;
    println!(
        "The number_f_64 is {} and number_f_32 is {}.",
        number_f_64, number_f_32
    );

    // Operaciones matematicas
    println!(
        "1 + 2 = {} and 8 - 5 = {} and 15 * 3 = {}",
        1u32 + 2,
        8i32 - 5,
        15 * 3
    );
    println!("9 / 2 = {} but 9.0 / 2.0 = {}", 9u32 / 2, 9.0 / 2.0);

    // Booleanos
    let is_bigger = 1 > 4;
    println!("Is 1 > 4? {}", is_bigger);

    // Caracteres y cadenas
    let uppercase_s = 'S';
    let lowercas_f = 'f';
    let smiley_face = '😃';
    println!(
        "The uppercase_s is {}, lowercase_f is {} and smile_face is {}",
        uppercase_s, lowercas_f, smiley_face
    );

    let string_1 = "miley ";
    let string_2: &str = "ace";
    println!(
        "{} is a {}{}{}{}.",
        smiley_face, uppercase_s, string_1, lowercas_f, string_2
    );
}
