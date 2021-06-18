extern crate html_escape;
use html_escape::encode_text;


fn replace_deco(s: &str, t: &str) -> String {
	"<span style=\"color:blue\">[".to_string()
		+ &encode_text(&s) + "->"
		+ &encode_text(&t) + "]</span>"
}

fn delete_deco(s: &str) -> String {
	"<span style=\"color:red\">".to_string()
		+ &encode_text(&s) + "</span>"
}

fn insert_deco(t: &str) -> String {
	"<span style=\"color:green\">".to_string()
		+ &encode_text(&t) + "</span>"
}



pub struct EditDistance {
	ss: String,
	st: String,
	dp: Vec<Vec<i32>>,
	restr: Vec<Vec<u8>>
}

impl EditDistance {
	pub fn new() -> Self {
		Self {
			ss: String::new(),
			st: String::new(),
			dp: Vec::<Vec<i32>>::new(),
			restr: Vec::<Vec<u8>>::new()
		}
	}

	pub fn calc(&mut self, ss: String, st: String) -> i32 {
		self.ss = ss;
		self.st = st;
		let s = self.ss.as_bytes();
		let t = self.st.as_bytes();
		let n = self.ss.len();
		let m = self.st.len();
		self.dp = vec![vec![1e9 as i32; m+1]; n+1];
		self.restr = vec![vec![0 as u8; m+1]; n+1];

		self.dp[0][0] = 0;
		for i in 0..=n {
			for j in 0..=m {
				if i>0 && j>0 {
					if s[i-1]==t[j-1] {
						if self.dp[i][j] > self.dp[i-1][j-1] {
							self.dp[i][j] = self.dp[i-1][j-1];
							self.restr[i][j] = 0;	// None
						}
					} else {
						if self.dp[i][j] > self.dp[i-1][j-1] + 1 {
							self.dp[i][j] = self.dp[i-1][j-1] + 1;
							self.restr[i][j] = 1;	// replace
						}
					}
				}

				if i>0 {
					if self.dp[i][j] > self.dp[i-1][j] + 1 {
						self.dp[i][j] = self.dp[i-1][j] + 1;
						self.restr[i][j] = 2;	// delete
					}
				}
				if j>0 {
					if self.dp[i][j] > self.dp[i][j-1] + 1 {
						self.dp[i][j] = self.dp[i][j-1] + 1;
						self.restr[i][j] = 3;	// insert
					}
				}
			}
		}

		self.dp[n][m]
	}

	pub fn restore(&self) -> String {
		let mut ret = Vec::<String>::new();
		let n = self.ss.len();
		let m = self.st.len();
		let mut i = n;
		let mut j = m;

		while i != 0 || j != 0 {
			match self.restr[i][j] {
				0 => {
					i -= 1;
					j -= 1;
					ret.push(self.ss[i..i+1].to_string());
				},
				1 => {
					i -= 1;
					j -= 1;
					ret.push(replace_deco(&self.ss[i..i+1], &self.st[j..j+1]));
				},
				2 => {
					i -= 1;
					ret.push(delete_deco(&self.ss[i..i+1]));
				},
				3 => {
					j -= 1;
					ret.push(insert_deco(&self.st[j..j+1]));
				},
				_ => {}
			}
		}

		ret.reverse();
		lf_to_br(ret.into_iter().collect()).clone()
	}
}




fn lf_to_br(s: String) -> String {
	let mut ret = "".to_string();
	for c in s.chars() {
		if c=='\n' {
			ret += "<br>";
		} else if c=='\t' {
			ret += "&nbsp;&nbsp;&nbsp;&nbsp;";
		} else {
			ret += &String::from(c);
		}
	}
	return ret;
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn edit_distance_test() {
		let mut edist = EditDistance::new();
		let mut s = "acac".to_string();
		let mut t = "acm".to_string();
		assert_eq!(2, edist.calc(s, t));

		s = "icpc".to_string();
		t = "icpc".to_string();
		assert_eq!(0, edist.calc(s, t));
	}
}
