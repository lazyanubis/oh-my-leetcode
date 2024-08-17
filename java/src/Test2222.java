import java.util.List;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.Arrays;
import java.util.stream.Collectors;
public class Test2222 {

    public static void main(String[] args) {
//         System.out.println(new Test2222().isMatch("aa", "a*"));
//         System.out.println(new Test2222().isMatch("aab", "c*a*b"));
        // System.out.println(new Solution().isMatch("aaa", "ab*ac*a"));
        //System.out.println(new Solution().isMatch("", "c*"));
        //System.out.println(new Solution().isMatch("aaa", "ab*a*c*a"));
//        System.out.println(new Test2222().isMatch("aaba", "ab*a*c*a"));
//        System.out.println(new Test2222().isMatch("ab",".*..c*"));
//        System.out.println(new Test2222().isMatch("abcdede", "ab.*de"));
//        System.out.println(new Test2222().isMatch("cabbbbcbcacbabc",".*b.*.ab*.*b*a*c"));
        System.out.println(new Test2222().isMatch("bbbba",".*a*a"));
    }

    public boolean isMatch(String s, String p) {
        return match(s, p, null, false);
    }
    private boolean match(String s, String p, Character lastP, boolean star) {
        if (s.length() == 0 && p.length() == 0) return true;
        if (p.length() == 0) {
            if (null != lastP && star) {
                if ('.' == lastP) return true;
                int index = 0;
                while(true) {
                    if (s.charAt(index) != lastP) return false;
                    index++;
                    if (s.length() <= index) return true;
                }
            }
            return false;
        }
        if (s.length() == 0) {
            if (null != lastP && p.charAt(0) == '*') return match("", p.substring(1), null, false);
            int index = 0;
            boolean star2 = false;
            char c;
            while(true) {
                c = p.charAt(index);
                if (star2) {
                    if (c == '*') {

                    } else {
                        return false;
                    }
                } else {
                    if (c == '*') {
                        return false;
                    } else {

                    }
                }
                index++;
                if (p.length() == index && star2) return true;
                if (p.length() <= index) return false;
                star2 = !star2;
            }
        }

        char firstC = s.charAt(0);
        char firstP = p.charAt(0);
        if (null != lastP) {
            if ('.' == lastP) {
                if (star) {
                    if (match(s.substring(1), p, lastP, true)) return true;
                    if (match(s.substring(1), p, null, false)) return true;
                    return match(s, p, null, false);
                } else {
                    if (firstP == '*') {
                        return match(s, p.substring(1), lastP, true);
                    } else {
                        return match(s, p, null, false);
                    }
                }
            } else {
                if (star) {
                    if (firstC == lastP) {
                        if (match(s.substring(1), p, lastP, true)) return true;
                        if (match(s.substring(1), p, null, false)) return true;
                        return match(s, p, null, false);
                    } else {
                        return match(s, p, null, false);
                    }
                } else {
                    if (firstP == '*') {
                        return match(s, p.substring(1), lastP, true);
                    } else {
                        return match(s, p, null, false);
                    }
                }
            }
        } else {
            if (firstP == '*') return false;
            if (2 <= p.length() && p.charAt(1) == '*') {
                if (match(s, p.substring(2), null, false)) return true;
            }
            if ('.' == firstP) return match(s.substring(1), p.substring(1), firstP, false);
            if (firstC == firstP) {
                return match(s.substring(1), p.substring(1), firstC, false);
            } else {
                return false;
            }
        }
    }

//    public boolean isMatch(String s, String p) {
//        for (String pp : getP(p)) {
//            System.out.println("pppp -> " + pp);
//            if (match(s, pp, null, false)) return true;
//        }
//        return false;
//    }

//    private static List<String> getP(String p) {
//        if (0 == p.length()) return Arrays.asList(p);
//        List<String> list = new LinkedList<>();
//        String t = "";
//        for (int i = 0; i < p.length(); i++) {
//            char c = p.charAt(i);
//            if (c == '*') {
//                switch (t.length()) {
//                    case 0: list.add("*"); continue;
//                    case 1: list.add(t + '*'); t = ""; continue;
//                    default: list.add(t.substring(0, t.length() - 1));
//                    list.add(t.substring(t.length() - 1) + '*'); t = ""; continue;
//                }
//            }
//            t += c;
//        }
//        if (0 < t.length()) list.add(t);
//        List<String> l = new LinkedList<>();
//        l.add("");
//        for (String s : list) {
//            if (s.length() == 2 && s.endsWith("*")) {
//                for (String ll : new ArrayList<>(l)) {
//                    l.add(ll + s);
//                }
//            } else {
//                l = l.stream().map(ll -> ll + s).collect(Collectors.toList());
//            }
//        }
//        return l;
//    }

//    private boolean match(String s, String p, Character lastP, boolean star) {
//         System.out.println(s + " " + p + " " + lastP + " " + star);
//        if (s.length() == 0 && p.length() == 0) return true;
//        if (0 < s.length() && 0 < p.length()) {
//            char lp = p.charAt(p.length() - 1);
//            if ('*' != lp) {
//                if ('.' == lp || lp == s.charAt(s.length() - 1)) {
//                    return match(s.substring(0, s.length() - 1),
//                            p.substring(0, p.length() - 1), null, false);
//                }
//            }
//        }
//        if (p.length() == 0) {
//            if (null != lastP && star) {
//                if ('.' == lastP) return true;
//                int index = 0;
//                while(true) {
//                    if (s.charAt(index) != lastP) return false;
//                    index++;
//                    if (s.length() <= index) return true;
//                }
//            }
//            return false;
//        }
//        if (s.length() == 0) {
//            if (null != lastP && p.charAt(0) == '*') return match("", p.substring(1), null, false);
//            int index = 0;
//            boolean star2 = false;
//            char c;
//            while(true) {
//                c = p.charAt(index);
//                if (star2) {
//                    if (c == '*') {
//
//                    } else {
//                        return false;
//                    }
//                } else {
//                    if (c == '*') {
//                        return false;
//                    } else {
//
//                    }
//                }
//                index++;
//                if (p.length() == index && star2) return true;
//                if (p.length() <= index) return false;
//                star2 = !star2;
//            }
//        }
//
//        char firstC = s.charAt(0);
//        char firstP = p.charAt(0);
//        if (null != lastP) {
//            if ('.' == lastP) {
//                if (star) {
//                    return match("", p, null, false);
//                } else {
//                    if (firstP == '*') {
//                        return match(s, p.substring(1), lastP, true);
//                    } else {
//                        return match(s, p, null, false);
//                    }
//                }
//            } else {
//                if (star) {
//                    if (firstC == lastP) {
//                        return match(s.substring(1), p, firstC, true);
//                    } else {
//                        return match(s, p, null, false);
//                    }
//                } else {
//                    if (firstP == '*') {
//                        return match(s, p.substring(1), lastP, true);
//                    } else {
//                        return match(s, p, null, false);
//                    }
//                }
//            }
//        } else {
//            if (firstP == '*') return false;
//            if ('.' == firstP) return match(s.substring(1), p.substring(1), '.', false);
//            if (firstC == firstP) {
//                return match(s.substring(1), p.substring(1), firstC, false);
//            } else {
////                if (2 <= p.length() && p.charAt(1) == '*')
////                    return match(s, p.substring(2), null, false);
//                return false;
//            }
//        }

//    }
}