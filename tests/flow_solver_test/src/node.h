// This file was automatically created,
// any defitions, including typedefs, structs, extern or #define
// have been moved to a -defs.h file of the same name

#include "node-defs.h"

int node_compare(const tree_node_t* a,
                 const tree_node_t* b);
tree_node_t* node_create(tree_node_t* parent,
                         const game_info_t* info,
                         const game_state_t* state);
void node_diagnostics(const game_info_t* info,
		      const tree_node_t* node);
void animate_solution(const game_info_t* info,
                           const tree_node_t* node);
