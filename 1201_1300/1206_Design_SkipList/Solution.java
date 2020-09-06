class Skiplist {
    final double PROMOTE_RATE = 0.5;
    Node head, tail;
    int maxLevel;

    class Node {
        int val;
        Node left, right, up, down;
        Node(int val) {
            this.val = val;
        }
    }

    public Skiplist() {
        head = new Node(Integer.MIN_VALUE);
        tail = new Node(Integer.MAX_VALUE);
        head.right = tail;
        tail.left = head;
    }

    public boolean search(int target) {
        return findNode(target).val == target;
    }

    Node findNode(int val) {
        Node n = head;
        while (true) {
            while (n.right.val != Integer.MAX_VALUE
                    && n.right.val <= val) {
                n = n.right;
            }
            if (n.down == null) {
                break;
            }
            n = n.down;
        }
        return n;
    }

    private void appendNode(Node preNode, Node newNode){
        newNode.left=preNode;
        newNode.right=preNode.right;
        preNode.right.left=newNode;
        preNode.right=newNode;
    }

    public void add(int num) {
        Node preNode = findNode(num);
        // if (preNode.val == num) return;
        Node node = new Node(num);
        appendNode(preNode, node);
        int currentLevel = 0;
        Random random = new Random();
        while (random.nextDouble() < PROMOTE_RATE) {
            if (currentLevel == maxLevel) addLevel();
            while (preNode.up == null) {
                preNode = preNode.left;
            }
            preNode = preNode.up;
            Node upperNode = new Node(num);
            appendNode(preNode, upperNode);
            upperNode.down = node;
            node.up = upperNode;
            node = upperNode;
            currentLevel++;
        }
    }

    private void addLevel(){
        maxLevel++;
        Node p1=new Node(Integer.MIN_VALUE);
        Node p2=new Node(Integer.MAX_VALUE);
        p1.right=p2;
        p2.left=p1;
        p1.down=head;
        head.up=p1;
        p2.down=tail;
        tail.up=p2;
        head=p1;
        tail=p2;
    }

    public boolean erase(int num) {
        Node removedNode = findNode(num);
        if(removedNode.val != num){
            return false;
        }
        int currentLevel=0;
        while (removedNode != null){
            removedNode.right.left = removedNode.left;
            removedNode.left.right = removedNode.right;
            if(currentLevel != 0
                    && removedNode.left.val == Integer.MIN_VALUE
                    && removedNode.right.val == Integer.MAX_VALUE){
                removeLevel(removedNode.left);
            }else {
                currentLevel++;
            }
            removedNode = removedNode.up;
        }
        return true;
    }

    private void removeLevel(Node leftNode){
        Node rightNode = leftNode.right;
        if(leftNode.up == null){
            leftNode.down.up = null;
            rightNode.down.up = null;
        }else {
            leftNode.up.down = leftNode.down;
            leftNode.down.up = leftNode.up;
            rightNode.up.down = rightNode.down;
            rightNode.down.up = rightNode.up;
        }
        maxLevel--;
    }
}

/**
 * Your Skiplist object will be instantiated and called as such:
 * Skiplist obj = new Skiplist();
 * boolean param_1 = obj.search(target);
 * obj.add(num);
 * boolean param_3 = obj.erase(num);
 */