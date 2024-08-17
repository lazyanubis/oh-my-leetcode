import java.util.*;
import java.util.stream.Collector;
import java.util.stream.Collectors;

/**
 * @author dawn
 * @date 2020/07/31
 */
public class Test149 {

    public static void main(String[] args) {
//        int r = new Test149().maxPoints(new int[][]{
//                new int[]{1,1},
//                new int[]{3,2},
//                new int[]{5,3},
//                new int[]{4,1},
//                new int[]{2,3},
//                new int[]{1,4},
//        });
        int r = new Test149().maxPoints(new int[][]{
                new int[]{0,0},
                new int[]{1,65536},
                new int[]{65536,0},
        });
        System.out.println(r);
    }

    private static class Point {
        int x;
        int y;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public boolean equals(Object o) {
            if (this == o) return true;
            if (o == null || getClass() != o.getClass()) return false;
            Point point = (Point) o;
            return x == point.x &&
                    y == point.y;
        }

        @Override
        public int hashCode() {
            return Objects.hash(x, y);
        }
    }

    private HashMap<Point, Integer> map;
    private Point[] ps;
    private int max = 0;

    public int maxPoints(int[][] points) {
        if (null == points) return 0;

        if (0 == points.length) return 0;

        if (1 == points.length) return 1;


        map = new HashMap<>();

        for (int[] point : points) {
            Point p = new Point(point[0], point[1]);
            if (map.containsKey(p)) { map.put(p, map.get(p) + 1); }
            else { map.put(p, 1); }
        }

        ps = new ArrayList<>(map.keySet()).toArray(new Point[]{});
        if (1 == ps.length) { return map.get(ps[0]); }
        if (2 == ps.length) { return map.get(ps[0]) + map.get(ps[1]); }

        maxPoints(0);

        return max;
    }

    private void maxPoints(int start) {

//        System.out.println("start -> " + start);

        Point p1 = ps[start];
        for (int i = start + 1; i <= ps.length; i++) {
//            System.out.println("i -> " + i);

            Point p2 = ps[i];
            long dy = p2.y - p1.y;
            long dx = p2.x - p1.x;

            int m = map.get(p1) + map.get(p2);

            for (int j = i + 1; j < ps.length; j++) {
//                System.out.println("jjj -> " + j);
                Point p3 = ps[j];
                long dy2 = p3.y - p1.y;
                long dx2 = p3.x - p1.x;
                if (dy * dx2 == dy2 * dx) { m += map.get(p3); }
            }

            if (max < m) { max = m; }

        }

        maxPoints(start + 1);
    }


}
