#include <linux/init.h>
#include <linux/module.h>
#include <linux/kernel.h>
#include <linux/slab.h>
#include <linux/bug.h>

char __morestack[1024];
char _GLOBAL_OFFSET_TABLE_;

void abort(void)
{
    BUG();
}

extern void rust_main(void);

static int hello_init(void)
{
    printk(KERN_INFO "hello: init\n");
    rust_main();
    return 0;
}

static void hello_exit(void)
{
    printk(KERN_INFO "hello: exit\n");
}

module_init(hello_init);
module_exit(hello_exit);

MODULE_LICENSE("MIT");
