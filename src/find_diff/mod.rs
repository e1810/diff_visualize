pub fn edit_distance(s: &[u8], t: &[u8]) -> (i32, String) {
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
						dps[i][j] = dps[i-1][j-1].clone() + &String::from(s[i-1]as char);
					}
				} else {
					if dp[i][j] > dp[i-1][j-1] + 1 {
						dp[i][j] = dp[i-1][j-1] + 1;
						dps[i][j] = dps[i-1][j-1].clone() + "<span style=\"color:blue\">["
							+ &String::from(s[i-1]as char) + "->"
							+ &String::from(t[j-1]as char) + "]</span>";
					}
				}
			}

			if i>0 {
				if dp[i][j] > dp[i-1][j] + 1 {
					dp[i][j] = dp[i-1][j] + 1;
					dps[i][j] = dps[i-1][j].clone() + "<span style=\"color:red\">" 
						+ &String::from(s[i-1]as char) + "</span>";
				}
			}
			if j>0 {
				if dp[i][j] > dp[i][j-1] + 1 {
					dp[i][j] = dp[i][j-1] + 1;
					dps[i][j] = dps[i][j-1].clone() + "<span style=\"color:green\">"
						+ &String::from(t[j-1]as char) + "</span>";
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
		let mut s = "acac".as_bytes();
		let mut t = "acm".as_bytes();
		assert_eq!((2, "acam".to_string()), edit_distance(&s, &t));

		s = "icpc".as_bytes();
		t = "icpc".as_bytes();
		assert_eq!((0, "icpc".to_string()), edit_distance(&s, &t));


	}
}
