#include <stdio.h>

#include "tracker.h"
#include "boolean_vector.hpp"

int main(void) {
  Storage *storage = storage_new();
  Frames *frames = frames_init(3);
  Live *live = live_init(3);

  frames_track_x(frames, 0);
  live_track_x(live, 0);
  frames_cx(frames, 0, 1);
  live_cx(live, 0, 1);
  measure_and_store(frames, 1, storage);
  frames_track_y(frames, 2);
  live_track_y(live, 2);
  measure_and_store_all(frames, storage);

  live_serialize(live, "output/live.json");
  storage_serialize(storage, "output/storage.json");

  live_free(live);
  frames_free(frames);
  storage_free(storage);

  test();

  return 0;
}
