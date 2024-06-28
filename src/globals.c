#include <stdio.h>
#include "w2c2_base.h"

// stubbed implementations for compilation

// for wabt
// void w2c_runtime_exceptionHandler(void*) {}
// void w2c_runtime_printErrorMessage(void*) {}

// for w2c2

void runtime__exceptionHandler(void*) {}
void runtime__printErrorMessage(void*) {}
void trap(Trap trap) {
    fprintf(stderr, "TRAP: %s\n", trapDescription(trap));
    abort();
}

// for both

// int main(int) { return 1;}

// code for initializing and cleaning up pointers 

typedef struct instance { wasmModuleInstance common;
    wasmMemory* m0;
    wasmTable t0;
} instance;

instance* witness_c_init() {
    instance* i = malloc(sizeof(struct instance));
    return i;
}

typedef void* (_resolver)(const char*, const char*);

_resolver* witness_c_resolver() {
    return NULL;
}

void witness_c_cleanup(instance * i) {
    free(i);
}