extern crate html_escape;
use html_escape::encode_text;

pub fn edit_distance(ss: String, st: String) -> (i32, String) {
	let s = ss.as_bytes();
	let t = st.as_bytes();
	let n = s.len();
	let m = t.len();

	let mut dps = vec![vec!["".to_string(); m+1]; n+1];
	let mut dp = vec![vec![1e9 as i32; m+1]; n+1];
	dp[0][0] = 0;

	for i in 0..=n {
		for j in 0..=m {
			if i>0 && j>0 {
				if s[i-1]==t[j-1] {
					if dp[i][j] > dp[i-1][j-1] {
						dp[i][j] = dp[i-1][j-1];
						dps[i][j] = dps[i-1][j-1].clone() + &encode_text(&ss[i-1..i]);
					}
				} else {
					if dp[i][j] > dp[i-1][j-1] + 1 {
						dp[i][j] = dp[i-1][j-1] + 1;
						dps[i][j] = dps[i-1][j-1].clone() + "<span style=\"color:blue\">["
							+ &encode_text(&ss[i-1..i]) + "->"
							+ &encode_text(&st[j-1..j]) + "]</span>";
					}
				}
			}

			if i>0 {
				if dp[i][j] > dp[i-1][j] + 1 {
					dp[i][j] = dp[i-1][j] + 1;
					dps[i][j] = dps[i-1][j].clone() + "<span style=\"color:red\">" 
						+ &encode_text(&ss[i-1..i]) + "</span>";
				}
			}
			if j>0 {
				if dp[i][j] > dp[i][j-1] + 1 {
					dp[i][j] = dp[i][j-1] + 1;
					dps[i][j] = dps[i][j-1].clone() + "<span style=\"color:green\">"
						+ &encode_text(&st[j-1..j]) + "</span>";
				}
			}
		}
	}

	return (dp[n][m], dps[n][m].clone());
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn edit_distance_test() {
		let mut s = "acac".to_string();
		let mut t = "acm".to_string();
		assert_eq!((2, "acam".to_string()), edit_distance(s, t));

		s = "icpc".to_string();
		t = "icpc".to_string();
		assert_eq!((0, "icpc".to_string()), edit_distance(s, t));
	}
}
