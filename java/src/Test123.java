import java.util.ArrayList;
import java.util.List;

/**
 * @author dawn
 * @date 2020/07/31
 */
public class Test123 {

    public static void main(String[] args) {
//        System.out.println(0 == new Test123().maxProfit(new int[] { 7,6,4,3,1 }));
//        System.out.println(17 == new Test123().maxProfit(new int[] { 1,2,4,2,5,7,2,4,9,0,9 }));
//        System.out.println(12 == new Test123().maxProfit(new int[] { 1,1,2,2,1,1,3,3,2,4,6,3,10,5,3 }));
        System.out.println(14 == new Test123().maxProfit(new int[] { 2,6,8,7,8,7,9,4,1,2,4,5,8 }));
//        System.out.println(11 == new Test123().maxProfit(new int[] { 8,6,4,3,3,2,3,5,8,3,8,2,6 }));
    }

    public int maxProfit(int[] prices) {
        if(prices==null || prices.length==0) {
            return 0;
        }
        int n = prices.length;
        //定义三维数组，第i天、交易了多少次、当前的买卖状态
        int[][][] dp = new int[n][3][2];
        //初始化第一天，这里的dp[0][2][1]可以不用管，后面也不会用到
        dp[0][0][0] = 0;
        dp[0][0][1] = -prices[0];
        dp[0][1][0] = 0;
        dp[0][1][1] = -prices[0];
        dp[0][2][0] = 0;
        dp[0][2][1] = -prices[0];
        for(int i=1;i<n;++i) {
            //dp[i][0][0]相当于初始状态，它只能从初始状态转换来
            dp[i][0][0] = dp[i-1][0][0];
            //处理第一次买入、第一次卖出
            dp[i][0][1] = Math.max(dp[i-1][0][1],dp[i-1][0][0]-prices[i]);
            dp[i][1][0] = Math.max(dp[i-1][1][0],dp[i-1][0][1]+prices[i]);
            //处理第二次买入、第二次卖出
            dp[i][1][1] = Math.max(dp[i-1][1][1],dp[i-1][1][0]-prices[i]);
            dp[i][2][0] = Math.max(dp[i-1][2][0],dp[i-1][1][1]+prices[i]);
        }
        //返回最大值
        int a = Math.max(dp[n-1][0][0],dp[n-1][0][1]);
        int b = Math.max(dp[n-1][1][0],dp[n-1][1][1]);
        return Math.max(Math.max(a,b),dp[n-1][2][0]);


//        if (null == prices) { return 0; }
//
//        if (prices.length <= 1) { return 0; }
//
//        int v1, s1, e1;
//        int v2, s2, e2;
//
//        s1 = findLow(prices, 0);
//        if (-1 == s1 || s1 == prices.length - 1) { return 0; }
//        e1 = findHigh(prices, s1 + 1);
//        v1 = prices[e1] - prices[s1];
//
//        s2 = findLow(prices, e1 + 1);
//        if (-1 == s2) { return v1; }
//        e2 = findHigh(prices, s2 + 1);
//        if (-1 == e2) { return v1; }
//        v2 = prices[e2] - prices[s2];
//
//        int v3, s3 = findLow(prices, e2 + 1);
//        int s33, e33 = e2;
//        while (true) {
//            s33 = findLow(prices, e33 + 1);
//            if (-1 == s33) { break; }
//            e33 = findHigh(prices, s33 + 1);
//            if (-1 == e33) { break; }
//            s3 = prices[s3] < prices[s33] ? s3 : s33;
//            v3 = prices[e33] - prices[s3];
//
//            int d1 = v1 + v2;
//            int d2 = v1 + v3;
//            int d3 = v2 + v3;
//            int d4 = prices[e2] - prices[s1] + v3;
//            int d5 = v1 + prices[e33] - prices[s2];
//
//            int max = max(d1, d2, d3, d4, d5);
//
//            if (d1 >= max) {
//
//            } else if (d2 >= max) {
//                v2 = v3; s2 = s3; e2 = e33;
//                s3 = findLow(prices, e2 + 1);
//            } else if (d3 >= max) {
//                v1 = v2; s1 = s2; e1 = e2;
//                v2 = v3; s2 = s3; e2 = e33;
//                s3 = findLow(prices, e2 + 1);
//            } else if (d4 >= max) {
//                v1 = prices[e2] - prices[s1]; e1 = e2;
//                v2 = v3; s2 = s3; e2 = e33;
//                s3 = findLow(prices, e2 + 1);
//            } else if (d5 >= max) {
//                v2 = prices[e33] - prices[s2]; e2 = e33;
//                s3 = findLow(prices, e2 + 1);
//            }
//
//        }
//
//        return v1 + v2;
    }

    private int max(int... values) {
        int v = values[0];
        for (int i = 1; i < values.length; i++) {
            if (v < values[i]) {
                v = values[i];
            }
        }
        return v;
    }

    private int findHigh(int[] prices, int start) {
        if (start >= prices.length) { return -1; }
        if (prices.length - 1 == start) { return start; }
        while(start < prices.length - 1) {
            if (prices[start] <= prices[start + 1]) {
                start++;
            } else {
                break;
            }
        }
        return start;
    }

    private int findLow(int[] prices, int start) {
        if (start >= prices.length) { return -1; }
        if (prices.length - 1 == start) { return start; }
        while(start < prices.length - 1) {
            if (prices[start] > prices[start + 1]) {
                start++;
            } else {
                break;
            }
        }
        return start;
    }

}
