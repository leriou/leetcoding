class Node:
    def __init__(self, key, value, prev, next):
        self.key = key
        self.value = value
        self.prev = prev
        self.next = next

class LRUCache:
    
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.size = 0
        self.cache = {}
        head = Node(0,0,None,None)
        tail = Node(0,0,None,None)
        head.next = tail
        tail.prev = head
        self.head = head
        self.tail = tail
    
    def append(self, node):
        node.prev = self.head
        node.next = self.head.next
        self.head.next.prev = node
        self.head.next = node
        
    def unlink(self, node):
        prev = node.prev
        next = node.next
        prev.next = next
        next.prev = prev

    def get(self, key: int) -> int:
        node = self.cache.get(key)
        if node is None:
            return -1
        else:
            self.unlink(node)
            self.append(node)
            return node.value

    def put(self, key: int, value: int) -> None:
        node = self.cache.get(key)
        if node is None:
            new_node = Node(key, value, None, None)
            self.cache[key] = new_node
            self.append(new_node)
            self.size += 1
            if self.size > self.capacity:
                last = self.tail.prev
                self.unlink(last)
                self.cache.pop(last.key)
                self.size -= 1
        else:
            node.value = value
            self.unlink(node)
            self.append(node)