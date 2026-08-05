package s4s;

import java.io.FileWriter;
import java.io.IOException;
import java.util.List;
import java.util.ArrayList;
import java.util.Scanner;
import java.net.UnixDomainSocketAddress;
import java.nio.channels.SocketChannel;
import java.nio.ByteBuffer;
import java.nio.charset.Charset;
import java.text.StringCharacterIterator;
import s4s.Base;
import s4s.Base.Position;
import s4s.Base.Move;
import s4s.Base.Piece;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        Piece[][] board = new Piece[10][10];

        // Lecture de la couleur du joueur depuis la console
        char playerColor = scanner.nextLine().charAt(0);

        // Parsage du plateau de jeu depuis la console
        for (int r = 0; r < 10; r++) {
            String line = scanner.nextLine();
            String[] row = line.split(",", -1);
            Piece[] pieceRow = new Piece[row.length];
            for (int c = 0; c < row.length; c++) {
                String pieceCode = row[c];
                if (!pieceCode.isEmpty()) {
                    pieceRow[c] = new Piece(pieceCode.charAt(0), pieceCode.charAt(1));
                } else {
                    pieceRow[c] = null;
                }
            }

            board[r] = pieceRow;
        }

        // Find legal moves
        String movesIn = scanner.nextLine();
        StringCharacterIterator characters = new StringCharacterIterator(movesIn);
        ArrayList<ArrayList<Move>> possibleMoves = new ArrayList<ArrayList<Move>>();
        ArrayList<Move> currentMoveseq = new ArrayList<Move>();
        while (characters.current() != StringCharacterIterator.DONE) {

            // If it is ';' add the current sequence to the move list, start a new sequence
            if (characters.current() == ';') {
                possibleMoves.add(currentMoveseq);
                currentMoveseq = new ArrayList<Move>();
            }

            // If it is a number, add the whole move to the current sequence
            if (characters.current() - '0' >= 0 && characters.current() - '0' <= 9) {
                Position from = new Position(characters.current() - '0', characters.next() - '0');
                characters.next();
                Position to = new Position(characters.next() - '0', characters.next() - '0');
                currentMoveseq.add(new Move(from, to));
            }

            characters.next();
        }

        scanner.close();

        // Appel de la fonction findMove pour trouver les coups à jouer
        List<Move> moves = Base.findMove(board, playerColor, possibleMoves);

        if (moves == null) {
            return;
        }

        // Format moves
        String moves_out = "";
        for (Move move : moves) {
            moves_out +=
                move.from().row() + "" + move.from().column() + ","
                + move.to().row() + "" + move.to().column() + ";";
        }

        // Send moves
        SocketChannel socket;
        try {
            socket = SocketChannel.open(UnixDomainSocketAddress.of(System.getenv("SOCK")));
            socket.write(Charset.forName("UTF-8").encode(moves_out));
            socket.close();
        } catch (IOException e) {
            System.out.println(e);
        }
    }
}
