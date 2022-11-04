#include <assert.h>
#include <stdlib.h>
#include <stdio.h>

#include "list-structs.h"
#include "list-functions.h"

// ---------------------------------------------
//         L I N K E D     L I S T


// public
list_t *create_empty_list() {

	list_t *list = malloc(sizeof(list_t));
	assert(list);

	list->head = NULL;
	list->tail = NULL;
	list->len = 0;

	return list;
}

// public
void add_to_list(list_t *list, void *data) {

	assert(list);

	node_t *newNode = create_node(data);

	if (list->tail) { 
		list -> tail -> next = newNode;

	} else { // list is empty
		list -> head = newNode;
	}

	list -> tail = newNode;	
	list -> len ++;
}

// public
node_t *create_node(void *data) {

	node_t *newNode = malloc(sizeof(node_t));
	assert(newNode);

	newNode->next = NULL;
	newNode->data = data;

	return newNode;
}


// public
void free_node(node_t *node, void (*free_data)(void *)) {
	assert(node);

	if (free_data) { free_data(node -> data); }
	free(node);
}


// public
void free_list(list_t *list, void (*free_data)(void *)) {
	assert(list);
		
	node_t *cur;
	node_t *nxt = list -> head; // store next, cur is freed

	while ((cur = nxt)) {
		nxt = cur -> next;
		free_node(cur, free_data);
	}

	free(list);
}


// public
void merge_lists(list_t *dst, list_t *src) {
	assert(dst);
	assert(src);

	dst -> tail -> next = src -> head;
	dst -> tail = src -> tail;
	free(src);
}