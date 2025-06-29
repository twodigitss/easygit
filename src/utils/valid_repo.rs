use regex::Regex;

pub fn is_valid_repo(url: &str) -> bool {
    let pattern = r"^(https:\/\/github\.com\/|git@github\.com:)[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+\.git$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(url)
}

