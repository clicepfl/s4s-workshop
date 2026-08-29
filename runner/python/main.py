import socket
import os

from script import Move,Position,Piece,find_move

def main():
    board = [
        [None for _ in range(10)] for _ in range(10)
    ]

    # Lecture de la couleur du joueur depuis la console
    player_color = input().strip()[0]

    # Parsage du plateau de jeu depuis la console
    for r in range(10):
        line = input().strip()
        row = line.split(",")
        for c, piece_code in enumerate(row):
            if piece_code:
                board[r][c] = Piece(piece_code[0], piece_code[1])
            else:
                board[r][c] = None

    # Read possible moves from stdin
    possible_moves = []
    pmoves_in = input().strip()
    for moveseq in pmoves_in.split(';'):
        if moveseq:
            curr_moveseq = []
            for move in moveseq.split(':'):
                if move:
                    curr_moveseq.append(Move(Position(int(move[0]), int(move[1])), Position(int(move[3]), int(move[4]))))
            possible_moves.append(curr_moveseq)

    # Appel de la fonction findMove pour trouver les coups à jouer
    moves = find_move(board, player_color, possible_moves)

    if not moves:
        raise Exception("No moves were returned.")
        return

    #print("sending program output to", os.environ['SOCK'])

    # Envoi des coups trouvés à la console
    move_out = ""
    for move in moves:
        move_out += move.__str__()
    with socket.socket(socket.AF_UNIX, socket.SOCK_STREAM) as client:
        client.connect(os.environ['SOCK'])
        client.send(move_out.encode("ascii"))
        client.close()


if __name__ == "__main__":
    main()
