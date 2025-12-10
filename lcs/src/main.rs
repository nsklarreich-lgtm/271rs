fn lcs(a: &str, b: &str) -> String {
    let a_bytes = a.as_bytes();
    let b_bytes = b.as_bytes();
    let n = a_bytes.len();
    let m = b_bytes.len();

    
    let mut dp = vec![vec![0usize; m + 1]; n + 1];

    
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            dp[i][j] = if a_bytes[i] == b_bytes[j] {
                dp[i + 1][j + 1] + 1
            } else {
                dp[i + 1][j].max(dp[i][j + 1])
            };
        }
    }

    
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::new();

    while i < n && j < m {
        if a_bytes[i] == b_bytes[j] {
            result.push(a_bytes[i]);
            i += 1;
            j += 1;
        } else if dp[i + 1][j] >= dp[i][j + 1] {
            i += 1;
        } else {
            j += 1;
        }
    }

    String::from_utf8(result).unwrap()
}

fn main() {
    let mut ss = std::env::args();
    let _ = &ss.next();
    dbg!(lcs(&ss.next().unwrap(), &ss.next().unwrap()));
    return;
}
