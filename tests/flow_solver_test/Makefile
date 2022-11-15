

# OBJ            object files IE file.o
# EXE            name of executable file
# COMPILER       name to use compiler IE gcc OR clang
# CPPFLAGS       compliler flags
# LDFLAGS        linker flags

OBJ       = src/node.o src/options.o src/utils.o src/extensions.o src/queues.o src/engine.o src/search.o src/flow_solver.o
EXE       = flow
COMPILER  = gcc
CPPFLAGS  = -Wall -Werror -O3 -g 
LDFLAGS   = -lm


$(EXE): cargo_run $(OBJ) 
	$(COMPILER) $(OBJ) $(CPPFLAGS) -o $(EXE) $(LDFLAGS)

cargo_run: 
	cd src; cargo run

clean:
	cd src; rm -f $(EXE) *.o 
	rm $(OBJ:.o=.h)


# RUNWITH = valgrind --track-origins=yes --leak-check=full
RUNWITH = 

run: $(EXE)

	make

	#$(RUNWITH) ./$(EXE)  -A -d puzzles/deadlock_6x6_01.txt
	#$(RUNWITH) ./$(EXE)  -A -d puzzles/unsolvable_cross.txt

	#$(RUNWITH) ./$(EXE) -A -d puzzles/regular_5x5_01.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/regular_6x6_01.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/regular_7x7_01.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/regular_8x8_01.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/regular_9x9_01.txt

	$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_10x10_01.txt
	$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_11x11_01.txt
	$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_12x12_30.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_13x13_26.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_14x14_01.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_14x14_02.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_14x14_19.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_14x14_21.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/jumbo_14x14_30.txt

	$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_8x8_01.txt
	$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_9x9_01.txt
	$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_9x9_30.txt
	$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_10x10_01.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_10x10_30.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_11x11_07.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_11x11_15.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_11x11_20.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_11x11_30.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_12x12_01.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_12x12_02.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_12x12_28.txt
	#$(RUNWITH) ./$(EXE) -A -d puzzles/extreme_12x12_29.txt
	#$(RUNWITH) ./$(EXE) - -dA puzzles/extreme_12x12_30.txt

	

	
