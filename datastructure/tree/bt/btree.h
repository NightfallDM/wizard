#ifndef HALFWARE_BTREE_H
#define HALFWARE_BTREE_H

#include "btree_types.h"
/* try to impl b+ tree in c 
   maybe will use futher more */

#define HALFWARE_BPLUS_B   6
#define HALFWARE_BPLUS_CAPACITY  (2 * HALFWARE_BPLUS_B + 1)

// #define BPLUS_ROOT   ({NULL,})

/* call this func after recursive insert bplus_node 
   the param need be the parent node of the insert node
   this func will reblance the bplus tree buttom up*/
extern struct bplue_node *reblance_bt_up(struct bplus_node*);

#endif /* end of HALFWARE_BTREE_H */