use {argh::FromArgs, std::fmt::Debug};

#[derive(FromArgs, PartialEq, Debug)]
/// Nix Systems Management
struct Args {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    OS(OSCommand),
    Home(HomeCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// OS
#[argh(subcommand, name = "os")]
struct OSCommand {
    #[argh(subcommand)]
    subcommand: OSSubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum OSSubCommand {
    Switch(OsSwitchCommand),
    Boot(BootCommand),
    Test(TestCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Switch
#[argh(subcommand, name = "switch")]
struct OsSwitchCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Boot
#[argh(subcommand, name = "boot")]
struct BootCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Test
#[argh(subcommand, name = "test")]
struct TestCommand {}

#[derive(FromArgs, PartialEq, Debug)]
/// Home
#[argh(subcommand, name = "home")]
struct HomeCommand {
    #[argh(subcommand)]
    subcommand: HomeSubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum HomeSubCommand {
    Switch(HomeSwitchCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Switch
#[argh(subcommand, name = "switch")]
struct HomeSwitchCommand {}

fn main() {
    let args: Args = argh::from_env();

    match args.command {
        Command::OS(OSCommand { subcommand }) => match subcommand {
            OSSubCommand::Switch(OsSwitchCommand {}) => {
                println!("os switch");
            }
            OSSubCommand::Boot(BootCommand {}) => {
                println!("os boot");
            }
            OSSubCommand::Test(TestCommand {}) => {
                println!("os test");
            }
        },
        Command::Home(HomeCommand { subcommand }) => match subcommand {
            HomeSubCommand::Switch(HomeSwitchCommand {}) => {
                println!("home switch");
            }
        },
    }
}
