#ifndef HALFWARE_BPLUSTREE_H
#define HALFWARE_BPLUSTREE_H

#define HALFWARE_B	6

const unsigned int B = HALFWARE_B;
const unsigned int MIN_CAPACITY = HALFWARE_B;
const unsigned int MAX_CAPACITY = HALFWARE_B;

struct bplus_node {
	unsigned int height;
	struct bplus_node **childs;
	unsigned int len;
};

struct bplus_root {
	struct bplus_node *bplus_node;
};


#endif /* end of HALFWARE_BPLUSTREE_H*/
