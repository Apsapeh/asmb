use std::env;

mod build;
mod config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Too few arguments");
        return;
    }
    
    let config = config::Config::from_file("asmb.toml");
    println!("Config: {:?}", config);
    
    match args[1].as_str() {
        "r" | "run" => println!("Hello, world!"),
        "b" | "build" => config.build(),
        "x" | "br" | "buildrun" => {
            config.build();
            //println!("Goodbye, world!");
        },
        _ => println!("Unknown command"),
    }
}
