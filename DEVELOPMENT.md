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
