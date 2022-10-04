use reqwest;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::blocking::get("https://v2.jokeapi.dev/joke/Programming,Dark?lang=es")?.json()?;
    println!("{:#?}", resp);
    Ok(())
}
