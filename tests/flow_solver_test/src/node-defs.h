#ifndef __NODE__
#define __NODE__

#include <stdio.h>
#include <stdint.h>

#include "utils.h"
#include "engine.h"

#include <stdbool.h> // MODIFIED


/**
 * Data structure containing the node information
 */


// Search node for DIJKSTRA
typedef struct tree_node_struct {
	game_state_t state;              // Current game state
	double cost_to_node;             // Cost to node
	struct tree_node_struct* parent; // Parent of this node (may be NULL)
} tree_node_t;


#endif
