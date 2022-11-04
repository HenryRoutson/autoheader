
#ifndef LINKED_LIST

	#define LINKED_LIST

	#include <stddef.h>

	typedef struct node node_t;
	struct node {
		void *data;
		node_t *next;
	};

	typedef struct {
		node_t *head;
		node_t *tail;
		size_t len;
	} list_t;

#endif