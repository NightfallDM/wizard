#ifndef BPLUSTREE_DEMO_H
#define BPLUSTREE_DEMO_H
#include "bplustree.h"

struct mytype_key {
	int key1;
	int key2;	
};

struct mytype_value {
	int val1;
	int val2;
};

struct mytype_internal {
	struct bplus_node node;
	struct mytype_key *keys;
};

struct mytype_leaf {
	struct bplus_node node;
	struct mytype_key *keys;
	struct mytype_value *values;
};

/* func start */
int compare_keys(struct mytype_key*, struct mytype_key*);
struct mytype_value *my_search(struct bplus_root*, struct mytype_key*);
struct mytype_leaf *my_search_leaf(struct bplus_root *root, struct mytype_key *s_key, unsigned int *offset);
int my_insert(struct bplus_root *root, struct mytype_key *ins_key, struct mytype_value *ins_value);
/* func end */

/* macro start */
#define container_of(ptr, type, field)	({\
	void* __mptr = (void *)((unsigned long)(ptr) - offsetof(type, field));\
	((type *) (__mptr)); })

/* macro end */
#endif /* end of BPLUSTREE_DEMO_H */

