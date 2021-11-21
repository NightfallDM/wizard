#include <cstddef>
#include <iostream>

template<class T>
class BTreeNode {
    public:
        T value;
        BTreeNode<T> *left;
        BTreeNode<T> *right;
        explicit BTreeNode(T val): value(val), left(NULL), right(NULL) {}
};

template<class T>
class BTree {
    public:
        BTreeNode<T> *root;
        BTree(): root(NULL) {}
        BTree(BTreeNode<T>& root_node): root(&root_node) {}
        BTree(BTreeNode<T> *root_node): root(root_node) {}
        int add(BTreeNode<T>& other_node) {
            root->left = &other_node;
        }
};

int main(int argc, char *argv[]) {
    // BTree<int> bt{};
    BTreeNode<int> node1(5);
    BTree<int> bt(node1);
    BTree<int> bt1(&node1);
    // bt.root = &node1;
}