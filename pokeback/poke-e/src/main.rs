use warp::{Filter, http::Method}; // Importing necessary modules from warp for routing and HTTP methods
use serde::{Deserialize, Serialize}; // Importing serde traits for serializing and deserializing data structures

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Main function entry point for the async runtime provided by tokio.
    
    // Setup CORS policy using warp's CORS handler. 
    // It allows any origin and permits GET requests.
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec![Method::GET]);

    // Define the warp filter for handling the "pokemon" path.
    // It listens for a GET request at /pokemon/{id} and extracts the 'id' parameter from the path.
    // The filter chains the request to the `handle_pokemon_details` function.
    let pokemon_details = warp::path("pokemon")
        .and(warp::get())  // Only allow GET method
        .and(warp::path::param())  // Extract the ID parameter from the URL
        .and_then(handle_pokemon_details);  // Route to handler function

    // Start the warp server, listening on localhost (127.0.0.1) at port 3030.
    // The server serves the 'pokemon_details' route, wrapped with the CORS policy.
    warp::serve(pokemon_details.with(cors)).run(([127, 0, 0, 1], 3030)).await;

    // Returns Ok if server runs successfully.
    Ok(())
}

async fn handle_pokemon_details(pokemon_id: usize) -> Result<impl warp::Reply, warp::Rejection> {
    // Function to handle requests for Pokemon details based on the provided 'pokemon_id'.
    
    // Format the URL to fetch data from the PokeAPI for the given pokemon_id.
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id);
    
    // Create a new HTTP client using reqwest.
    let client = reqwest::Client::new();

    // Send a GET request to the PokeAPI using the client.
    // If the request is successful and data is returned, it will attempt to deserialize it into a PokemonDetail struct.
    match client.get(&url).send().await {
        Ok(response) => match response.json::<PokemonDetail>().await {
            // If deserialization is successful, return the details as a JSON response.
            Ok(detail) => Ok(warp::reply::json(&detail)),
            // If deserialization fails, return a 404 error (not found).
            Err(_) => Err(warp::reject::not_found()),
        },
        // If the request fails (e.g., network error), return a 404 error.
        Err(_) => Err(warp::reject::not_found()),
    }
}

#[derive(Serialize, Deserialize)] // Enables serialization and deserialization for the struct
struct PokemonDetail {
    // Struct to represent the detailed data of a Pokemon.
    // It includes basic attributes like name, id, height, weight, experience, types, abilities, and sprites.
    name: String,
    id: u32,
    height: u32,
    weight: u32,
    base_experience: u32,
    types: Vec<TypeInfo>,  // A list of types the Pokémon belongs to
    abilities: Vec<AbilityInfo>,  // A list of abilities the Pokémon has
    sprites: Sprites,  // A struct to hold URLs for Pokémon sprites (images)
}

#[derive(Serialize, Deserialize)] // Enables serialization and deserialization for the struct
struct TypeInfo {
    // Struct representing type information for a Pokemon.
    // Each type has a slot number and a type name.
    slot: u32,  // Slot number indicating the order of types
    #[serde(rename = "type")] // Rename 'type' field to avoid conflict with Rust's reserved keyword 'type'
    type_name: TypeName,  // Struct holding the actual type name of the Pokémon
}

#[derive(Serialize, Deserialize)] // Enables serialization and deserialization for the struct
struct TypeName {
    // Struct to hold the name of a Pokémon type (e.g., "grass", "fire").
    name: String,  // The name of the Pokémon type
}

#[derive(Serialize, Deserialize)] // Enables serialization and deserialization for the struct
struct AbilityInfo {
    // Struct representing ability information for a Pokemon.
    // Each ability has a name field.
    ability: AbilityName,  // Struct holding the actual ability name of the Pokémon
}

#[derive(Serialize, Deserialize)] // Enables serialization and deserialization for the struct
struct AbilityName {
    // Struct to hold the name of a Pokémon ability.
    name: String,  // The name of the Pokémon's ability
}

#[derive(Serialize, Deserialize)] // Enables serialization and deserialization for the struct
struct Sprites {
    // Struct to represent Pokémon sprite images.
    // Contains a URL to the default front-facing image of the Pokémon.
    front_default: String,  // URL to the front sprite of the Pokémon
}




// use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Write;
// use std::path::PathBuf;
// use warp::Filter;

// #[derive(Debug, Serialize, Deserialize)]
// struct PokemonDetail {
//     name: String,
//     id: usize,
//     height: usize,
//     weight: usize,
//     base_experience: usize,
//     types: Vec<TypeInfo>,
//     abilities: Vec<AbilityInfo>,
//     sprites: Sprites,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TypeInfo {
//     slot: usize,
//     type_: TypeName,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TypeName {
//     name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct AbilityInfo {
//     ability: AbilityName,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct AbilityName {
//     name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Sprites {
//     front_default: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new();

//     // Define a Warp filter to serve Pokémon details
//     let pokemon_details = warp::path("pokemon")
//         .and(warp::get())
//         .and(warp::path::param())
//         .and_then(move |pokemon_id: usize| {
//             let client = client.clone();
//             async move {
//                 let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id);
//                 match client.get(&url).send().await {
//                     Ok(response) => {
//                         match response.json::<PokemonDetail>().await {
//                             Ok(detail) => Ok(warp::reply::json(&detail)),
//                             Err(_) => Err(warp::reject::not_found()),
//                         }
//                     }
//                     Err(_) => Err(warp::reject::not_found()),
//                 }
//             }
//         });

//     // Start the Warp server
//     warp::serve(pokemon_details).run(([127, 0, 0, 1], 3030)).await;

//     Ok(())
// }




// use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Write;
// use std::path::PathBuf;
// use warp::{Filter, Rejection, Reply};

// #[derive(Debug, Serialize, Deserialize)]
// struct Pokemon {
//     name: String,
//     url: String,
//     id: usize,
//     sprite: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct PokemonDetail {
//     name: String,
//     id: usize,
//     height: usize,
//     weight: usize,
//     base_experience: usize,
//     types: Vec<TypeInfo>,
//     abilities: Vec<AbilityInfo>,
//     sprites: Sprites,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TypeInfo {
//     slot: usize,
//     type_: TypeName,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct TypeName {
//     name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct AbilityInfo {
//     ability: AbilityName,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct AbilityName {
//     name: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct Sprites {
//     front_default: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Define the URL for the Pokémon API
//     let url = "https://pokeapi.co/api/v2/pokedex/national";

//     // Create a new HTTP client
//     let client = Client::new();

//     // Send a GET request to the URL
//     let response = client.get(url).send().await?.json::<Pokedex>().await?;

//     // Define the path where the file will be saved
//     let mut file_path = PathBuf::from("C:\\Users\\Kliea\\Documents\\Development\\Poke\\pokefront\\poke-e\\public");
//     file_path.push("pokedex.json");

//     // Serialize the response to a JSON string
//     let json_data = serde_json::to_string_pretty(&response)?;

//     // Write the JSON string to the file
//     let mut file = File::create(file_path.clone())?;
//     file.write_all(json_data.as_bytes())?;

//     println!("Pokédex data successfully saved to {}", file_path.display());

//     // Define a Warp filter to serve Pokémon details
//     let pokemon_details = warp::path("pokemon")
//         .and(warp::get())
//         .and(warp::path::param())
//         .and_then(move |pokemon_id: usize| {
//             let client = client.clone();
//             async move {
//                 let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokemon_id);
//                 match client.get(&url).send().await {
//                     Ok(response) => {
//                         match response.json::<PokemonDetail>().await {
//                             Ok(detail) => Ok(warp::reply::json(&detail)),
//                             Err(_) => Err(warp::reject::not_found()),
//                         }
//                     }
//                     Err(_) => Err(warp::reject::not_found()),
//                 }
//             }
//         });

//     // Start the Warp server
//     warp::serve(pokemon_details).run(([127, 0, 0, 1], 3030)).await;

//     Ok(())
// }



// use reqwest::Client;
// use serde::{Deserialize, Serialize};
// use std::fs::File;
// use std::io::Write;
// use std::path::PathBuf;

// #[derive(Debug, Serialize, Deserialize)]
// struct Pokemon {
//     name: String,
//     url: String,
//     id: usize,
//     sprite: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct PokemonListResponse {
//     results: Vec<PokemonEntry>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct PokemonEntry {
//     name: String,
//     url: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Define the URL for the Pokémon API
//     let url = "https://pokeapi.co/api/v2/pokemon?limit=151";

//     // Create a new HTTP client
//     let client = Client::new();

//     // Fetch the list of Pokémon
//     let response = client.get(url).send().await?.json::<PokemonListResponse>().await?;

//     // Prepare to collect detailed Pokémon data
//     let mut pokedex = Vec::new();

//     for (id, pokemon_entry) in response.results.iter().enumerate() {
//         // Fetch Pokémon details
//         let detail_response = client.get(&pokemon_entry.url).send().await?.json::<serde_json::Value>().await?;
//         let sprite = detail_response["sprites"]["front_default"]
//             .as_str()
//             .unwrap_or("")
//             .to_string();

//         // Store the data in the Pokedex
//         pokedex.push(Pokemon {
//             id: id + 1,
//             name: pokemon_entry.name.clone(),
//             url: pokemon_entry.url.clone(),
//             sprite,
//         });
//     }

//     // Define the path where the file will be saved
//     let file_path = PathBuf::from("C:\\Users\\Kliea\\Documents\\Development\\Poke\\pokefront\\poke-e\\public").join("pokedex.json");

//     // Serialize the Pokedex to a JSON string
//     let json_data = serde_json::to_string_pretty(&pokedex)?;

//     // Write the JSON string to the file
//     let mut file = File::create(&file_path)?;
//     file.write_all(json_data.as_bytes())?;

//     // Print the file path for confirmation
//     println!("Pokédex data successfully saved to {}", file_path.display());

//     Ok(())
// }
