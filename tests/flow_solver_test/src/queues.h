// This file was automatically created,
// any defitions, including typedefs, structs, extern or #define
// have been moved to a -defs.h file of the same name

#include "queues-defs.h"

heapq_t heapq_create(size_t max_nodes);
size_t heapq_count(const heapq_t* q);
int heapq_empty(const heapq_t* q);
const tree_node_t* heapq_peek(const heapq_t* q);
void heapq_enqueue(heapq_t* q, tree_node_t* node);
tree_node_t* heapq_deque(heapq_t* q);
void heapq_destroy(heapq_t* q);
queue_t queue_create(size_t max_nodes);
int queue_empty(const queue_t* q);
size_t queue_count(const queue_t* q);
void queue_enqueue(queue_t* q, tree_node_t* n);
const tree_node_t* queue_peek(const queue_t* q);
tree_node_t* queue_deque(queue_t* q);
void queue_destroy(queue_t* q);
