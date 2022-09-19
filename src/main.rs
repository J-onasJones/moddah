use std::env;

mod cliargs;

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in args.iter()
    {
        match i.as_str()
        {
            "--help" => cliargs::help(),
            _ => {
                if !args.iter().position(|r| r.as_str() == i).unwrap().parse() == 0
                {
                    println!("Error")
                } else {
                    println!("{}", args.iter().position(|r| r.as_str() == i).unwrap());
                }
            },
        }
    }
}