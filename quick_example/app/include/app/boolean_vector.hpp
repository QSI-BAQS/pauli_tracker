#pragma once

#include <vector>

namespace boolean_vector {
typedef std::vector<bool> BoolVec;

extern "C" BoolVec *bool_vec_new();
extern "C" void bool_vec_free(BoolVec *bool_vec);
extern "C" void bool_vec_push(BoolVec *bool_vec, bool value);
extern "C" bool bool_vec_pop(BoolVec *bool_vec);
extern "C" bool bool_vec_get(BoolVec *bool_vec, int index);
} // namespace boolean_vector
