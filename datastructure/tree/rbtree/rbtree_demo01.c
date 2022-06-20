#include "rbtree.h"
#include <assert.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

struct my_key_val {
	int key;
	int val;
};

int my_nd_compare(const void* nd1, const void* nd2) {
	struct my_key_val* kv1 = (struct my_key_val*)nd1;
	struct my_key_val* kv2 = (struct my_key_val*)nd2;
	if (kv1->key > kv2->key)
		return 1;
	else if (kv1->key < kv2->key)
		return -1;
	else
		return 0;
}

void my_kv_destructor(void* kv) {
	free((struct my_key_val*)kv);
}

void my_show_val(const void* kv) {
	printf("val = %d\n", ((struct my_key_val*)kv)->val);
}

#define ARRAY_SIZE 100
int main(void) {
	srand(time(0));
	struct my_key_val* kvp_array[100];
	for (int i = 0; i < ARRAY_SIZE; i++) {
		struct my_key_val* kv = malloc(sizeof(struct my_key_val));
		kv->key = rand();
		kv->val = kv->key;
		kvp_array[i] = kv;
	}

	rbtree_t* rbt = rb_create(my_nd_compare, my_kv_destructor, my_show_val);
	for (int i = 0; i < ARRAY_SIZE; i++) {
		assert(rb_insert(rbt, (void*)kvp_array[i], i) == INSERT_KEY_NOT_EXIST);
	}
	rb_show(rbt);
	rb_destroy(rbt);
}
