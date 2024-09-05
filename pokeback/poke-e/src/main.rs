use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    url: String,
    id: usize,
    sprite: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Call the Pokémon API and fetch all the Pokémon
    let response = reqwest::get("https://pokeapi.co/api/v2/pokemon?limit=151")
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Extract names and URLs
    let results = response["results"].as_array().unwrap();
    let mut pokedex = Vec::new();

    for (id, pokemon) in results.iter().enumerate() {
        let name = pokemon["name"].as_str().unwrap().to_string();
        let url = pokemon["url"].as_str().unwrap().to_string();

        // Fetch Pokémon details
        let detail_response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
        let sprite = detail_response["sprites"]["front_default"]
            .as_str()
            .unwrap()
            .to_string();

        // Store the data in the Pokedex
        pokedex.push(Pokemon {
            id: id + 1,
            name,
            url,
            sprite,
        });
    }

    // Save to JSON
    let json = serde_json::to_string_pretty(&pokedex)?;
    let mut file = File::create("pokedex.json")?;
    file.write_all(json.as_bytes())?;

    // Print saved for confirmation
    println!("Pokedex data saved!");

    Ok(())
}
