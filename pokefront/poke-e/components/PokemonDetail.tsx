import React from 'react';
import styles from '../styles/poke-e.module.css';

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

interface PokemonDetail {
  name: string;
  id: number;
  height: number;
  weight: number;
  base_experience: number;
  types: TypeInfo[];
  abilities: AbilityInfo[];
  sprites: Sprites;
}

interface PokemonDetailProps {
  pokemon: PokemonDetail;
}

const PokemonDetail: React.FC<PokemonDetailProps> = ({ pokemon }) => {
  return (
    <div className={styles.pokemonDetail}>
      <h2>{pokemon.name}</h2>
      <p>Height: {pokemon.height}</p>
      <p>Weight: {pokemon.weight}</p>
      <p>Base Experience: {pokemon.base_experience}</p>
      <img src={pokemon.sprites.front_default} alt={pokemon.name} />
      <div>
        <h3>Types</h3>
        {pokemon.types.map((typeInfo) => (
          <p key={typeInfo.slot}>{typeInfo.type.name}</p>
        ))}
      </div>
      <div>
        <h3>Abilities</h3>
        {pokemon.abilities.map((abilityInfo) => (
          <p key={abilityInfo.ability.name}>{abilityInfo.ability.name}</p>
        ))}
      </div>
    </div>
  );
};

export default PokemonDetail;

