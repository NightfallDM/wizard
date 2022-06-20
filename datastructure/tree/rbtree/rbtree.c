#include <stdlib.h>
#include "rbtree.h"
#include <assert.h>

struct rbtree {
	rbnode_t* root;
	rbnode_compare cmp;
	rbnode_destructor dest;
	rbnode_show show;
};

struct rbnode {
	// key at there represent key val;
	// and key should at first byte addr;
	// so you should put below form
	//
	// struct my_key_val {
	//		key_type key;
	//		val_type val;
	// };
	void* key;
	rbnode_t* left;
	rbnode_t* right;
	color_t color;
};

rbtree_t* rb_create(rbnode_compare cmp, rbnode_destructor dest, rbnode_show show) {
	rbtree_t* rbt = malloc(sizeof(rbtree_t));
	rbt->root = NULL;
	rbt->cmp = cmp;
	rbt->dest = dest;
	rbt->show = show;
	return rbt;
}


static void _rb_nd_free(rbnode_t* rbn, rbnode_destructor dest) {
	if (!rbn)
		return;
	if (rbn->left) {
		_rb_nd_free(rbn->left, dest);
	}
	if (rbn->right) {
		_rb_nd_free(rbn->right, dest);
	}
	dest(rbn->key);
	free(rbn);
	return;
}

void rb_destroy(rbtree_t* rbt) {
	_rb_nd_free(rbt->root, rbt->dest);	
}

static void _rb_nd_show(rbnode_t* rbn, rbnode_show show) {
	if (!rbn)
		return;
	if (rbn->left)
		_rb_nd_show(rbn->left, show);
	show(rbn->key);
	if (rbn->right)
		_rb_nd_show(rbn->right, show);
}

void rb_show(rbtree_t* rbt) {
	_rb_nd_show(rbt->root, rbt->show);
}

static inline void* rb_node_kv(rbnode_t* rbn) {
	return rbn->key;
}

static inline rbnode_t* rb_nd_new(void* key) {
	rbnode_t* rbn = malloc(sizeof(rbnode_t));
	rbn->key = key;
	rbn->left = NULL;
	rbn->right = NULL;
	rbn->color = RED;
	return rbn;
}

// direction:
// 0 => left
// 1 => right
static inline rbnode_t* __rb_rotate_lr(rbnode_t* rbn, int direction) {
	rbnode_t* mdf_nd;
	color_t col_tmp;
	if (!direction) {
		mdf_nd = rbn->right;
		rbn->right = mdf_nd->right;
		mdf_nd->left = rbn;
		col_tmp = rbn->color;
		rbn->color = mdf_nd->color;
		mdf_nd->color = col_tmp;
	} else {
		mdf_nd = rbn->left;
		rbn->left = mdf_nd->right;
		mdf_nd->right = rbn;
		col_tmp = mdf_nd->color;
		mdf_nd->color = rbn->color;
		rbn->color = col_tmp;
	}
	return mdf_nd;
}

static inline rbnode_t* _rb_rotate_left(rbnode_t* rbn) {
	return __rb_rotate_lr(rbn, 0);
}

static inline rbnode_t* _rb_rotate_right(rbnode_t* rbn) {
	return __rb_rotate_lr(rbn, 1);
}

static inline rbnode_t* _rb_blance(rbnode_t* rbn) {
	assert(rbn);
	
	if (rbn->right) {
		// just the last insert node'parent can have right red node
		if (rbn->right->color) {
			if (!rbn->left)
				return _rb_rotate_left(rbn);
			else {
				// at this section, left child must be red;
				rbn->color = RED;
				rbn->left->color = BLACK;
				rbn->right->color = BLACK;
				return rbn;
			}
		}
	}

	if (rbn->left) {
		if (rbn->left->color) {
			if (rbn->left->left) {
				if (rbn->left->left->color) {
					rbnode_t* tmp =  _rb_rotate_right(rbn);
					tmp->color = RED;
					tmp->left->color = BLACK;
					tmp->right->color = BLACK;
					return tmp;
				}
			}
		}
	}
}

static rbnode_t* _rb_insert(rbnode_t *rbn, void* key, rbnode_compare cmp, int* jump_blance, insert_state_t *insert_state) {
	if (!rbn) {
		return rb_nd_new(key);
	}
	
	rbnode_t* rbn_ret;
	int result = cmp(rbn->key, key);
	if (result == -1)
		rbn->right = _rb_insert(rbn->right, key, cmp, jump_blance, insert_state);
	else if (result == 1)
		rbn->left = _rb_insert(rbn->left, key, cmp, jump_blance, insert_state);
	else {
		*insert_state = INSERT_KEY_EXIST;
		rbn->key = key;
		*jump_blance = 1;
		return rbn;
	}
	if (!jump_blance)
		return _rb_blance(rbn);
	return rbn;
}

insert_state_t rb_insert(rbtree_t* rbt, void* key, unsigned long size) {
	int jump_blance = 0;
	insert_state_t ret_state = INSERT_KEY_NOT_EXIST;
	rbt->root = _rb_insert(rbt->root, key, rbt->cmp, &jump_blance, &ret_state);
	return ret_state;
}

rbnode_t* rb_get(rbtree_t* rbt, void* key) {}

delete_state_t rb_delete(rbtree_t* rbt, void* key) {}

