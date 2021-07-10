#include <iostream>

template <typename T>
class Stack {
 private:
  struct Node {
    T val;
    Node* next;
  };

 public:
  Stack();
  ~Stack();
  void push(T val);
  T top();
  void pop();
  int size();

 private:
  int elements;
  Node* root;
};


template <typename T>
Stack<T>::Stack() {
  root = nullptr;
  elements = 0;
}


template <typename T>
Stack<T>::~Stack() {
  while (elements) pop();
}


template <typename T>
void Stack<T>::push(T val) {
  Node* old_root = root;
  root = new Node;
  root->val = val;
  root->next = old_root;
  elements += 1;
}


template <typename T>
T Stack<T>::top() {
  if (elements) {
    return root->val;
  } else {
    T ret;
    return ret;
  }
}


template <typename T>
void Stack<T>::pop() {
  if (elements) {
    Node* old_root = root;
    root = old_root->next;
    delete old_root;
    --elements;
  }
}


template <typename T>
int Stack<T>::size() {
  return elements;
}


int main(int argc, char** argv) {
  Stack<int> my_stack;
  my_stack.push(3);
  my_stack.push(2);
  my_stack.push(5);

  while (my_stack.size()) {
    std::cout << my_stack.top() << std::endl;
    my_stack.pop();
  }

  return 0;
}
