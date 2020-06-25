public class LRUCache {
    class Node {
        int key;
        int value;
        Node prev;
        Node next;
    }
    private void addNode(Node node) {
        node.prev = head;
        node.next = head.next;
        head.next.prev = node;
        head.next = node;
    }
  private void removeNode(Node node){
    Node prev = node.prev;
    Node next = node.next;
    prev.next = next;
    next.prev = prev;
  }
  private Map<Integer, Node> cache = new HashMap();
  private int size;
  private int capacity;
  private Node head, tail;

  public LRUCache(int capacity) {
    this.size = 0;
    this.capacity = capacity;
    head = new Node();
    tail = new Node();
    head.next = tail;
    tail.prev = head;
  }

  public int get(int key) {
    Node node = cache.get(key);
    if (node == null) return -1;
    removeNode(node);
    addNode(node);
    return node.value;
  }

  public void put(int key, int value) {
    Node node = cache.get(key);
    if(node == null) {
      Node newNode = new Node();
      newNode.key = key;
      newNode.value = value;
      cache.put(key, newNode);
      addNode(newNode);
      ++size;
      if(size > capacity) {
        Node res = tail.prev;
        removeNode(res);
        cache.remove(res.key);
        --size;
      }
    } else {
      node.value = value;
      removeNode(node);
      addNode(node);
    }
  }
}