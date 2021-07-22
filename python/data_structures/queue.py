class Node:
    def __init__(self, val):
        self.val = val
        self.next = None

class Queue:
    def __init__(self):
        self.first = None
        self.last = None
        self.num_elems = 0

    def push(self, val):
        new_item = Node(val)
        if self.num_elems:
            self.last.next = new_item
        else:
            self.first = new_item
        self.last = new_item
        self.num_elems = self.num_elems + 1

    def pop(self):
        if self.num_elems:
            self.first = self.first.next
            self.num_elems = self.num_elems - 1

    def front(self):
        if self.num_elems:
            return self.first.val
        else:
            return None

    def back(self):
        if self.num_elems:
            return self.last.val
        else:
            return None

    def size(self) -> int:
        return self.num_elems

    def empty(self) -> bool:
        if self.num_elems:
            return False
        else:
            return True

if __name__ == "__main__":
    queue = Queue()

    for i in range(0,5):
        queue.push(i)
        assert queue.back() == i
        assert queue.size() == i+1

    i = 0
    while not queue.empty():
        assert queue.front() == i
        i = i+1
        queue.pop()
