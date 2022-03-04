# twenty48
Twenty48 is an implementation of the 2048 where users can compete against each other. The aim of the project is to demonstrate the capabilities of WebAssembly and wasmCloud, by using the same Rust game engine compiled to WebAssembly both client and server side.

The application was developed for demonstration purposes, initially for a presentation about WebAssembly

## Architecture
Game state is locally validated and advanced using the WebAssembly game engine. The game state consists of a random seed and the sequence of completed moves. This state is used for synching against the backend, which validates incoming states against the persisted states in the application database in a wasmCloud actor. A custom wasmCloud provider provides persistance in a mySQL database.

## Gameplay
Users can use the arrow keys or swipe the tiles to push them in any of the directions up, down, left or right. Tiles with the same value merge to a new tile with the sum of the values when pushed together, adding the value of the merged tile to the player's score. The game ends when there are no legal moves.

Users can share their game ID with other users as a link available through the share or copy buttons on the game screen. Games with the same game ID have the same random seed, making competition fair between players. Clicking on the rank button displays a leaderboard of the current game.

By clicking on the button with the refresh symbol, players can choose to start a new game, or choose to resume a previous game.