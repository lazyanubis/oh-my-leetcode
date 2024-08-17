/**
 * @author dawn
 * @date 2020/07/31
 */
public class Test61 {
    public static class ListNode {
      int val;
      ListNode next;
      ListNode(int x) { val = x; }
   }
    public static void main(String[] args) {
        ListNode head = new ListNode(1);
        ListNode head2 = new ListNode(2);
        ListNode head3 = new ListNode(3);
        ListNode head4 = new ListNode(4);
        ListNode head5 = new ListNode(5);
        head.next = head2;
        head2.next = head3;
        head3.next = head4;
        head4.next = head5;
        ListNode h = new Test61().rotateRight(head, 3);
        System.out.println();
    }
    public ListNode rotateRight(ListNode head, int k) {
        if (null == head) return null;
        if (k <= 0) return head;
        if (head.next == null) return head;

        ListNode temp = head;
        int size = 1;
        while (temp.next != null) {
            System.out.println(size);
            size++;
            temp = temp.next;
        }

        System.out.println(size);

        temp.next = head;

        k = k % size;

        for (int i = 0; i < size - k; i++) {
            temp = temp.next;
        }

        head = temp.next;
        temp.next = null;

        return head;
    }

}
