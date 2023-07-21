#include "boolean_vector.hpp"

using namespace boolean_vector;

BoolVec *boolean_vector::bool_vec_new() {
  BoolVec *bool_vec = new BoolVec;
  return bool_vec;
}

void boolean_vector::bool_vec_free(BoolVec *bool_vec) { delete bool_vec; }

void boolean_vector::bool_vec_push(BoolVec *bool_vec, bool value) {
  bool_vec->push_back(value);
}

bool boolean_vector::bool_vec_pop(BoolVec *bool_vec) {
  bool value = bool_vec->back();
  bool_vec->pop_back();
  return value;
}

bool boolean_vector::bool_vec_get(BoolVec *bool_vec, int index) {
  return bool_vec->at(index);
}
