#include <iostream>
template<class T>
class Node {
    public:
        T val;
        Node<T> *next;
        explicit Node(T val): val(val), next(nullptr) {}
};

template<class T>
class LinkedList {
    unsigned int cnt;

    public:
        Node<T> *first;
        Node<T> *last;
        LinkedList(): first(nullptr), last(nullptr), cnt(0) {}

        void insert_head(T val) {
            Node<T> *old_first = first;
            Node<T> *node = new Node<T>(val);
            node->next = old_first;
            first = node;
            cnt++;
        }

        void insert_tail(T val) {
            Node<T> *old_tail = last;
            Node<T> *node = new Node<T>(val);
            old_tail->next = node;
            last = node;
            cnt++;
        }

        void traverse() {
            Node<T> *node = first;
            while(node) {
                std::cout << node->val << std::endl;
                node = node->next;
            }
        }

        void drop(Node<T> *node) {
            if (!node) {
                return;
            }
            drop(node->next);
            free(node);
        }

        void reverse() {
            last = reverse(first);
            last->next = nullptr;
        }

        ~LinkedList() {
            drop(first);
        }

        private:
        Node<T> *reverse(Node<T> *node) {
            if (!node) {
                return nullptr;
            }

            Node<T> *curr = node;
            Node<T> *after = reverse(node->next);
            if (!after) {
                first = curr;
            }

            if (after) {
                after->next = curr;
            }
            return curr;
        }

};

int main(void) {
    LinkedList<int> list;
    list.insert_head(100);
    list.insert_head(200);
    list.traverse();
    list.reverse();
    list.traverse();
}