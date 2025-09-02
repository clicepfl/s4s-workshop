#include <vector>
#include <optional>
#include <iostream>

struct Piece {
    char pieceType; // pieceType: 'M' pour pion, 'K' pour dame
    char pieceColor; // pieceColor: 'W' pour blanc, 'B' pour noir
    Piece(char type, char color) : pieceType(type), pieceColor(color) {}
};

struct Position {
    int row; // row: ligne de la cellule
    int column; // column: colonne de la cellule
};

struct Move {
    Position from; // from: cellule de départ
    Position to; // to: cellule d'arrivée
};

// Fonction pour trouver les coups à jouer
std::vector<Move> findMove(const std::vector<std::vector<std::optional<Piece>>>& board, char playerColor, std::vector<std::vector<Move>> possible_moves) {

    // TODO: Implémentez ici la logique pour choisir les coups à jouer et les retourner
    // Les coups doivent être retournés sous forme d'une liste d'objets Move,
    // Chaque objet Move représente un coup, avec une cellule de départ et une cellule d'arrivée
    // Les classes Position(row, column) et Move(from, to) sont fournies pour vous

    //std::cout << "Vous pouvez afficher des valeurs de votre programme ici: " << possible_moves[0][0].from.row << '\n' ;

    // possible_moves contient toutes les séquences de mouvements possibles.
    return possible_moves[0];
}

