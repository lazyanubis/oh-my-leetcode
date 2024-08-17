/**
 * @author dawn
 * @date 2020/07/05
 */
public class Test27 {

    public static void main(String[] args) {
        new Test27().strStr("hello", "ll");
    }

    public int strStr(String haystack, String needle) {


        // int L = needle.length(), n = haystack.length();
        // if (L == 0) return 0;

        // int pn = 0;
        // while (pn < n - L + 1) {
        //     // find the position of the first needle character
        //     // in the haystack string
        //     while (pn < n - L + 1 && haystack.charAt(pn) != needle.charAt(0)) ++pn;

        //     // compute the max match string
        //     int currLen = 0, pL = 0;
        //     while (pL < L && pn < n && haystack.charAt(pn) == needle.charAt(pL)) {
        //         ++pn;
        //         ++pL;
        //         ++currLen;
        //     }

        //     // if the whole needle string is found,
        //     // return its start position
        //     if (currLen == L) return pn - L;

        //     // otherwise, backtrack
        //     pn = pn - currLen + 1;
        // }
        // return -1;


        // if (null == haystack) return -1;
        // if (null == needle) return -1;
        // if (needle.isEmpty()) return 0;
        // if (haystack.isEmpty()) return -1;

        // mark:
        // for (int i = 0; i <= haystack.length() - needle.length(); i++) {
        //     if (haystack.charAt(i) == needle.charAt(0)) {
        //         for (int j = 1; j < needle.length(); j++) {
        //             if (haystack.charAt(i + j) != needle.charAt(j)) {
        //                 continue mark;
        //             }
        //         }
        //         return i;
        //     }
        // }

        // return -1;

        if (null == haystack) return -1;
        if (null == needle) return -1;
        if (needle.isEmpty()) return 0;
        if (haystack.isEmpty()) return -1;
        if (haystack.length() < needle.length()) return -1;

        if (1 == needle.length()) {
            for (int i = 0; i <= haystack.length() - needle.length(); i++) {
                if (haystack.charAt(i) == needle.charAt(0)) return i;
            }
            return -1;
        }

        int length = needle.length();
        long module = (1 << (5 * (length <= 12 ? length : 12))) - 1;

        long hash = caculate(needle) & module;
        long hash1 = caculate(haystack.substring(0, length)) & module;
        int i = 0;
        int max = haystack.length() - length + 1;
        while(true) {
            if (hash == hash1) {
                if (needle.equals(haystack.substring(i, i+length))) return i;
            }
            i++;
            if (i >= max) break;
            hash1 = ((hash1 << 5) + haystack.charAt(i + length - 1) - 'a') & module;
        }
        return -1;
    }

    private long caculate(String c) {
        long hash = 0;
        for (int i = 0; i < c.length(); i++) {
            hash = hash << 5;
            hash += c.charAt(i) - 'a';
        }
        return hash;
    }



}
