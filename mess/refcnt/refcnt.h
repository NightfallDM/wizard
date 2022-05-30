#ifndef REFCNT_H
#define REFCNT_H

#include <stdatomic.h>
#include <stdint.h>

typedef _Atomic uint64_t refcnt;

static inline void refcnt_init(refcnt *rc) {
	return atomic_init(rc, 1);
}

static inline void refcnt_get(refcnt *rc) {
	atomic_fetch_add(rc, 1);
	return;
}

/*
 * use the RET value to determine whether the refcnt goes 0;
 * ret 1: refcnt goes to zero, obj need free;
 * otherwise 0
 * */
static int refcnt_put(refcnt *rc) {
	if (atomic_fetch_sub(rc, 1) == 1) {
		//free(rc);
		return 1;
	}
	return 0;
}


#endif /* REFCNT_H */
