fn variables() {
    // Variables
    let a_number;
    // println!("{}", a_number); ERROR used binding `a_number` is possibly-uninitialized `a_number` used here but it is possibly-uninitialized

    let a_word = "Ten";

    a_number = 10;
    // a_number = 15; ERROR cannot assign twice to immutable variable

    println!("The a_number is {}.", a_number);
    println!("The a_word is {}.", a_word);

    // Variables mutables (mut)
    let mut a_number_2 = 10;
    println!("The a_number_2 is {}.", a_number_2);

    a_number_2 = 15;
    println!("Now the a_number_2 is {}.", a_number_2);

    // Reemplazo de variables
    let shadow_num = 5;
    let shadow_num = shadow_num + 5;
    let shadow_num = shadow_num * 2;
    println!("The shadow_num is {}.", shadow_num);
}
