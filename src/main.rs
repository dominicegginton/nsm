use std::env;
use {argh::FromArgs, std::fmt::Debug};

trait Runnable {
    fn run(&self);
}

#[derive(FromArgs, PartialEq, Debug)]
/// Nix Systems Management
struct Args {
    #[argh(subcommand)]
    command: Command,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum Command {
    Host(HostCommand),
    Home(HomeCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Host
#[argh(subcommand, name = "host")]
struct HostCommand {
    #[argh(subcommand)]
    subcommand: HostSubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum HostSubCommand {
    Switch(HostSwitchCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Switch
#[argh(subcommand, name = "switch")]
struct HostSwitchCommand {}
impl Runnable for HostSwitchCommand {
    fn run(&self) {
        let _ = std::process::Command::new("sudo")
            .arg("nixos-rebuild")
            .arg("switch")
            .arg("--flake")
            .arg(env::var("NSM_FLAKE").unwrap())
            .spawn()
            .expect("ls command failed to start")
            .wait();
    }
}

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
impl Runnable for HomeSwitchCommand {
    fn run(&self) {
        let _ =    std::process::Command::new("home-manager")
            .arg("switch")
            .arg("--flake")
            .arg(env::var("NSM_FLAKE").unwrap())
            .spawn()
            .expect("ls command failed to start")
            .wait();
    }
}

fn main() {
    if env::var("NSM_FLAKE").is_ok() {
        let args: Args = argh::from_env();
        let command: &dyn Runnable = match args.command {
            Command::Host(HostCommand { subcommand }) => match subcommand {
                HostSubCommand::Switch(HostSwitchCommand {}) => &HostSwitchCommand {},
            },
            Command::Home(HomeCommand { subcommand }) => match subcommand {
                HomeSubCommand::Switch(HomeSwitchCommand {}) => &HomeSwitchCommand {},
            },
        };
        command.run();
    } else {
        println!("NSM_FLAKE is not set");
    }
}
