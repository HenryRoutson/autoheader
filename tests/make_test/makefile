
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


# compile c files into object files
%.o: %.c
	cargo run $<
	$(CC) $(CFLAGS) $(CPPFLAGS) -c -o $@ $<

# link object files together into executable
$(EXE): $(OBJ) 
	$(COMPILER) $(OBJ) $(CPPFLAGS) -o $(EXE) $(LDFLAGS)

clean:
	rm -f $(EXE) *.o 
	rm -f *-functions.h 

run: $(EXE)
	make
	./$(EXE) 

