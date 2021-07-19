class Node:
    def __init__(self):
        self.next: Node = None
        self.val = None

    def __init__(self, val, node):
        self.next = node
        self.val = val

class Stack:
    def __init__(self):
        self.root: Node = None
        self.number_of_elements = 0

    def size(self) -> int:
        return self.number_of_elements

    def push(self, val):
        self.root = Node(val, self.root)
        self.number_of_elements = self.number_of_elements + 1

    def top(self):
        return self.root.val

    def pop(self):
        if self.number_of_elements:
            self.root = self.root.next
            self.number_of_elements = self.number_of_elements - 1

if __name__ == "__main__":
    stack = Stack()
    for i in range(0, 5):
        stack.push(i)

    while stack.size():
        print("{}".format(stack.top()))
        stack.pop()
