fn functions() {
    // Uso de funciones
    let formal = "Formal: Good bye.";
    let casual = "Casual: See you later!";
    goodbye(formal);
    goodbye(casual);

    let num = 25;
    println!("{} divide by 5 = {}", num, divide_by_5(num));
}

// Funciones
fn goodbye(message: &str) {
    println!("\n{}", message)
}

fn divide_by_5(num: u32) -> u32 {
    if num == 0 {
        return 0;
    }
    num / 5
}
