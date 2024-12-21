use chrono::Local;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    io,
    process::{Command, ExitStatus, Stdio},
    time::Duration,
};

#[derive(Parser)]
#[command(name = "sync")]
#[command(version = "1.0")]
#[command(about = r#"
  _________.__. ____   ____  
 /  ___<   |  |/    \_/ ___\ 
 \___ \ \___  |   |  \  \___ 
/____  >/ ____|___|  /\___  >
     \/ \/         \/     \/ 
    "#)]
struct Cli {
    #[arg(short, long, help = "Perform nix-garbage-collection")]
    garbage_collection: bool,
    #[arg(short, long, help = "Skip Git Commands")]
    skip_git: bool,
}

fn run_nix_collect_garbage() -> io::Result<ExitStatus> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.white} {msg}")
            .expect("Invalid template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("✓ Running nix-collect-garbage...");

    let status = Command::new("nix-collect-garbage")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    pb.finish_with_message("✓ 'nix-collect-garbage' completed.");
    Ok(status)
}

fn pull_main_branch(repo_path: &str) -> io::Result<ExitStatus> {
    println!(" ✓ Pulling main branch...");

    let status = Command::new("git")
        .arg("-C") // Specify the directory
        .arg(repo_path)
        .arg("pull")
        .arg("origin")
        .arg("main")
        .status()?;

    println!(" ✓ 'git pull' completed.");
    Ok(status)
}

fn run_nixos_rebuild(flake_path: &str, profile: &str) -> io::Result<ExitStatus> {
    let pb = ProgressBar::new_spinner();
    pb.set_message("✓ Running nixos-rebuild...");

    let full_flake = format!("{}#{}", flake_path, profile);

    let status = Command::new("sudo")
        .arg("nixos-rebuild")
        .arg("switch")
        .arg("--flake")
        .arg(&full_flake)
        .status()?;

    pb.finish_with_message("✓ nixos-rebuild completed.");

    Ok(status)
}

fn git_add_all(repo_path: &str) -> io::Result<ExitStatus> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.white} {msg}")
            .expect("Invalid template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("✓ Staging changes...");

    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("add")
        .arg(".")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    pb.finish_with_message("✓ Staging operation completed.");
    Ok(status)
}

fn run_home_manager_switch(flake_path: &str, profile: &str) -> io::Result<ExitStatus> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.white} {msg}")
            .expect("Invalid template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("✓ Running home-manager switch...");

    let full_flake = format!("{}#{}", flake_path, profile);

    let status = Command::new("home-manager")
        .arg("switch")
        .arg("--flake")
        .arg(&full_flake)
        .status()?;

    pb.finish_with_message("✓ home-manager switch completed.");
    Ok(status)
}

fn git_show(repo_path: &str) -> io::Result<String> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.white} {msg}")
            .expect("Invalid template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("✓ Running git show...");

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("show")
        .output()?;

    pb.finish_with_message("✓ git diff completed.");

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Failed to execute git diff: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}

fn git_commit_with_time(repo_path: &str) -> io::Result<ExitStatus> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.white} {msg}")
            .expect("Invalid template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message("✓ Committing changes...");

    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let commit_message = format!("Commit on {}", current_time);

    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

    pb.finish_with_message("✓ Commit operation completed.");
    Ok(status)
}

fn clear_screen() -> io::Result<ExitStatus> {
    let status = Command::new("clear").status()?;
    Ok(status)
}

fn git_push(repo_path: &str) -> io::Result<ExitStatus> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.white} {msg}")
            .expect("Invalid template"),
    );
    pb.set_message("✓ Pushing changes to main branch...");

    let status = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("push")
        .arg("origin")
        .arg("main")
        .status()?;

    pb.enable_steady_tick(Duration::from_millis(100));
    pb.finish_with_message("✓ Push operation completed.");
    Ok(status)
}

fn main() {
    let args = Cli::parse();

    let repo_path = "/home/rotted/.system";
    let nixos_profile = "fm";
    let home_manager_profile = "rotted@fm";
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    println!(
        r#"
  _________.__. ____   ____  
 /  ___<   |  |/    \_/ ___\ 
 \___ \ \___  |   |  \  \___ 
/____  >/ ____|___|  /\___  >
     \/ \/         \/     \/ 
-----------------------------
     {}
        "#,
        current_time
    );

    if args.garbage_collection {
        if let Err(e) = run_nix_collect_garbage() {
            eprintln!("✗ Error during garbage collection: {}", e);
        }
    }

    if let Err(e) = clear_screen() {
        eprintln!("  ✗ Error clearing clear: {}", e);
    }

    if let Err(e) = git_add_all(repo_path) {
        eprintln!("  ✗ Error during staging operation: {}", e);
    }

    if let Err(e) = run_nixos_rebuild(repo_path, nixos_profile) {
        eprintln!("  ✗ Error during nixos-rebuild operation: {}", e);
    }

    if let Err(e) = run_home_manager_switch(repo_path, home_manager_profile) {
        eprintln!("  ✗ Error during home-manager operation: {}", e);
    }

    // Skip Git operations if --skip-git is set
    if !args.skip_git {
        if let Err(e) = pull_main_branch(repo_path) {
            eprintln!("  ✗ Error during pull operation: {}", e);
        }

        if let Err(e) = git_add_all(repo_path) {
            eprintln!("  ✗ Error during staging operation: {}", e);
        }

        if let Err(e) = git_commit_with_time(repo_path) {
            eprintln!("  ✗ Error during commit operation: {}", e);
        }
        match git_show(repo_path) {
            Ok(show) => {
                if show.is_empty() {
                    println!("No differences found.");
                } else {
                    println!("{}", show);
                }
            }
            Err(e) => eprintln!("  ✗ Error executing git show: {}", e),
        }

        if let Err(e) = git_push(repo_path) {
            eprintln!("  ✗ Error during push operation: {}", e);
        }
    } else {
        println!(" Skipping Git operations as per the '--skip-git' flag.");
    }
}
