Development History: "Leave the Dungeon" Project
Project Overview: A technical log of the design, implementation, and architectural evolution of a procedural Roguelike engine developed in Rust using the bracket-lib library.

Phase 1: Environment & Rendering
Integration of the bracket-lib framework.
Configuration of the ASCII-based terminal rendering system.
Implementation of the core game loop (tick function).

Phase 2: Procedural Generation
Implemented a 1D vector-based map system for memory efficiency.
Developed a "Room and Corridor" algorithm:
Randomized placement of Rect rooms with overlap validation.
L-shaped vertical and horizontal tunnels to ensure 100% connectivity.

Phase 3: Entity & State Management
Designed the State struct to track player coordinates, HP, map depth, and enemy positions.
Implemented level progression logic: depth tracking and specific exit conditions (* Goal vs > Stairs).

Phase 4: Gameplay Mechanics & AI
Turn-Based Logic: Converted a 60 FPS real-time engine into a turn-based tactical system using a player_moved flag.
Combat System: Implemented "Collision Combat"—enemies are removed and player HP is deducted upon coordinate intersection.
Enemy AI: Integrated random patrolling behavior that respects wall collisions and interacts with the player.

Phase 5: Refactor
Split the monolithic codebase into independent, modular files for better Separation of Concerns.

Technical Challenges & Solutions : 

The Rust Borrow Checker
Challenge: Attempting to mutate the enemies vector while simultaneously iterating over it caused compilation errors due to Rust’s ownership rules.
Solution: Used functional programming patterns. Utilized .retain() for filtering defeated enemies and shadow vectors (next_enemies) to store updated positions before re-assigning them to the main state.

Out-of-Bounds Memory Safety (at the start when walls dosen't exist)
Challenge: Moving entities beyond the 80x50 grid caused "index out of bounds" panics when accessing the 1D map vector.
Solution: Implemented strict coordinate sanitization using the .clamp() method for every movement calculation.

Fog of War (Field of View)
Challenge: Implementing a dynamic visibility system to hide unexplored tiles.
Outcome: This feature was omitted from the current version due to persistent calculation errors.

