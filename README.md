# Switch Mood Recommendation

## Descripción

Switch Mood Recommendation es un smart contract simple en Solana que permite almacenar y actualizar el estado de ánimo de un usuario. Con base en el estado de ánimo guardado, el programa puede devolver una recomendación.

Este proyecto demuestra cómo crear e interactuar con un programa básico en la blockchain de Solana usando Anchor.

## Funcionalidades

* Guardar el estado de ánimo de un usuario en la blockchain
* Actualizar el estado de ánimo
* Consultar el estado de ánimo actual
* Obtener una recomendación basada en el estado de ánimo

## Tecnologías utilizadas

* Solana
* Rust
* Anchor Framework
* Solana Playground

## Estructura del proyecto

src/
lib.rs → lógica principal del smart contract

client/
scripts para interactuar con el programa

tests/
pruebas del programa

## Cómo funciona

1. El usuario envía su estado de ánimo.
2. El programa guarda ese estado de ánimo en la blockchain.
3. El programa puede devolver una recomendación dependiendo del estado de ánimo.

Ejemplo:

* Si el estado de ánimo = "feliz" → recomendación = "Sigue haciendo lo que te gusta"
* Si el estado de ánimo = "triste" → recomendación = "Tómate un descanso o escucha música"

## Cómo ejecutar el proyecto

1. Abrir el proyecto en Solana Playground.
2. Compilar el programa.
3. Desplegar el programa.
4. Ejecutar el cliente o las pruebas para interactuar con el smart contract.

## Autor

Miriam Duran Barrios

## Licencia

MIT
