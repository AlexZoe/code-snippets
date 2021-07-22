#include <iostream>

template <typename T>
class Queue {
 private: 
  struct Node {
    Node* next;
    T val;
  };

 public:
  Queue();
  ~Queue();
  bool isEmpty();
  void push(T val);
  void pop();
  int size();
  T front();
  T back();
  
 private:
  Node* first;
  Node* last;
  int num_items;
};

template <typename T>
Queue<T>::Queue() {
  first = nullptr;
  last = nullptr;
  num_items = 0;
}

template <typename T>
Queue<T>::~Queue() {
  while (!isEmpty()) pop();
}

template <typename T>
void Queue<T>::push(T val) {
  Node* new_node = new Node;
  new_node->next = nullptr;
  new_node->val = val;
  if (!isEmpty()) {
    last->next = new_node;
  } else {
    first = new_node;
  }
  last = new_node;
  ++num_items;
}

template <typename T>
void Queue<T>::pop() {
  if (!isEmpty()) {
    Node* target = first;
    first = first->next;
    delete target;
    --num_items;
  }
}

template <typename T>
bool Queue<T>::isEmpty() {
  return (num_items == 0);
}

template <typename T>
T Queue<T>::front() {
  if (!isEmpty()) {
    return first->val;
  } else {
    return T();
  }
}

template <typename T>
T Queue<T>::back() {
  if (!isEmpty()) {
    return last->val;
  } else {
    return T();
  }
}

int main(int argc, char** argv) {
  Queue<int> queue;
  for (int i = 0; i < 5; ++i) {
    queue.push(i);
  }

  while (!queue.isEmpty()) {
    std::cout << "front is: " << queue.front() << std::endl;
    std::cout << "back is: " << queue.back() << std::endl;
    queue.pop();
  }

  std::cout << "front on empty: " << queue.front() << std::endl;
  std::cout << "back on empty: " << queue.back() << std::endl;

  return 0;
}
