#ifndef __QUEUES__
#define __QUEUES__

#include "options.h"
#include "node.h"

// Data structure for heap based priority queue
typedef struct heapq_struct {
	tree_node_t** start; // Array of node pointers
	size_t capacity;     // Maximum allowable queue size
	size_t count;        // Number enqueued
	size_t total_count;  // Total Number enqueued
} heapq_t;

// First in, first-out queue implemented as an array of pointers.
typedef struct queue_struct {
	tree_node_t** start; // Array of node pointers
	size_t capacity;     // Maximum number of things to enqueue ever
	size_t count;        // Total enqueued (next one will go into start[count])
	size_t next;         // Next index to dequeue
} queue_t;

#endif