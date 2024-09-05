use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;


#[derive(Debug, Serialize, Deserialize)]

struct Pokemon{
    name: String,
    url: String,
    id: usize,
    sprite: string,

}

#[tokio::main]
async fn main() -> result<(), Error> {
    // call the pokemin API and fetch all the poke
    let response = reqwest::get("https://pokeapi.co/api/v2/pokemon?limit=151")
    .await?
    .json::<serde_json::Value()
    .await?;

    //extract names / urls
    let results = response["results"].as_array().unwrap();
    let mut pokedex =Vec::new();

    for (id, pokemon) in results.iter().enumerate(){
        let name = pokemon["name"].as_str().unwrap().to_string();
        let url = pokemon["url"].as_str().unwrap().to_string();

        // fetch pokemation
        let detail_response = reqwest::get(&url).await?.json::<serde_json::Value>().await?;
        let sprite = detail_response["sprites"]["front_default"]
        .as_str()
        .unwrap()
        .to_string()


        //store the data in pokedex

        pokedex.push(Pokemon {
            id: id +1,
            name,
            url,
            sprite,
            }
        );
    }
}

// now lets save to json


