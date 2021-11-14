#include <cstddef>
#include <iostream>

const int sq_capacity = 100;
template<class Tp>
class Squeue {
    Tp sq_array[sq_capacity];
    unsigned int cursor;
    unsigned int len;
    public:
        Squeue(): cursor(0), len(0){}
        unsigned int length() {
            return len;
        }

        // -1 represent err: sq full;
        int in(Tp val) {
            if (len < sq_capacity){
                sq_array[cursor] = val;
                len++;
                if (++cursor == 100){
                    cursor = 0;
                }
                return 0;
            }
                return -1;
        }

        Tp out() {
            if (len == 0){
                return NULL;
            }
            len--;
            if (--cursor == -1) {
                cursor = sq_capacity - 1;
                return sq_array[0];
            }
            return sq_array[cursor + 1];
        }

        bool empty() {
            return len == 0;
        }
};

template<class Tp>
class BinaryTreeNode {
    public:
        BinaryTreeNode<Tp> *left;
        BinaryTreeNode<Tp> *right;
        Tp value;
        explicit BinaryTreeNode(Tp val) {
            left = NULL;
            right = NULL;
            value = val;
        }
};

template<class Tp>
class BinaryTree {
    public:
        BinaryTreeNode<Tp> *root;
        BinaryTree() {
            root = NULL;
        }

        /* temp: use print value */
        void wideOrder() {
            Squeue<BinaryTreeNode<Tp> *> sq;
            BinaryTreeNode<Tp> *p = root;
            if (!root) {
                return;
            }else {
                std::cout << root->value << std::endl;
                sq.in(root);
            }

            while (!sq.empty()) {
                p = sq.out();
                if (p->left) {
                    std::cout << p->left->value << std::endl;
                    sq.in(p->left);
                }
                if (p->right) {
                    std::cout << p->right->value << std::endl;
                    sq.in(p->right);
                }
            }
        }
};


int main(int argc, char *argv[]) {

    /* build a binary tree */
    BinaryTree<int> bt{};
    BinaryTreeNode<int> node1(6);
    bt.root = &node1;
    BinaryTreeNode<int> node2(3), node3(9);
    node1.left = &node2;
    node1.right = &node3;
    BinaryTreeNode<int> node4(2), node5(4), node6(7), node7(10);
    node2.left = &node4;
    node2.right = &node5;
    node3.left = &node6;
    node3.right = &node7;
    /* end build */

    bt.wideOrder();
}