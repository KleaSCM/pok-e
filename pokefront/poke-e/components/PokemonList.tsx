import React from 'react';
import styles from '../styles/pokemonList.module.scss'; 

interface Pokemon {
  id: number;
  name: string;
  sprite: string;
}

interface PokemonListProps {
  pokedex: Pokemon[];
  onPokemonClick: (pokemon: Pokemon) => void;
  expandedPokemonId: number | null;
  onExpandPokemon: (id: number | null) => void;
  pokemonDetails: PokemonDetailData | null;
}

const PokemonList: React.FC<PokemonListProps> = ({ pokedex, onPokemonClick, expandedPokemonId, onExpandPokemon, pokemonDetails }) => {
  return (
    <div className={styles.pokemonList}>
      {pokedex.map((pokemon) => (
        <div 
          key={pokemon.id} 
          className={`${styles.pokemonCard} ${expandedPokemonId === pokemon.id ? styles.expanded : ''}`} 
          onClick={() => {
            onPokemonClick(pokemon);
            onExpandPokemon(pokemon.id);
          }}
        >
          <img className={styles.pokemonCardImg} src={pokemon.sprite} alt={pokemon.name} />
          <p>{pokemon.id}. {pokemon.name}</p>
          {expandedPokemonId === pokemon.id && pokemonDetails && (
            <div className={styles.pokemonDetails}>
              <h2>{pokemonDetails.name}</h2>
              <p>Height: {pokemonDetails.height}</p>
              <p>Weight: {pokemonDetails.weight}</p>
              <p>Base Experience: {pokemonDetails.base_experience}</p>
              <img src={pokemonDetails.sprites.front_default} alt={pokemonDetails.name} />
              <div>
                <h3>Types</h3>
                {pokemonDetails.types.map((typeInfo) => (
                  <p key={typeInfo.slot}>{typeInfo.type.name}</p>
                ))}
              </div>
              <div>
                <h3>Abilities</h3>
                {pokemonDetails.abilities.map((abilityInfo) => (
                  <p key={abilityInfo.ability.name}>{abilityInfo.ability.name}</p>
                ))}
              </div>
            </div>
          )}
        </div>
      ))}
    </div>
  );
};

export default PokemonList;
