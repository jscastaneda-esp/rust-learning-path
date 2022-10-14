fn enums() {
    // Instanciar enumeraciones
    let we_load = WebEvent2::WELoad(true);

    let click = MouseClick { x: 100, y: 250 };
    println!("Mouse click location: {}, {}", click.x, click.y);
    let we_click = WebEvent2::WEClick(click);

    let keys = KeyPress(String::from("Ctrl+"), 'N');
    println!("\nKeys pressed: {}{}", keys.0, keys.1);
    let we_key = WebEvent2::WEKeys(keys);

    // # Usar la sintaxis {:#?} para mostrar la estructura del enum
    println!(
        "\nWebEvent enum structure: \n\n {:#?} \n\n {:#?} \n\n {:#?}",
        we_load, we_click, we_key
    );
}

// Enumeradores
// # Forma basica cuando se tiene datos con diferente variacion de tipo, presenta problemas al usarlo, se recomienda usar esta manera cuando los datos sean del mismo tipo
enum WebEvent {
    WELoad,
    WEKeys(String, char),
    WEClick { x: i64, y: i64 },
}

#[derive(Debug)] // Setea el flag Debug para ver los valores durante la ejecucion del codigo en el comando println! se debe usar la sintaxis {:#?} para mostrar el contenido de manera legible
struct KeyPress(String, char);

#[derive(Debug)]
struct MouseClick {
    x: i64,
    y: i64,
}

// # Forma correcta cuando se tiene datos con diferente variacion de tipo
#[derive(Debug)]
enum WebEvent2 {
    WELoad(bool),
    WEClick(MouseClick),
    WEKeys(KeyPress),
}
