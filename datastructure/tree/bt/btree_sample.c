#include "btree.h"
#include <stdlib.h>
#include <stdbool.h>


struct my_key {
    int key_one;
    int key_two;
};

struct my_value {
    long value_one;
    long value_two;
};

struct my_internal {
    struct bplus_node node;
    // struct my_key keys[HALFWARE_BPLUS_CAPACITY];
    struct my_key *keys;
};

struct my_leaf {
    struct bplus_node node;
    // struct my_key keys[HALFWARE_BPLUS_CAPACITY];
    // struct my_value values[HALFWARE_BPLUS_CAPACITY];
    struct my_key *keys;
    struct my_value *values;
};

struct my_data {
    struct my_key k;
    struct my_value v;
};

struct bplus_node *_my_insert(struct bplus_node* bpn, struct my_data *data) {
    //TODO
}


int my_insert(struct bplus_root *root, struct my_data *data) {
    if (!root->node) {
        struct my_leaf *leaf_node_p = (struct my_leaf *)malloc(sizeof(struct my_leaf));
        struct my_key *keys = (struct my_key *)malloc(sizeof(struct my_key) * HALFWARE_BPLUS_CAPACITY);
        struct my_value *values = (struct my_value *)malloc(sizeof(struct my_value) * HALFWARE_BPLUS_CAPACITY);
        keys[0] = data->k;
        values[0] = data->v;
        struct bplus_node *bpn = (struct bplus_node *)malloc(sizeof(struct bplus_node));

        /* we do not access 'childs' when the 'height' == 0 */
        bpn->height = 0;
        bpn->len = 1;
        bpn->childs = NULL;
        leaf_node_p->node = *bpn;
        leaf_node_p->keys = keys;
        leaf_node_p->values = values;
        root->bplus_node = bpn;
        return true;
    }else {
        root->bplus_node = _my_insert(root->bplus_node, data);
    }
}