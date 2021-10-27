#include "mem.h"
#include <stdio.h>

int main(void){
	int *array = malloc(sizeof(int) * 1024);
	int idx = 0;
	for (idx=0;idx<1024;idx++){
		array[idx] = idx;
	}
	for (idx=0;idx<1024;idx++){
		printf("array[%d] = %d\n", idx, array[idx]);
	}
}
