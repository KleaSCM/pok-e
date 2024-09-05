import { useEffect, useState } from "react";
import styles from '../styles/poke-e.module.css'; 

interface Pokemon {
  id: number;
  name: string;
  sprite: string;
}

export default function Home() {
  const [pokedex, setPokedex] = useState<Pokemon[]>([]);

  useEffect(() => {
    // Fetch the Pokedex JSON data
    fetch("/pokedex.json")
      .then((response) => response.json())
      .then((data) => setPokedex(data));
  }, []);

  return (
    <div className={styles.pokedex}>
      <h1>Poké</h1>
      <div className={styles.pokemonList}>
        {pokedex.map((pokemon) => (
          <div key={pokemon.id} className={styles.pokemonCard}>
            <img className={styles.pokemonCardImg} src={pokemon.sprite} alt={pokemon.name} />
            <p>{pokemon.id}. {pokemon.name}</p>
          </div>
        ))}
      </div>
    </div>
  );
}
