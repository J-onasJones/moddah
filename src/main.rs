use std::env;

mod cliargs;
mod errorhandler;

fn main() 
{
    // pub(crate) const VERSION: &str = env!("CARGO_PKG_VERSION");
    // pub(crate) const OS: &str = env::consts::OS;
    // pub(crate) const ARCHITECTURE : &str = env::consts::ARCH;
    let args: Vec<String> = env::args().collect();
    let flags: Vec<&str> = ["-h", "--help", "-v", "--version", "-i", "--install", "-u", "--uninstall"].to_vec();
    flagcheck(&args, &flags);
    flagprocessing(args);
    
}

// NEW CONCEPT CAUSE OLD ONE IS GARBAGE:
// - move args from old to new vector one by one when checked
// - spit out error when leftover elements in old vector
// - process elements of new vector

fn flagcheck(args: &Vec<String>, flags: &Vec<&str>) 
{
    let mut skip = 1; //first element in flags vector always skipped since it's the scripts directory and therefore garbage data
    for c in args.iter()
    {
        if skip < 1
        {
            if !flags.contains(&c.as_str())
            {
                errorhandler::error(c.as_str());
                std::process::exit(-1);
            } else {
                match c.as_str()
                {
                    "-i" => skip += 5,
                    &_ => continue

                }
            }
        } else {
            skip -= 1;
        }
    }
}

fn flagprocessing(args: Vec<String>)
{
    let mut skip = 1; //first element in flags vector always skipped since it's the scripts directory and therefore garbage data
    for c in args.iter()
    {
        if skip == 0
        {
            match c.as_str()
            {
                "--help" => {cliargs::help(); std::process::exit(0);},
                "-h" => {cliargs::help(); std::process::exit(0);},
                "--version" => cliargs::version(),
                "-v" => cliargs::version(),
                "--install" => cliargs::install(),
                "-i" => cliargs::install(),
                "--uninstall" => cliargs::uninstall(),
                "-u" => cliargs::uninstall(),
                _ => {
                    errorhandler::unexpectederror();
                    std::process::exit(-1);
                },
            }
        } else {
            skip -= 1;
        }
    }
}