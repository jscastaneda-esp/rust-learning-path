fn process_s(input: String) {}
fn process_n(input: u32) {}

fn print_greeting(message: &String) {
    println!("Greeting: {}", message);
}

fn change_1(message: &String) {
    // message.push_str("!"); // We try to add a "!" to the end of our message
}

fn change_2(text: &mut String) {
    text.push_str(", world");
}

fn longest_word<'a>(x: &'a String, y: &'a String) -> &'a String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[derive(Debug)]
struct Highlight<'document>(&'document str);

fn erase(_: String) {}

fn rust_memory_management() {
    // Propiedad y transferencia
    {
        let mascot = String::from("ferris");
        let ferris = mascot;
        // println!("{}", mascot) // We'll try to use mascot after we've moved ownership of the string data from mascot to ferris.
    }
    // println!("{}", mascot); ERROR cannot find value `mascot` in this scope not found in this scope

    let s = String::from("Hello, world!");
    process_s(s); // Ownership of the string in `s` moved into `process`
                  // process_s(s); // Error! ownership already moved.

    let n = 1u32;
    process_n(n); // Ownership of the number in `n` copied into `process`
    process_n(n); // `n` can be used again because it wasn't moved, it was copied.

    let s = String::from("Hello, world!");
    process_s(s.clone()); // Passing another value, cloned from `s`.
    process_s(s); // s was never moved and so it can still be used.

    // Prestamo
    let mut greeting = String::from("hello");
    let greeting_reference = &greeting; // We borrow `greeting` but the string data is still owned by `greeting`
    println!("Greeting: {}", greeting); // We can still use `greeting`

    print_greeting(&greeting); // `print_greeting` takes a `&String` not an owned `String` so we borrow `greeting` with `&`
    greeting = String::from("Bye!");
    print_greeting(&greeting);
    change_1(&greeting);
    change_2(&mut greeting);
    print_greeting(&greeting);

    // Duraciones
    let x: &i32 = &0;
    {
        let y = 42;
        // x = &y; // We store a reference to `y` in `x` but `y` is about to be dropped.
    }
    println!("x: {}", x); // `x` refers to `y` but `y has been dropped!

    // let x;                // ---------+-- 'a
    // {                     //          |
    //     let y = 42;       // -+-- 'b  |
    //     x = &y;           //  |       |
    // }                     // -+       |
    // println!("x: {}", x); //          |

    let magic1 = String::from("abracadabra!");
    let magic2 = String::from("shazam!");

    let result = longest_word(&magic1, &magic2);
    println!("The longest magic word is {}", result);

    let magic1 = String::from("abracadabra!");
    let result: &String = &String::from("");
    {
        let magic2 = String::from("shazam!");
        // result = longest_word(&magic1, &magic2);
    }
    println!("The longest magic word is {}", result);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);

    // erase(text); // move out of `text` occurs here

    println!("{:?}", fox);
    println!("{:?}", dog);
}
