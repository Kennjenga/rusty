class Node:
    def __init__(self, data):
        self.data = data
        self.next = None

class LinkedList:
    def __init__(self):
        self.head = None

    def append(self, data):
        new_node = Node(data)
        if not self.head:
            self.head = new_node
            return
        last_node = self.head
        while last_node.next:
            last_node = last_node.next
        last_node.next = new_node

    def display(self):
        current_node = self.head
        while current_node:
            print(current_node.data, end=" -> ")
            current_node = current_node.next
        print("None")


if __name__ == "__main__":
    node1 = Node(10)
    node2 = Node(15)
    node3 = Node(7)

    l1 = LinkedList()
    l1.append(2)
    l1.append(10)
    l1.append(15)
    l1.append(7)
    l1.display()