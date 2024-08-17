import java.util.BitSet;

/**
 * @author dawn
 * @date 2020/07/14
 */
public class Test37 {

    public static void main(String[] args) {
        char[][] board = new char[][] {
                {'.','.','9','7','4','8','.','.','.'},
                {'7','.','.','.','.','.','.','.','.'},
                {'.','2','.','1','.','9','.','.','.'},
                {'.','.','7','.','.','.','2','4','.'},
                {'.','6','4','.','1','.','5','9','.'},
                {'.','9','8','.','.','.','3','.','.'},
                {'.','.','.','8','.','3','.','2','.'},
                {'.','.','.','.','.','.','.','.','6'},
                {'.','.','.','2','7','5','9','.','.'}
        };
        new Test37().solveSudoku(board);
        System.out.println(123);
    }

    public void solveSudoku(char[][] board) {
        BitSet[][] b = new BitSet[9][9];
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                if (board[r][c] == '.') {
                    b[r][c] = new BitSet(9);
                    for (int i = 1; i <= 9; i++) {
                        b[r][c].set(i);
                    }
                } else {
                    b[r][c] = new BitSet(9);
                    b[r][c].set(board[r][c] - '0');
                }
            }
        }
        b = solve(b);
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                if (board[r][c] == '.') {
                    for (int i = 1; i <= 9; i++) {
                        if (b[r][c].get(i)) {
                            board[r][c] = (char) (i + '0');
                            break;
                        }
                    }
                }
            }
        }

    }

    public BitSet[][] solve(BitSet[][] board) {
        clean(board);
        int rr = -1;
        int cc = -1;
        mark:
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                if (board[r][c].cardinality() > 1) {
                    rr = r; cc = c;
                    break mark;
                }
                if (board[r][c].cardinality() == 0) {
                    return null;
                }
            }
        }
        if (rr == -1) return board;

        for (int i = 1; i <= 9; i++) {
            if (board[rr][cc].get(i)) {
                BitSet[][] t = copy(board);
                t[rr][cc].clear();
                t[rr][cc].set(i);
                t = solve(t);
                if (null != t) {
                    return t;
                }
            }
        }

        return null;
    }

    public BitSet[][] copy(BitSet[][] board) {
        BitSet[][] copy = new BitSet[9][9];
        for (int r = 0; r < 9; r++) {
            for (int c = 0; c < 9; c++) {
                if (board[r][c].cardinality() == 1) {
                    copy[r][c] = board[r][c];
                } else {
                    copy[r][c] = new BitSet();
                    copy[r][c].or(board[r][c]);
                }
            }
        }
        return copy;
    }

    public void clean(BitSet[][] board) {
        boolean changed;
        do {
            changed = false;
            for (int r = 0; r < 9; r++) {
                for (int c = 0; c < 9; c++) {
                    if (board[r][c].cardinality() == 1) continue;

                    mark:
                    for (int i = 1; i <= 9; i++) {
                        if (board[r][c].get(i)) {
                            for (int rr = 0; rr < 9; rr++) {
                                if (rr != r && board[rr][c].cardinality() == 1 && board[rr][c].get(i)) {
                                    board[r][c].clear(i);
                                    changed = true;
                                    continue mark;
                                }
                            }

                            for (int cc = 0; cc < 9; cc++) {
                                if (cc != c && board[r][cc].cardinality() == 1 && board[r][cc].get(i)) {
                                    board[r][c].clear(i);
                                    changed = true;
                                    continue mark;
                                }
                            }

                            for (int rr = r / 3 * 3; rr < r / 3 * 3 + 3; rr++) {
                                for (int cc = c / 3 * 3; cc < c / 3 * 3 + 3; cc++) {
                                    if (!(rr == r && cc == c) && board[rr][cc].cardinality() == 1 && board[rr][cc].get(i)) {
                                        board[r][c].clear(i);
                                        changed = true;
                                        continue mark;
                                    }
                                }
                            }

                        }
                    }

                    if (board[r][c].cardinality() == 0) {
                        return;
                    }

                }
            }
        } while (changed);
    }

}
