use regex::Regex;

fn add_third_party_crates_to_project() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2012-01-01"));
}
