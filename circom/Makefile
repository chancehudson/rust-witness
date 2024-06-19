CC=g++
CFLAGS=-std=c++11 -D_LONG_LONG_LIMB -O3 -I. -I../depends/json/include -I../depends/gmp/package/include -L../depends/gmp/package/lib/
# FR_CPP = fr.cpp fr_generic.cpp fr_raw_generic.cpp
DEPS_O = main.o calcwit.o fr.o fr_generic.o fr_raw_generic.o stub_circuit.o # fr_asm.o
	
all: $(DEPS_O)
	$(CC) *.o -lgmp $(CFLAGS) -v
	
%.o: %.cpp $(DEPS_HPP)
	$(CC) -c $< $(CFLAGS)

clean:
	rm $(DEPS_O)