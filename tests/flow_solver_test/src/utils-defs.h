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

#endif
