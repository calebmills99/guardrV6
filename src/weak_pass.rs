use std::fs;
use std::collections::HashSet;

pub fn load_password_list(file_path: &str) -> Result<HashSet<String>, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    // Trimming ensures no hidden whitespace messes with your glam
    let passwords = contents.lines().map(|line| line.trim().to_string()).collect();
    Ok(passwords)
}

pub fn is_password_weak(password: &str, common_passwords: &HashSet<String>) -> bool {
    common_passwords.contains(password)
}
