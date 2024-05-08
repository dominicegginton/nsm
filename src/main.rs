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
/// Host configuration
#[argh(subcommand, name = "host")]
struct HostCommand {
    #[argh(subcommand)]
    subcommand: HostSubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum HostSubCommand {
    Switch(HostSwitchCommand),
    Test(HostTestCommand),
    CollectGarbage(HostCollectGarbageCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Switch to the host configuration
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
            .expect("failed")
            .wait();
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Test the host configuration
#[argh(subcommand, name = "test")]
struct HostTestCommand {}
impl Runnable for HostTestCommand {
    fn run(&self) {
        let _ = std::process::Command::new("sudo")
            .arg("nixos-rebuild")
            .arg("test")
            .arg("--flake")
            .arg(env::var("NSM_FLAKE").unwrap())
            .spawn()
            .expect("failed")
            .wait();
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Collect garbage on the host
#[argh(subcommand, name = "collect-garbage")]
struct HostCollectGarbageCommand {
    #[argh(switch)]
    /// delete old generations (+2days old or more than 5 generations)
    generations: bool,
}
impl Runnable for HostCollectGarbageCommand {
    fn run(&self) {
        let _ = std::process::Command::new("sudo")
            .arg("nix-collect-garbage")
            .arg("-d")
            .spawn()
            .expect("failed")
            .wait();
        let _ = std::process::Command::new("sudo")
            .arg("nix")
            .arg("store")
            .arg("optimise")
            .spawn()
            .expect("failed")
            .wait();
        if self.generations {
            let _ = std::process::Command::new("sudo")
                .arg("nix-env")
                .arg("-p")
                .arg("/nix/var/nix/profiles/system")
                .arg("--delete-generations")
                .spawn()
                .expect("failed")
                .wait();
        }
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Home-manager configuration
#[argh(subcommand, name = "home")]
struct HomeCommand {
    #[argh(subcommand)]
    subcommand: HomeSubCommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum HomeSubCommand {
    Switch(HomeSwitchCommand),
    Build(HomeBuildCommand),
    CollectGarbage(HomeCollectGarbageCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Switch to the home-manager configuration
#[argh(subcommand, name = "switch")]
struct HomeSwitchCommand {}
impl Runnable for HomeSwitchCommand {
    fn run(&self) {
        let _ = std::process::Command::new("home-manager")
            .arg("switch")
            .arg("--flake")
            .arg(env::var("NSM_FLAKE").unwrap())
            .spawn()
            .expect("failed")
            .wait();
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Build te home-manager configuration
#[argh(subcommand, name = "build")]
struct HomeBuildCommand {}
impl Runnable for HomeBuildCommand {
    fn run(&self) {
        let _ = std::process::Command::new("home-manager")
            .arg("build")
            .arg("--flake")
            .arg(env::var("NSM_FLAKE").unwrap())
            .spawn()
            .expect("failed")
            .wait();
    }
}

#[derive(FromArgs, PartialEq, Debug)]
/// Cleanup the home-manager generations
#[argh(subcommand, name = "collect-garbage")]
struct HomeCollectGarbageCommand {}
impl Runnable for HomeCollectGarbageCommand {
    fn run(&self) {
        let _ = std::process::Command::new("home-manager")
            .arg("expire-generations")
            .arg("0")
            .spawn()
            .expect("failed")
            .wait();
    }
}

fn main() {
    if env::var("NSM_FLAKE").is_ok() {
        let args: Args = argh::from_env();
        match args.command {
            Command::Host(HostCommand { subcommand }) => match subcommand {
                HostSubCommand::Switch(HostSwitchCommand {}) => HostSwitchCommand {}.run(),
                HostSubCommand::Test(HostTestCommand {}) => HostTestCommand {}.run(),
                HostSubCommand::CollectGarbage(HostCollectGarbageCommand { generations }) => {
                    HostCollectGarbageCommand { generations }.run()
                }
            },
            Command::Home(HomeCommand { subcommand }) => match subcommand {
                HomeSubCommand::Switch(HomeSwitchCommand {}) => HomeSwitchCommand {}.run(),
                HomeSubCommand::Build(HomeBuildCommand {}) => HomeBuildCommand {}.run(),
                HomeSubCommand::CollectGarbage(HomeCollectGarbageCommand {}) => HomeCollectGarbageCommand {}.run(),
            },
        }
    } else {
        println!("NSM_FLAKE is not set");
    }
}
