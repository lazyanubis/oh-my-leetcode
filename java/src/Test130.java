import java.util.*;
import java.util.stream.Collectors;

/**
 * @author dawn
 * @date 2020/07/31
 */
public class Test130 {

    public static void main(String[] args) {
        
//        char[][] board = new char[][] {
//                new char[] { 'O','X','O','O','O','O','O','O','O'},
//                new char[] { 'O','O','O','X','O','O','O','O','X'},
//                new char[] { 'O','X','O','X','O','O','O','O','X'},
//                new char[] { 'O','O','O','O','X','O','O','O','O'},
//                new char[] { 'X','O','O','O','O','O','O','O','X'},
//                new char[] { 'X','X','O','O','X','O','X','O','X'},
//                new char[] { 'O','O','O','X','O','O','O','O','O'},
//                new char[] { 'O','O','O','X','O','O','O','O','O'},
//                new char[] { 'O','O','O','O','O','X','X','O','O'}
//        };
//        char[][] board = new char[][] {
//                new char[] { 'O','X','O','O','O','X'},
//                new char[] { 'O','O','X','X','X','O'},
//                new char[] { 'X','X','X','X','X','O'},
//                new char[] { 'O','O','O','O','X','X'},
//                new char[] { 'X','X','O','O','X','O'},
//                new char[] { 'O','O','X','X','X','X'}
//        };
        char[][] board = new char[][] {
                new char[] { 'X','X','X','X'},
                new char[] { 'X','O','O','X'},
                new char[] { 'X','X','O','X'},
                new char[] { 'X','O','X','X'}
        };
        new Test130().solve(board);
        System.out.println("================");
        for (int i = 0; i < board.length; i++) {
            System.out.println(Arrays.toString(board[i]));
        }
        System.out.println(board);
    }

    private boolean[] flag;
    public void solve(char[][] board) {
        if (null == board) return;

        if (board.length == 0 || board[0].length == 0) return;

        int height = board.length;
        int width = board[0].length;

        if (height <= 2 || width <= 2) return;

        flag = new boolean[width * height];
        boolean[][] mark = new boolean[height][];
        for (int i = 0; i < height; i++) mark[i] = new boolean[width];

        for (int i = 0; i < height; i++) if (board[i][0] == 'O') { mark[i][0] = true; Arrays.fill(flag, false); check(board, mark, i,  1); }
        for (int i = 0; i < height; i++) if (board[i][width - 1] == 'O') { mark[i][width - 1] = true; Arrays.fill(flag, false); check(board, mark, i,  width - 2); }
        for (int i = 0; i < width; i++) if (board[0][i] == 'O') { mark[0][i] = true; Arrays.fill(flag, false); check(board, mark, 1,  i);}
        for (int i = 0; i < width; i++) if (board[height - 1][i] == 'O') { mark[height - 1][i] = true; Arrays.fill(flag, false); check(board, mark, height - 2,  i); }

        for (int h = 0; h < height; h++) {
            for (int w = 0; w < width; w++) {
                if (!mark[h][w]) board[h][w] = 'X';
            }
        }
    }

    private void check(char[][] board, boolean[][] mark, int h, int w) {
        if (h <= 0 || w <= 0) return;
        if (board.length - 1 <= h || board[0].length - 1 <= w) return;
        int i = h * board[0].length + w;
        if (flag[i]) return;
        flag[i] = true;
        if (board[h][w] == 'X') return;
        mark[h][w] = true;
        check(board, mark,h - 1, w);
        check(board, mark,h + 1, w);
        check(board, mark, h,w - 1);
        check(board, mark, h,w + 1);
    }

}
