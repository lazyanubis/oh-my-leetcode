/**
 * @author dawn
 * @date 2020/07/03
 */
public class Test25 {

    public static void main(String[] args) {
        ListNode head1 = new ListNode(1);
        ListNode head2 = new ListNode(2);
        ListNode head3 = new ListNode(3);
        ListNode head4 = new ListNode(4);
        ListNode head5 = new ListNode(5);
        head1.next = head2;
        head2.next = head3;
        head3.next = head4;
        head4.next = head5;
        System.out.println(new Test25().reverseKGroup(head1, 2));
    }

    public ListNode reverseKGroup(ListNode head, int k) {
        if (null == head) return null;
        if (null == head.next) return head;

        ListNode dummy = new ListNode(0);
        dummy.next = head;

        ListNode last = dummy;
        ListNode last2 = head;
        ListNode first = null;
        ListNode next = head;
        int i = 0;

        while (true) {
            if (null == next) {
                if (i == 0) break;
                do {
                    last.next = first.next;
                    first.next = next;
                    next = first;
                    first = last.next;
                } while (null != first);
                last.next = next;
                break;
            }
            last.next = next.next;
            next.next = first;
            first = next;
            next = last.next;

            i++;
            if (i == k) {
                last.next = first;
                last = last2;
                i = 0;
                last2 = next;
                first = null;
            }
        }

        return dummy.next;
    }
}
class ListNode {
    int val;
    ListNode next;
    ListNode(int x) { val = x; }

    @Override
    public String toString() {
        return "ListNode{" +
                "val=" + val +
                ", next=" + next +
                '}';
    }
}
