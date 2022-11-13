#include "list.h"

#include <stdio.h>
#include <stdlib.h>


#define LIST_LEN 5

int main() {

	list_t *list = create_empty_list();
	
	// add some numbers to a linked list
	for (int i = 0; i < LIST_LEN; i++) {
		int *num =  malloc(sizeof(*num));
		*num = i;
		add_to_list(list, num);
	}

	// print off those numbers
	node_t *cur = list -> head;
	while(cur) {
		printf("%i", * (int *) cur->data);
		cur = cur -> next;
	}

	free_list(list, free);
}