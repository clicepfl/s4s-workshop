class Piece:
    # piece_type: M pour pion, K pour dame
    # piece_color: B pour noir, W pour blanc
    def __init__(self, piece_type, piece_color):
        self.piece_type = piece_type
        self.piece_color = piece_color

class Position:
    # row: ligne de la cellule
    # col: colonne de la cellule
    def __init__(self, row, col):
        self.row = row
        self.col = col

class Move:
    # start: Position de départ du coup
    # end: Position d'arrivée du coup
    def __init__(self, start, end):
        self.start = start
        self.end = end

    def __str__(self):
        return f"{self.start.row}{self.start.col},{self.end.row}{self.end.col};"

    def __repr__(self):
        return self.__str__()


def find_move(board, player_color, possible_moves):

    # TODO: Implémentez ici la logique pour choisir les coups à jouer et les retourner
    # Les coups doivent être retournés sous forme d'une liste d'objets Move,
    # Chaque objet Move représente un coup, avec une cellule de départ et une cellule d'arrivée
    # Les classes Position(row, column) et Move(start, end) sont fournies pour vous

    #print("Vous pouvez afficher des valeurs de votre programme ici:", possible_moves[0][0].start.row)

    # possible_moves contient toutes les séquences de mouvements possibles.
    moves = possible_moves[0]

    return moves

