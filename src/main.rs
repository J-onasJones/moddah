use std::env;

mod cliargs;
mod errorhandler;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let flags: Vec<&str> = ["-h", "--help", "-v", "--version", "-i", "--install"];
    flagcheck(args, flags);
    flagprocessing(args, flags);
    
}

// NEW CONCEPT CAUSE OLD ONE IS GARBAGE:
// - move args from old to new vector one by one when checked
// - spit out error when leftover elements in old vector
// - process elements of new vector

fn flagcheck(args: Vec<String>, flags: Vec<&str>) 
{
    let mut skip = 1; //first element in flags vector always skipped since it's the scripts directory and therefore garbage data
    for c in args.iter()
    {
        if !flags.contains(&c)
        {
            errorhandler::error(c.as_str());
            std::process::exit(-1);
        } else {
            match c.as_str()
            {
                "-i" => skip += 1
            }
        }
    }
}

fn flagprocessing(args: Vec<String>, flags: Vec<&str>)
{
    let mut skip = 1; //first element in flags vector always skipped since it's the scripts directory and therefore garbage data
    for c in args.iter()
    {
        if skip == 0
        {
            match c.as_str()
            {
                "--help" => cliargs::help(),
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