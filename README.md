# Leave the Dungeon

A classic roguelike game developed in Rust using the `bracket-lib` library. Explore procedurally generated dungeons, face enemies, and survive long enough to find the exit.

## 🌟 Características

* **Procedural Generation:** Each game creates a unique dungeon with randomly generated rooms and corridors.
* **Multiple Levels:** Descend through 3 levels of depth.
* **Basic Artificial Intelligence:** Enemies that patrol the dungeons randomly.
* **Combat System:** Crash into enemies to attack them. But be careful, they'll hurt you too!
* **Victory/Defeat Conditions:** Survive with your 3 Hearth Points (HP) until you reach the top floor to win, or die trying.
* **Modular Code:** Clean architecture separated into modules (`main`, `game`, `map`) with documented code.

## Controls

| **Key | Action** |
| :--- |:--- |
**Arrows (↑ ↓ ← →)** | Move player |
**Space** | Interact (Go down the stairs or take the exit) |
**Escape (Esc)** | Close the game (on the Victory or Game Over screen) |

## Map Legend

The game uses classic ASCII graphics. Here's what each symbol means:

* `@` **(Yellow):** Your character (Player).
* `x` **(Red):** Enemy. Touching it will subtract 1 HP.
* `.` **(Dark Green):** Walkable floor.
* `#` **(Gray):** Wall (You cannot pass through it).
* `>` **(Red):** Stairs to descend to the next level.
* `*` **(Gold):** The exit! Reach this point on the final level to win.

## Prerequisites

To compile and play this game, you will need to have **Rust** and its package manager **Cargo** installed.

If you don't have Rust installed, you can do so by following the official instructions:
[https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

## Installation and Execution

1. Clone this repository to your local machine:
   ```bash
   git clone <url-de-tu-repositorio>
   cd <nombre-de-tu-carpeta>
   
2. Compile and run the game directly with Cargo:
  ```bash
  cargo run

3. Project Structure
The source code is logically divided to facilitate maintenance and expansion:

- src/main.rs: Program entry point. Configures the terminal window and starts the game engine.

- src/game.rs: Contains the main game logic (the Game Loop), collision handling, combat, enemy AI, and screen rendering.

-  src/map.rs: Procedural dungeon generation engine. Handles excavating rooms, connecting them with corridors, and positioning entities.

 ## Additional Notes
The comments in the source code are written in French, detailing the internal workings of each function and state logic.
