import { useEffect, useState } from "react";
import styles from '../styles/poke-e.module.css'; 
import PokemonList from '../components/PokemonList';
import PokemonDetail from "../components/PokemonDetail";

interface TypeInfo {
  slot: number;
  type: TypeName;
}

interface TypeName {
  name: string;
}

interface AbilityInfo {
  ability: AbilityName;
}

interface AbilityName {
  name: string;
}

interface Sprites {
  front_default: string;
}

interface Pokemon {
  id: number;
  name: string;
  sprite: string;
}

interface PokemonDetailData {
  name: string;
  id: number;
  height: number;
  weight: number;
  base_experience: number;
  types: TypeInfo[];
  abilities: AbilityInfo[];
  sprites: Sprites;
}

export default function Home() {
  const [pokedex, setPokedex] = useState<Pokemon[]>([]);
  const [selectedPokemon, setSelectedPokemon] = useState<PokemonDetailData | null>(null);
  const [expandedPokemonId, setExpandedPokemonId] = useState<number | null>(null);

  useEffect(() => {
    // Fetch the Pokedex JSON data
    fetch("/pokedex.json")
      .then((response) => response.json())
      .then((data) => setPokedex(data));
  }, []);

  async function handlePokemonClick(pokemon: Pokemon) {
    try {
      const response = await fetch(`http://localhost:3030/pokemon/${pokemon.id}`);
      if (!response.ok) {
        throw new Error(`Failed to fetch: ${response.statusText}`);
      }
      const data: PokemonDetailData = await response.json();
      setSelectedPokemon(data);
    } catch (error) {
      console.error('Failed to fetch Pokémon details:', error);
    }
  }

  function handleExpandPokemon(id: number | null) {
    setExpandedPokemonId(id);
  }

  return (
    <div className={styles.pokedex}>
      <h1>Poké</h1>
      <div className={styles.pokemonList}>
        <PokemonList 
          pokedex={pokedex} 
          onPokemonClick={handlePokemonClick} 
          expandedPokemonId={expandedPokemonId}
          onExpandPokemon={handleExpandPokemon}
          pokemonDetails={selectedPokemon}
        />
      </div>
      <div className={styles.pokemonDetail}>
        {selectedPokemon ? (
          <PokemonDetail pokemon={selectedPokemon} />
        ) : (
          <p>Select a Pokémon to see details</p>
        )}
      </div>
    </div>
  );
}
