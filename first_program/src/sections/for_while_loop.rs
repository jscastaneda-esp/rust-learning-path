fn for_while_loop() {
    // Bucles
    // # loop
    loop {
        println!("We loop forever!");
        break;
    }

    let mut counter_l = 1;
    let stop_loop = loop {
        counter_l *= 2;
        if counter_l > 100 {
            break counter_l;
        }
    };
    println!("Break the loop at counter = {}.", stop_loop);

    // # while
    let mut counter_w = 1;
    while counter_w < 5 {
        println!("We loop a while...");
        counter_w = counter_w + 1;
    }

    // # for
    let big_birds = ["ostrich", "peacock", "stork"];
    // Tambien con big_birds.iter()
    for bird in big_birds {
        println!("The {} is a big bird.", bird);
    }

    for number in 0..5 {
        println!("{}", number * 2);
    }
}
