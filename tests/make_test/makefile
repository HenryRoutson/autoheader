
# OBJ            object files IE file.o
# EXE            name of executable file
# COMPILER       name to use compiler IE gcc OR clang
# CPPFLAGS       compliler flags
# LDFLAGS        linker flags

OBJ       = list.o main.o
EXE       = exe
COMPILER  = gcc
CPPFLAGS  = -Wall -Werror -O3 -g 
LDFLAGS   = -lm




cargo_run: 
	cargo run

$(EXE): cargo_run $(OBJ) 
	$(COMPILER) $(OBJ) $(CPPFLAGS) -o $(EXE) $(LDFLAGS)

clean:
	rm -f $(EXE) *.o 
	rm -f *-functions.h 

run: $(EXE)
	./$(EXE) 

