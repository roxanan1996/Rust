#[macro_use]
extern crate clap;



fn main() {

    let args = clap_app!(("Command line simple parser") =>
        (author: "Roxana Nicolescu <nicolescu.roxana1996@gmail.com>")
        (about: "Does awesome things")
        (@arg interface: +required "String")
        (@arg arg1: "Option<String>")
        (@arg arg2: "Option<u16>"))
        .get_matches();
    
    println!("{:?}", args.value_of("interface").unwrap());
    println!("{:?}", args.value_of("arg1").map(|s| s ));
    println!("{:?}", args.value_of("arg2").map(|s| s ));

    println!("Hello, world!");
}

