#include <linux/module.h>
#include <linux/init.h>
#include <linux/vmalloc.h>
#include <linux/kernel.h>

static void *vm_addr;

int simple_prt_init(void) {
    vm_addr = vmalloc(4096);
    printk("0x%016lx", (unsigned long)vm_addr);
    return 0;
}

void simple_prt_exit(void) {
    vfree(vm_addr);
}

module_init(simple_prt_init);
module_exit(simple_prt_exit);
MODULE_LICENSE("GPL");