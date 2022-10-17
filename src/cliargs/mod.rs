use std::env;

pub(crate) fn help()
{
    println!("Moddah is a managing tool for Minecraft modded and plugin servers.
The supported toolchains are FabricMC, ForgeModLoader, QuiltMC, SpigotMC.

Usage:
    moddah
    moddah <operation> [...]

Operations:
    moddah {{-h --help}}                  print this help message
    moddah {{-v --version}}               print the version of Moddah
    moddah {{-l --list}}          [...]   list versions
    moddah {{-i --install}}       [...]   install a Modloader

Options:
    moddah {{-l --list}} [...]
        - minecraft                     list all minecraft versions
        - fabric                        list all minecraft versions supported by the FabricMC toolchain
        - forge                         list all minecraft versions supported by the ForgeModLoader toolchain
        - quilt                         list all minecraft versions supported by the QuiltMC toolchain

    moddah {{-i --install}} [modloader] [minecraft-version] [environment] [loader-version] [directory]
        [modloader]:
            - fabric
            - forge
            - quilt
        
        [minecraft-version]:
            - [any version that is supported by the modloader of choice]
        
        [environment]:
            - client
            - server
        
        [loader-version]:
            - [any version of the loader of choice]
        
        [directory]:
            - [leave blank for current directory]
            - [directory of choice]
    ")
}

pub(crate) fn version()
{
    println!("Moddah version {}-{}({})", env!("CARGO_PKG_VERSION"), env::consts::OS, env::consts::ARCH);
    std::process::exit(0);
}

pub(crate) fn install()
{
    println!("install");
}

pub(crate) fn uninstall()
{
    println!("To uninstall a modloader instance, simply delete the directory of the instance.
As of right now there is no way to uninstall an instance with the moddah CLI.
    ");
}