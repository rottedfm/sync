use chrono::Local;
use clap::Parser;
use console::{Term, style};
use std::env;
use std::io;
use std::process::Command;

#[derive(Parser)]
#[command(version, about = r#"
                                                                       
            _____   ______   _____  _____    _____            _____    
       _____\    \ |\     \ |     ||\    \   \    \      _____\    \_  
      /    / \    |\ \     \|     | \\    \   |    |    /     /|     | 
     |    |  /___/| \ \           |  \\    \  |    |   /     / /____/| 
  ____\    \ |   ||  \ \____      |   \|    \ |    |  |     | |____|/  
 /    /\    \|___|/   \|___/     /|    |     \|    |  |     |  _____   
|    |/ \    \            /     / |   /     /\      \ |\     \|\    \  
|\____\ /____/|          /_____/  /  /_____/ /______/|| \_____\|    |  
| |   ||    | |          |     | /  |      | |     | || |     /____/|  
 \|___||____|/           |_____|/   |______|/|_____|/  \|_____|    ||  
                                                              |____|/  "#, long_about = None)]
struct Cli {
    #[arg(short, long)]
    skip_git: bool,

    #[arg(short, long)]
    first_sync: bool,
}

fn rebuild_nixos() -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let flake = format!("{}/.nix#fm", home_dir);

    println!("{} - Rebuilding NixOS with flake...", style("Info").cyan());

    let status = Command::new("sudo")
        .args(["nixos-rebuild", "switch", "--flake", &flake])
        .status()?;

    if status.success() {
        println!(
            "{} - NixOS rebuild completed successfully using flake.",
            style("Success").green()
        );
        Ok(())
    } else {
        eprintln!("Error: Failed to rebuild NixOS using flake.");
        Err(io::Error::new(
            io::ErrorKind::Other,
            "NixOS rebuild with flake failed",
        ))
    }
}

fn rebuild_homemanager() -> io::Result<()> {
    println!("{} - Rebuilding NixOS with flake...", style("Info").cyan());

    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let flake = format!("{}/.nix#rotted@fm", home_dir);

    let status = Command::new("home-manager")
        .args(["switch", "--flake", &flake])
        .status()?;

    if status.success() {
        println!(
            "{} - HomeManager rebuild completed successfully using flake.",
            style("Success").green()
        );
        Ok(())
    } else {
        eprintln!("Error: Failed to rebuild NixOS using flake.");
        Err(io::Error::new(
            io::ErrorKind::Other,
            "HomeManager rebuild with flake failed",
        ))
    }
}

fn remove_dir(dir: &str) -> io::Result<()> {
    let status = Command::new("sudo").args(["rm", dir]).status()?;

    if status.success() {
        println!("{} - Removed directory {}.", style("Success").green(), dir);
        Ok(())
    } else {
        eprintln!("Error: Failed to add .nix directory to git.");
        Err(io::Error::new(io::ErrorKind::Other, "Git add failed"))
    }
}

fn git_add() -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let nix_dir = format!("{}/.nix", home_dir);

    println!(
        "{} - Adding changes in {} to git...",
        style("Info").green(),
        nix_dir
    );
    let status = Command::new("git").arg("add").arg(&nix_dir).status()?;

    if status.success() {
        println!(
            "{} - Added changes in {} to git.",
            style("Success").green(),
            nix_dir
        );
        Ok(())
    } else {
        eprintln!("Error: Failed to add .nix directory to git.");
        Err(io::Error::new(io::ErrorKind::Other, "Git add failed"))
    }
}

fn git_commit() -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let nix_dir = format!("{}/.nix", home_dir);
    let commit_message = format!("Auto commit - {}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    println!(
        "{} - Committing changes to git with message: {}",
        style("Info").cyan(),
        commit_message
    );
    let status = Command::new("git")
        .current_dir(&nix_dir)
        .args(["commit", "-m", &commit_message])
        .status()?;

    if status.success() {
        println!("{} - Commited changes to git.", style("Success").green());
        Ok(())
    } else {
        eprintln!("Error: Failed to commit changes to git.");
        Err(io::Error::new(io::ErrorKind::Other, "Git commit failed"))
    }
}

fn git_push() -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let nix_dir = format!("{}/.nix", home_dir);

    println!(
        "{} - Pushing changes to git repository...",
        style("Info").cyan()
    );
    let status = Command::new("git")
        .current_dir(&nix_dir)
        .arg("push")
        .status()?;

    if status.success() {
        println!(
            "{} Sucessfully pushed changes to git.",
            style("Success").green()
        );
        Ok(())
    } else {
        eprintln!("Error: Failed to push changes to git.");
        Err(io::Error::new(io::ErrorKind::Other, "Git push failed"))
    }

    
}

fn chown_lock() -> io::Result<()> {
    let home_dir = env::var("HOME").expect("Failed to get HOME directory");
    let lock = format!("{}/.nix/flake.lock", home_dir);
    let status = Command::new("sudo")
        .args(["chown", "rotted", &lock])
        .status()?;

    if status.success() {
        println!(
            "{} Sucessfully chowned flake.lock.",
            style("Success").green()
        );
        Ok(())
    } else {
        eprintln!("Error: Failed to chown flake.lock");
        Err(io::Error::new(io::ErrorKind::Other, "Failed to chown"))
    }
    
}

fn main() {
    let cli = Cli::parse();

    let term = Term::stdout();

    let _ = term.clear_screen();
    println!(
        r#"
            _____   ______   _____  _____    _____            _____    
       _____\    \ |\     \ |     ||\    \   \    \      _____\    \_  
      /    / \    |\ \     \|     | \\    \   |    |    /     /|     | 
     |    |  /___/| \ \           |  \\    \  |    |   /     / /____/| 
  ____\    \ |   ||  \ \____      |   \|    \ |    |  |     | |____|/  
 /    /\    \|___|/   \|___/     /|    |     \|    |  |     |  _____   
|    |/ \    \            /     / |   /     /\      \ |\     \|\    \  
|\____\ /____/|          /_____/  /  /_____/ /______/|| \_____\|    |  
| |   ||    | |          |     | /  |      | |     | || |     /____/|  
 \|___||____|/           |_____|/   |______|/|_____|/  \|_____|    ||  
                                                              |____|/ "#
    );

    if cli.skip_git {
        if let Err(e) = rebuild_nixos() {
            eprintln!("Error rebuilding NixOS: {}", e);
        }

        if let Err(e) = rebuild_homemanager() {
            eprintln!("Error rebuilding HomeManager: {}", e);
        }
    } else {
        if let Err(e) = git_add() {
            eprintln!("Error adding .nix directory to git: {}", e);
        }

        if let Err(e) = rebuild_nixos() {
            eprintln!("Error rebuilding NixOS: {}", e);
        }

        if let Err(e) = rebuild_homemanager() {
            eprintln!("Error rebuilding HomeManager: {}", e);
        }

        if let Err(e) = git_commit() {
            eprintln!("Error commiting changes to git: {}", e);
        }

        if let Err(e) = git_push() {
            eprintln!("Error pushing changes to git: {}", e);
        }
    }

    if cli.first_sync {
        if let Err(e) = rebuild_nixos() {
            eprintln!("Error rebuilding NixOS: {}", e);
        }

        let channel_path1 = "/root/.nix-defexpr/channels";
        let channel_path2 = "/nix/var/nix/profiles/per-user/root/channels";

        if let Err(e) = remove_dir(channel_path1) {
            eprintln!("Error removing path: {}", e);
        }

        if let Err(e) = remove_dir(channel_path2) {
            eprintln!("Error rebuilding path: {}", e);
        }

        if let Err(e) = chown_lock() {
            eprintln!("Error chowning lock: {}", e);
        }

        if let Err(e) = rebuild_homemanager() {
            eprintln!("Error rebuilding HomeManager: {}", e);
        }

    }
}
