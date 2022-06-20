#ifndef HW_RBTREE_H
#define HW_RBTREE_H

struct rbnode;
struct rbtree;
typedef struct rbnode rbnode_t;
typedef struct rbtree rbtree_t;

// user privade callback compare func
// RETURN:
// -1 => key1 < key2
//  1 => key1 > key2
//  0 => key1 == key2
typedef int (*rbnode_compare)(const void* key1, const void* key2);

// user privade callback destructor func
typedef void (*rbnode_destructor)(void*);

typedef void (*rbnode_show)(const void*);

typedef enum {
	INSERT_KEY_NOT_EXIST,
	INSERT_KEY_EXIST,
}insert_state_t;

typedef enum {
	KEY_DEL_SUCCESS,
	KEY_NOT_FIND,
}delete_state_t;

// 0-black
// 1-red
typedef enum {
	BLACK,
	RED,
}color_t;

rbtree_t* rb_create(rbnode_compare cmp, rbnode_destructor dest, rbnode_show show);
void rb_destroy(rbtree_t* rbt);
rbnode_t* rb_get(rbtree_t* rbt, void* key);
insert_state_t rb_insert(rbtree_t* rbt, void* key, unsigned long size);
delete_state_t rb_delete(rbtree_t* rbt, void* key);
void rb_show(rbtree_t* rbt);

/*
static inline void* rb_node_kv(rbnode_t* rbn) {
	return rbn->key;
}
*/

#endif /* HW_RBTREE_H */
