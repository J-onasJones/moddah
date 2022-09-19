pub(crate) fn error(error: &str)
{
    println!("
ERROR: Couldn't find the given flag '{}'.

Please check the provided flags.

For help, run: 

moddah -h OR moddah --help

", error)
}

pub(crate) fn unexpectederror()
{
    println!("Oof, an unexpected error occured. Either something illegal happened or I did an oopsie.

Either way, this is a fallback to prevent the program from crashing unexpectedly.

Hopefully this won't happen again. Otherwhise please consider writing a bug report on GitHub at

> https://github.com/J-onasJones/moddah/issues/.

Thanks!
    ")
}