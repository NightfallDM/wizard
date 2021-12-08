#ifndef HALFWARE_BTREE_H
#define HALFWARE_BTREE_H

struct bplus_node {
    unsigned int height;
    unsigned int len;
    struct bplus_node **childs;
}__attribute__((align(sizeof(long))));

struct bplus_root {
    struct bplus_node *bplus_node;
};

#endif /* end of HALFWARE_BTREE_H */