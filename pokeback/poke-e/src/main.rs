use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    name: String,
    url: String,
    id: usize,
    sprite: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PokemonListResponse {
    results: Vec<PokemonEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
struct PokemonEntry {
    name: String,
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define the URL for the Pokémon API
    let url = "https://pokeapi.co/api/v2/pokemon?limit=151";

    // Create a new HTTP client
    let client = Client::new();

    // Fetch the list of Pokémon
    let response = client.get(url).send().await?.json::<PokemonListResponse>().await?;

    // Prepare to collect detailed Pokémon data
    let mut pokedex = Vec::new();

    for (id, pokemon_entry) in response.results.iter().enumerate() {
        // Fetch Pokémon details
        let detail_response = client.get(&pokemon_entry.url).send().await?.json::<serde_json::Value>().await?;
        let sprite = detail_response["sprites"]["front_default"]
            .as_str()
            .unwrap_or("")
            .to_string();

        // Store the data in the Pokedex
        pokedex.push(Pokemon {
            id: id + 1,
            name: pokemon_entry.name.clone(),
            url: pokemon_entry.url.clone(),
            sprite,
        });
    }

    // Define the path where the file will be saved
    let file_path = PathBuf::from("C:\\Users\\Kliea\\Documents\\Development\\Poke\\pokefront\\poke-e\\public").join("pokedex.json");

    // Serialize the Pokedex to a JSON string
    let json_data = serde_json::to_string_pretty(&pokedex)?;

    // Write the JSON string to the file
    let mut file = File::create(&file_path)?;
    file.write_all(json_data.as_bytes())?;

    // Print the file path for confirmation
    println!("Pokédex data successfully saved to {}", file_path.display());

    Ok(())
}
