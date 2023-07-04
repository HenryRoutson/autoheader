// This file was automatically created,
// any defitions, including typedefs, structs, extern or #define
// have been moved to a -defs.h file of the same name

#include "engine-defs.h"

void game_print(const game_info_t* info,
                const game_state_t* state);
int game_can_move(const game_info_t* info,
                  const game_state_t* state,
                  int color, int dir);
void game_make_move(const game_info_t* info,
                      game_state_t* state, 
                      int color, int dir);
int game_next_move_color(const game_info_t* info,
                         const game_state_t* state);
int game_num_free_coords(const game_info_t* info,
                         const game_state_t* state,
                         int x, int y);
int game_num_free_pos(const game_info_t* info,
                      const game_state_t* state,
                      pos_t pos);
int game_read(const char* filename,
              game_info_t* info,
              game_state_t* state);
void game_save_svg(const char* filename,
                   const game_info_t* info,
                   const game_state_t* state);
