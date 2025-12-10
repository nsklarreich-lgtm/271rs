use anyhow::Result;
use std::env;
use scm::{load_scm, save_scm, commit, scrape, revert};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: scm <commit|scrape|revert|viewer>");
        return Ok(());
    }

    let cmd = &args[1];
    let mut scm = load_scm()?;

    match cmd.as_str() {
        "commit" => {
            commit(&mut scm)?;
            save_scm(&scm)?;
        }
        "scrape" => {
            scrape(&scm)?;
        }
        "revert" => {
            revert(&mut scm)?;
            save_scm(&scm)?;
        }
        "viewer" => {
            println!("{}", serde_json::to_string_pretty(&scm)?);
        }
        _ => println!("Unknown command"),
    }

    Ok(())
}

