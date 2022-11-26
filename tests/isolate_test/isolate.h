#ifndef __ENGINE__
#define __ENGINE__

#include <stdio.h>
#include <stdint.h>
#include <locale.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>
#include <math.h>
#include <time.h>

///////////////////////////////////////////////////////////
// Positions are 8-bit integers with 4 bits each for y, x.
enum {

	// Number to represent "not found"
	INVALID_POS = 0xff,
  
	// Maximum # of colors in a puzzle
	MAX_COLORS = 16,
  
	// Maximum valid size of a puzzle
	MAX_SIZE = 15,
  
	// Maximum # cells in a valid puzzle -- since we just use bit
	// shifting to do x/y, need to allocate space for 1 unused column.
	MAX_CELLS = (MAX_SIZE+1)*MAX_SIZE-1,
  
	// One million(ish) bytes
	MEGABYTE = 1024*1024,
  
};

////////////////////////////////////////////////////////
// Represent the contents of a cell on the game board
typedef uint8_t cell_t;

///////////////////////////////////////////////////////
// Represent a position within the game board
typedef uint8_t pos_t;


///////////////////////////////////////////////////////////////////////
// Static information about a puzzle layout -- anything that does not
// change as the puzzle is solved is stored here.
typedef struct game_info_struct {

	// Index in color_dict table of codes
	int    color_ids[MAX_COLORS];

	// Color order
	int    color_order[MAX_COLORS];

	// Initial and goal positions
	pos_t  init_pos[MAX_COLORS];
	pos_t  goal_pos[MAX_COLORS];

	// Length/width of game board
	size_t size;

	// Number of colors present
	size_t num_colors;

	// Color table for looking up color ID
	uint8_t color_tbl[128];

  
} game_info_t;

////////////////////////////////////////////////////////////////////////
// Incremental game state structure for solving -- this is what gets
// written as the search progresses, one state per search node
typedef struct game_state_struct {

	// State of each cell in the world; a little wasteful to duplicate,
	// since only one changes on each move, but necessary for DIJKSSTRA
	// (would not be needed for depth-first search).
	cell_t   cells[MAX_CELLS];

	// Head position
	pos_t    pos[MAX_COLORS];

	// How many free cells?
	uint8_t  num_free;

	// Which was the last color / endpoint
	uint8_t  last_color;

	// Bitflag indicating whether each color has been completed or not
	// (cur_pos is adjacent to goal_pos).
	uint16_t completed;
  
} game_state_t;


///////////////////////////////////////////////////////
// Various cell types, all but freespace have a color
enum {
	TYPE_FREE = 0, // Free space
	TYPE_PATH = 1, // Path between init & goal
	TYPE_INIT = 2, // Starting point
	TYPE_GOAL = 3  // Goal position
};

/////////////////////////////////////////////////////////
// Enumerate cardinal directions so we can loop over them
// RIGHT is increasing x, DOWN is increasing y.
enum {
	DIR_LEFT  = 0,
	DIR_RIGHT = 1,
	DIR_UP    = 2,
	DIR_DOWN  = 3
};



//////////////////////////////////////////////////////////////////////
// Print out game board

void game_print(const game_info_t* info, const game_state_t* state);

//////////////////////////////////////////////////////////////////////
// Consider whether the given move is valid.

int game_can_move(const game_info_t* info, const game_state_t* state, int color, int dir);


//////////////////////////////////////////////////////////////////////
// Update the game state to make the given move.

void game_make_move(const game_info_t* info, game_state_t* state, int color, int dir);

//////////////////////////////////////////////////////////////////////
// Pick the next color to move deterministically

int game_next_move_color(const game_info_t* info, const game_state_t* state);


//////////////////////////////////////////////////////////////////////
// Return the number of free spaces around an x, y position

int game_num_free_coords(const game_info_t* info, const game_state_t* state, int x, int y);

//////////////////////////////////////////////////////////////////////
// NEW: Return the number of free spaces or starting locations around 
// an x, y position
int game_num_source_coords(const game_info_t* info, const game_state_t* state, int x, int y);


//////////////////////////////////////////////////////////////////////
// Return the number of free spaces around an 8-bit position

int game_num_free_pos(const game_info_t* info, const game_state_t* state, pos_t pos);


//////////////////////////////////////////////////////////////////////
// Return free if in bounds and unoccupied

int game_is_free(const game_info_t* info, const game_state_t* state, int x, int y);



//////////////////////////////////////////////////////////////////////
// Read game board from text file

int game_read(const char* filename, game_info_t* info, game_state_t* state);

//////////////////////////////////////////////////////////////////////
// Print out game board as SVG

void game_save_svg(const char* filename, const game_info_t* info, const game_state_t* state);

#endif





#ifndef __NODE__
#define __NODE__

#include <stdio.h>
#include <stdint.h>

#include "utils.h"
#include "engine.h"


/**
 * Data structure containing the node information
 */


// Search node for DIJKSTRA
typedef struct tree_node_struct {
	game_state_t state;              // Current game state
	double cost_to_node;             // Cost to node
	struct tree_node_struct* parent; // Parent of this node (may be NULL)
	unsigned int children;           // Used to count number of direct children active in search
} tree_node_t;

// Compare total cost for nodes, used by heap functions below.
int node_compare(const tree_node_t* a, const tree_node_t* b);

// Create Node and update cost
tree_node_t* node_create(tree_node_t* parent, const game_info_t* info, const game_state_t* state);



//////////////////////////////////////////////////////////////////////
// Perform diagnostics on the given node

void node_diagnostics(const game_info_t* info, const tree_node_t* node);


//////////////////////////////////////////////////////////////////////
// Animate the solution by printing out boards in reverse order,
// following parent pointers back from solution to root.

void animate_solution(const game_info_t* info, const tree_node_t* node);



#endif




#ifndef __OPTIONS__
#define __OPTIONS__

#include <stdio.h>
#include <stdint.h>

// Options for this program
typedef struct options_struct {

	int    display_quiet;
	int    display_diagnostics;
	int    display_animate;
	int    display_color;
	int    display_fast;
	int    display_save_svg;  

	int    node_check_deadends;
  
	int    order_most_constrained;
	int    order_random;

	size_t search_max_nodes;
	double search_max_mb;
  
} options_t;

//Parse Command-line options
size_t parse_options(int argc, char** argv, const char** input_files);

// Global options struct gets setup during main
extern options_t g_options;

#endif





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



//////////////////////////////////////////////////////////////////////
// Create a binary heap to store the given # of nodes

heapq_t heapq_create(size_t max_nodes);

//////////////////////////////////////////////////////////////////////
// Is heap queue count

size_t heapq_count(const heapq_t* q);

//////////////////////////////////////////////////////////////////////
// Is heap queue empty?

int heapq_empty(const heapq_t* q);

//////////////////////////////////////////////////////////////////////
// Peek at the next item to be removed

const tree_node_t* heapq_peek(const heapq_t* q);

//////////////////////////////////////////////////////////////////////
// Enqueue a node onto the heap

void heapq_enqueue(heapq_t* q, tree_node_t* node);

//////////////////////////////////////////////////////////////////////
// Pop a node off the heap

tree_node_t* heapq_deque(heapq_t* q);

//////////////////////////////////////////////////////////////////////
// Free memory allocated for heap

void heapq_destroy(heapq_t* q);



//////////////////////////////////////////////////////////////////////
// QUEUE via flat array

queue_t queue_create(size_t max_nodes);

//////////////////////////////////////////////////////////////////////
// Check if empty

int queue_empty(const queue_t* q);

//////////////////////////////////////////////////////////////////////
// Check count

size_t queue_count(const queue_t* q);

//////////////////////////////////////////////////////////////////////
// Push node into QUEUE

void queue_enqueue(queue_t* q, tree_node_t* n);

//////////////////////////////////////////////////////////////////////
// Peek at current QUEUE node

const tree_node_t* queue_peek(const queue_t* q);

//////////////////////////////////////////////////////////////////////
// Dequeue node from QUEUE

tree_node_t* queue_deque(queue_t* q);

//////////////////////////////////////////////////////////////////////
// De-allocate storage for QUEUE

void queue_destroy(queue_t* q);
#endif


#ifndef __SEARCH__
#define __SEARCH__


#include "node.h"
#include "engine.h"

//////////////////////////////////////////////////////////////////////
// Peforms Dijkstra  search

int game_dijkstra_search(const game_info_t* info, const game_state_t* init_state, double* elapsed_out, size_t* nodes_out, game_state_t* final_state);

#endif



#ifndef __UTILS__
#define __UTILS__

#include <stdio.h>
#include <stdint.h>
#include <locale.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>
#include <math.h>
#include <time.h>

#include "engine.h"

////////////////////////////////////////////////////////
// Search termination results
enum {
	SEARCH_SUCCESS = 0,
	SEARCH_UNREACHABLE = 1,
	SEARCH_FULL = 2,
	SEARCH_IN_PROGRESS = 3,
};


//////////////////////////////////////////////////////
// Match color characters to ANSI color codes
typedef struct color_lookup_struct {
	char input_char;   // Color character
	char display_char; // Punctuation a la nethack
	const char* ansi_code;    // ANSI color code
	const char* fg_rgb;
	const char* bg_rgb;
} color_lookup_t;


////////////////////////////////////////////
// Used for auto-sorting colors
typedef struct color_features_struct {
	int index;
	int user_index;
	int wall_dist[2];
	int min_dist;
} color_features_t;

// Was gonna try some unicode magic but meh
extern const char* BLOCK_CHAR;

// For visualizing cardinal directions ordered by the enum above.
extern const char DIR_CHARS[4];

///////////////////////////////////////////////////////
// x, y, pos coordinates for each direction
extern const int DIR_DELTA[4][3];

// Look-up table mapping characters in puzzle definitions to 
// output char, ANSI color, foreground/background RGB
extern const color_lookup_t color_dict[MAX_COLORS];

// For succinct printing of search results
extern const char SEARCH_RESULT_CHARS[4];

// For verbose printing of search results
extern const char* SEARCH_RESULT_STRINGS[4];


//////////////////////////////////////////////////////////////////////
// Peform lookup in color_dict above

int get_color_id(char c);

//////////////////////////////////////////////////////////////////////
// Detect whether terminal supports color & cursor commands

int terminal_has_color();

//////////////////////////////////////////////////////////////////////
// Emit color string for index into color_dict table above

const char* color_char(const char* ansi_code, char color_out, char mono_out);

//////////////////////////////////////////////////////////////////////
// Clear screen and set cursor pos to 0,0

const char* unprint_board(const game_info_t* info);

//////////////////////////////////////////////////////////////////////
// Are the coords on the map?

int coords_valid(const game_info_t* info, int x, int y);

//////////////////////////////////////////////////////////////////////
// Compose an offset as a position and return whether valid or not

pos_t offset_pos(const game_info_t* info, int x, int y, int dir);

//////////////////////////////////////////////////////////////////////
// Compose an offset as a position and return whether valid or not

pos_t pos_offset_pos(const game_info_t* info, pos_t pos, int dir);

//////////////////////////////////////////////////////////////////////
// Get the distance from the wall for x, y coords

int get_wall_dist(const game_info_t* info, int x, int y);

//////////////////////////////////////////////////////////////////////
// Get the distance from the wall for 8-bit position

int pos_get_wall_dist(const game_info_t* info, pos_t pos);

//////////////////////////////////////////////////////////////////////
// Create a cell from a 2-bit type, a 4-bit color, and a 2-bit
// direction.

cell_t cell_create(uint8_t type, uint8_t color, uint8_t dir);

//////////////////////////////////////////////////////////////////////
// Get the type from a cell value

uint8_t cell_get_type(cell_t c);

//////////////////////////////////////////////////////////////////////
// Get the direction from a cell value

uint8_t cell_get_direction(cell_t c);

//////////////////////////////////////////////////////////////////////
// Get the color from a cell value

uint8_t cell_get_color(cell_t c);

//////////////////////////////////////////////////////////////////////
// For displaying a color nicely

const char* color_name_str(const game_info_t* info, int color);

//////////////////////////////////////////////////////////////////////
// For displaying a cell nicely

const char* color_cell_str(const game_info_t* info,cell_t cell);

//////////////////////////////////////////////////////////////////////
// Compare 2 ints

int cmp(int a, int b);

//////////////////////////////////////////////////////////////////////
// Return the current time as a double. Don't actually care what zero
// is cause we will just offset.

double now();

//////////////////////////////////////////////////////////////////////
// Create a delay

void delay_seconds(double s) ;

//////////////////////////////////////////////////////////////////////
// Create a 8-bit position from 2 4-bit x,y coordinates

pos_t pos_from_coords(pos_t x, pos_t y);

//////////////////////////////////////////////////////////////////////
// Split 8-bit position into 4-bit x & y coords

void pos_get_coords(pos_t p, int* x, int* y);

#endif
