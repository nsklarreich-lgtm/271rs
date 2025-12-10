use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiffOp {
    Insert { index: usize, line: String },
    Delete { index: usize, line: String },
}

// LCS table
fn lcs_table(a: &[String], b: &[String]) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];

    for i in 0..n {
        for j in 0..m {
            if a[i] == b[j] {
                dp[i+1][j+1] = dp[i][j] + 1;
            } else {
                dp[i+1][j+1] = dp[i+1][j].max(dp[i][j+1]);
            }
        }
    }
    dp
}

// Structured diff (machine reversible)
pub fn diff_structured(old: &[String], new: &[String]) -> Vec<DiffOp> {
    let dp = lcs_table(old, new);
    let mut ops = Vec::new();

    let mut i = old.len();
    let mut j = new.len();

    while i > 0 || j > 0 {
        if i > 0 && j > 0 && old[i-1] == new[j-1] {
            i -= 1;
            j -= 1;
        } else if j > 0 && (i == 0 || dp[i][j-1] >= dp[i-1][j]) {
            ops.push(DiffOp::Insert { index: i, line: new[j-1].clone() });
            j -= 1;
        } else {
            ops.push(DiffOp::Delete { index: i-1, line: old[i-1].clone() });
            i -= 1;
        }
    }

    ops.reverse();
    ops
}

// Human-readable unified diff
pub fn diff_unified(old: &[String], new: &[String]) -> Vec<String> {
    let ops = diff_structured(old, new);
    let mut out = vec!["@@" .to_string()];

    for op in ops {
        match op {
            DiffOp::Insert { line, .. } => out.push(format!("+{}", line)),
            DiffOp::Delete { line, .. } => out.push(format!("-{}", line)),
        }
    }
    out
}

// Apply reverse diff for revert
pub fn apply_reverse(lines: &mut Vec<String>, ops: &[DiffOp]) {
    for op in ops.iter().rev() {
        match op {
            DiffOp::Insert { index, .. } => {
                if *index < lines.len() {
                    lines.remove(*index);
                }
            }
            DiffOp::Delete { index, line } => {
                lines.insert(*index, line.clone());
            }
        }
    }
}

