#include <stdio.h>
#include "refcnt.h"
#include <pthread.h>
#include <stdlib.h>

const int DATA_SIZE = 512;

/* === */
struct xxx {
	refcnt rc;
	int *data;
};

/* mb need to use int to determine whether the "rc" in x have inited */
static inline void xxx_get(struct xxx *x) {
	refcnt_get(&x->rc);
	return;
}

static inline void xxx_put(struct xxx *x) {
	if (refcnt_put(&x->rc)) {
		printf("free data\n");
		free(x->data);
	}
}

/* === */

void *th_func(void *arg) {
	struct xxx *x = (struct xxx*)arg;
	for (int i = 0; i < DATA_SIZE; i++) {
		printf("thread_%ld[%d] = %d\n", pthread_self(), i, x->data[i]);
	}
	
	xxx_put(x);
	return NULL;
}

int main(void) {
	struct xxx x;
	refcnt_init(&x.rc);
	x.data =  calloc(sizeof(int), DATA_SIZE);
	for (int i = 0; i < DATA_SIZE; i++) {
		x.data[i] = i;
	}

	pthread_t th1, th2;

	xxx_get(&x);
	pthread_create(&th1, NULL, th_func, (void*)&x);

	xxx_get(&x);
	pthread_create(&th2, NULL, th_func, (void*)&x);

	xxx_put(&x);
	pthread_join(th1, NULL);
	pthread_join(th2, NULL);
}
