#include <stdio.h>

#include "pauli_tracker.h"


int main(void) {
  Storage *storage = new_storage();
  put_some_stuff_into_storage(storage);
  RawVec_Tuple sorted = sort_storage(storage);
  printf("qubit: %lu\n", sorted.ptr->qubit);
  RawPauliVec pauli = raw_pauli_vec(sorted.ptr->pauli);
  printf("x correction %u\n", *pauli.left.ptr);
  printf("z correction %u\n", *pauli.right.ptr);
  free_storage(storage);
}
