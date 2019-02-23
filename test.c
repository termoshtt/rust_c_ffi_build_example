#include <stdlib.h>

typedef struct {
  int value;
} DATA;

DATA* new_data() { return (DATA*)malloc(sizeof(DATA)); }
void del_data(DATA* data) { free(data); }
