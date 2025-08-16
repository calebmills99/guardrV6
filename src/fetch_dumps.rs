use reqwest::blocking::Client;
use std::fs::File;
use std::io::copy;

pub fn download_dump(url: &str, save_as: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading from: {}", url);
    let response = Client::new().get(url).send()?;

    // Fix the borrow-checker drama:
    let bytes = response.bytes()?;               // Keep bytes alive here
    let mut content = bytes.as_ref();            // Safe reference now

    let mut file = File::create(save_as)?;
    copy(&mut content, &mut file)?;

    println!("✅ File saved as {}", save_as);
    Ok(())
}
