// This file was automatically created,
// any defitions, including typedefs, structs, extern or #define
// have been moved to a -defs.h file of the same name

#include "search-defs.h"

int game_dijkstra_search(const game_info_t* info,
                const game_state_t* init_state,
                double* elapsed_out,
                size_t* nodes_out,
                game_state_t* final_state);
