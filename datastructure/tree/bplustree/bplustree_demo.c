#include <stddef.h>
#include "bplustree.h"
#include "bplustree_demo.h"

int compare_keys(struct mytype_key *key1, struct mytype_key *key2) {
	return key1->key1 + key1->key2 - key2->key1 - key2->key2;
}

struct mytype_leaf *my_search_leaf(struct bplus_root *root, struct mytype_key *s_key, unsigned int *offset) {
	if (!root) {
		return NULL;
	}

	struct bplus_node *node = root->bplus_node;
	while (node->height > 1) {
		struct mytype_internal *itnd = container_of(node, struct mytype_internal, node);
		int idx = 0;
		for (; idx < node->len; idx++) {
			if (compare_keys(&itnd->keys[idx], s_key) >= 0) {
				node = node->childs[idx];
				break;
			}
		}
	}
	int oidx = 0;
	for (; oidx < node->len; oidx++) {
		struct mytype_leaf *lfnd = container_of(node, struct mytype_leaf, node);
		if (!compare_keys(&lfnd->keys[oidx], s_key)) {
			*offset = oidx;
			return lfnd;
		}
	}
	return NULL;
}

struct mytype_value *my_search(struct bplus_root *root, struct mytype_key *s_key) {
	if (!root) {
		return NULL;
	}

	struct bplus_node *node = root->bplus_node;
	while (node->height > 1) {
		struct mytype_internal *itnd = container_of(node, struct mytype_internal, node);
		int idx = 0;
		for (; idx < node->len; idx++) {
			if (compare_keys(&itnd->keys[idx], s_key) >= 0) {
				node = node->childs[idx];
				break;
			}
		}
	}
	int oidx = 0;
	for (; oidx < node->len; oidx++) {
		struct mytype_leaf *lfnd = container_of(node, struct mytype_leaf, node);
		if (!compare_keys(&lfnd->keys[oidx], s_key)) {
			return lfnd->values;
		}
	}
	return NULL;
}

int my_insert(struct bplus_root *root, struct mytype_key *ins_key, struct mytype_value *ins_value) {
	
}

int main() {
	
}
