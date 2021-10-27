#include "mem.h"
#include <errno.h>
#include <stdbool.h>
#include <pthread.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

#define MAX_BLOCK	128
struct block {
	void *base;
	unsigned long size;
	bool map;
};

static struct block mmblock[MAX_BLOCK];

struct mmblockInfo {
	unsigned int cnt;
	unsigned int next;
	struct block *mmblock_p;
	bool inited;
};

static struct mmblockInfo mmbinfo = {0, 0, mmblock, false};


static int get_free_blk(){
	int temp_idx = 0;
	int ret = 0;
	if (mmbinfo.cnt == MAX_BLOCK){
		// return 0 is ok
		return 0;
	}
	for (temp_idx=0;temp_idx<MAX_BLOCK;temp_idx++){
		if (mmblock[temp_idx].size == 0){
			ret = temp_idx;
			break;
		}
	}
	return ret;
}

static void init_mmblock(){
	if(mmbinfo.inited){
		return;
	}
	memset(mmblock, 0, sizeof(struct block)*MAX_BLOCK);
	return;
}


void *malloc(size_t size){
	init_mmblock();
	void *base = NULL;
	int idx = 0;
	if (mmbinfo.cnt == MAX_BLOCK || size == 0){
		return NULL;
	}
	size = (size + 0xfff)&0xfffff000;
	if (size > (1UL << 19)){
		base = mmap(NULL, size, PROT_WRITE|PROT_READ, MAP_SHARED|MAP_ANONYMOUS, -1, 0);
		if (base == MAP_FAILED){
			return NULL;
		}
		struct block blk = {base, size, true};
		//TODO thread safe;
		idx = mmbinfo.next;
		mmblock[idx] = blk;
		mmbinfo.next = get_free_blk();
	}else {
		base = sbrk(size);
		if (base == (void*)-1){
			// TODO set errno
			return NULL;
		}else {
			idx = mmbinfo.next;
			struct block blk = {base, size, false};
			mmblock[idx] = blk;
			mmbinfo.next = get_free_blk();
		}
	}
	return base;
}

void free(void *ptr){
	
	//get_block_by_base(ptr);
}
