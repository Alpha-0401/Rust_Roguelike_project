# Roguelike en Rust

Projet academique pour initialiser en Rust.

# Conditions préalables (Linux)
Pour compiler et executer le jeu en sistèmes Linux (Ubuntu/Debian), il faut installer les dépendances système pour la gestion des fenêtres et des polices. Exécutez la commande suivante dans votre terminal :

\`\`\`bash
sudo apt update
sudo apt install pkg-config libfontconfig1-dev 
sudo apt install libx11-dev libasound2-dev libudev-dev
sudo apt install libfreetype6-dev fonts-liberation
\`\`\`

*(Note : en Windows ou macOS, Cargo devrait télécharger et installer automatiquement les outils nécessaires si vous avez installé les outils de compilation.)*

## Comment exécuter
Une fois les dépendances installées, il suffit d'utiliser Cargo à la racine du projet :

\`\`\`bash
cargo run
\`\`\`
