use std::fs;

fn curl_input(day: u32) -> String {
    let cookie_value = fs::read_to_string("aoc_session_cookie").unwrap();
    let output = std::process::Command::new("curl")
        .args([
            "-H".to_string(),
            format!("Cookie: {}", cookie_value),
            format!("https://adventofcode.com/2024/day/{}/input", day),
        ])
        .output()
        .unwrap();
    if !output.status.success() {
        panic!("Failed to call curl")
    }
    std::str::from_utf8(&output.stdout).unwrap().to_string()
}

pub fn input(day: u32) -> String {
    let path = format!("input/{:02}", day);
    fs::read_to_string(&path).unwrap_or_else(|_| {
        let input = curl_input(day);
        fs::write(&path, &input).unwrap();
        input
    })
}
