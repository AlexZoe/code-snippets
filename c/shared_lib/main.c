#include <stdio.h>
#include "myshared.h"

int main(void)
{
	printf("Call shared lib ...\n");
	lib_test();
	return 0;
}
