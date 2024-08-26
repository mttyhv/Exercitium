#include <stdio.h>

int main() {
	int x = 10;
	int *ptr = &x;  // ptr aponta para o endereço de memória de x
    
	printf("Valor de x: %d\n", x);
	printf("Endereço de memória de x: %p\n", &x);
	printf("Valor apontado por ptr: %d\n", *ptr);
	printf("Endereço de memória apontado por ptr: %p\n", ptr);

	return 0;
}