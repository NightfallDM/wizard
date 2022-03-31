#include <linux/module.h>
#include <linux/init.h>
#include <linux/vmalloc.h>
#include <linux/kernel.h>

static void *vm_addr;

int simple_prt_init(void) {
    vm_addr = vmalloc(4096);
    printk("0x%016lx", (unsigned long)vm_addr);
    printk("y = 0x%016lx\n",(unsigned long)((unsigned long)vm_addr - 0xffffffff80000000UL));
    return 0;
}

void simple_prt_exit(void) {
    vfree(vm_addr);
}

module_init(simple_prt_init);
module_exit(simple_prt_exit);
MODULE_LICENSE("GPL");
