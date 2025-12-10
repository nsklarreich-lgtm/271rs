use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use anyhow::Result;
use std::fs;
use std::path::Path;
use diff::{DiffOp, diff_structured};

pub type Snapshot = HashMap<String, Vec<String>>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Commit {
    pub snapshot: Snapshot,
    pub diff: HashMap<String, Vec<DiffOp>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Scm {
    pub commits: Vec<Commit>,
}

pub const SCM_NAME: &str = ".scm";

pub fn load_scm() -> Result<Scm> {
    if !Path::new(SCM_NAME).exists() {
        return Ok(Scm { commits: vec![] });
    }
    Ok(serde_json::from_str(&fs::read_to_string(SCM_NAME)?)?)
}

pub fn save_scm(scm: &Scm) -> Result<()> {
    fs::write(SCM_NAME, serde_json::to_string_pretty(scm)?)?;
    Ok(())
}

pub fn read_all_files() -> Result<Snapshot> {
    let mut map = Snapshot::new();

    fn walk(path: &Path, out: &mut Snapshot) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if name.starts_with('.') {
                continue;
            }

            if p.is_dir() {
                walk(&p, out)?;
            } else {
                let text = match fs::read_to_string(&p) {
                    Ok(t) => t,
                    Err(_) => continue,
                };
                let lines: Vec<String> = text.lines().map(|x| x.to_string()).collect();
                out.insert(p.to_string_lossy().to_string(), lines);
            }
        }
        Ok(())
    }

    walk(Path::new("."), &mut map)?;
    Ok(map)
}

fn sync_to_snapshot(snapshot: &Snapshot) -> Result<()> {
    let current = read_all_files()?;
    for file in current.keys() {
        if !snapshot.contains_key(file) {
            if Path::new(file).exists() {
                fs::remove_file(file)?;
            }
        }
    }
    for (file, lines) in snapshot {
        fs::write(file, lines.join("\n"))?;
    }
    Ok(())
}

pub fn commit(scm: &mut Scm) -> Result<()> {
    let current = read_all_files()?;
    let mut diffs = HashMap::new();

    if let Some(prev) = scm.commits.last() {
        let prev_snap = &prev.snapshot;

        for (file, new_lines) in &current {
            match prev_snap.get(file) {
                Some(old_lines) => {
                    if old_lines != new_lines {
                        let ops = diff_structured(old_lines, new_lines);
                        diffs.insert(file.clone(), ops);
                    }
                }
                None => {
                    let empty: Vec<String> = Vec::new();
                    let ops = diff_structured(&empty, new_lines);
                    diffs.insert(file.clone(), ops);
                }
            }
        }

        for (file, old_lines) in prev_snap {
            if !current.contains_key(file) {
                let empty: Vec<String> = Vec::new();
                let ops = diff_structured(old_lines, &empty);
                diffs.insert(file.clone(), ops);
            }
        }
    }

    scm.commits.push(Commit {
        snapshot: current,
        diff: diffs,
    });

    Ok(())
}

pub fn scrape(scm: &Scm) -> Result<()> {
    if let Some(last) = scm.commits.last() {
        sync_to_snapshot(&last.snapshot)?;
    }
    Ok(())
}

pub fn revert(scm: &mut Scm) -> Result<()> {
    if scm.commits.len() < 2 {
        return Ok(());
    }

    scm.commits.pop();

    if let Some(prev) = scm.commits.last() {
        sync_to_snapshot(&prev.snapshot)?;
    }

    Ok(())
}

