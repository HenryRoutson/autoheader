// This file was automatically created,
// any defitions, including typedefs, structs, extern or #define
// have been moved to a -defs.h file of the same name

#include "utils-defs.h"

int get_color_id(char c);
int terminal_has_color();
const char* color_char(const char* ansi_code, char color_out, char mono_out);
const char* unprint_board(const game_info_t* info);
int coords_valid(const game_info_t* info,
                 int x, int y);
pos_t offset_pos(const game_info_t* info,
                 int x, int y, int dir);
pos_t pos_offset_pos(const game_info_t* info,
                     pos_t pos, int dir);
int get_wall_dist(const game_info_t* info,
                  int x, int y);
int pos_get_wall_dist(const game_info_t* info,
                      pos_t pos);
cell_t cell_create(uint8_t type, uint8_t color, uint8_t dir);
uint8_t cell_get_type(cell_t c);
uint8_t cell_get_direction(cell_t c);
uint8_t cell_get_color(cell_t c);
const char* color_name_str(const game_info_t* info,
                           int color);
const char* color_cell_str(const game_info_t* info,
                           cell_t cell);
int cmp(int a, int b);
double now();
void delay_seconds(double s);
pos_t pos_from_coords(pos_t x, pos_t y);
void pos_get_coords(pos_t p, int* x, int* y);
