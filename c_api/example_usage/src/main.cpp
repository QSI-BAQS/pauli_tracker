#include <filesystem>
#include <stdio.h>

#include "pauli_tracker.h"

namespace fs = std::filesystem;

int main(void) {
  Storage *storage = storage_new();
  Frames *frames = frames_init(3);
  Live *live = live_init(8);
  Live_bvpd *live_bvpd = live_bvpd_init(4);

  frames_track_x(frames, 0);
  live_track_x(live, 0);
  live_bvpd_track_x(live_bvpd, 0);
  frames_cx(frames, 0, 1);
  live_cx(live, 0, 1);
  live_bvpd_cx(live_bvpd, 0, 1);
  measure_and_store(frames, 1, storage);
  frames_track_y(frames, 2);
  live_track_y(live, 2);
  live_bvpd_track_y(live_bvpd, 2);
  measure_and_store_all(frames, storage);

  fs::create_directories("output");
  live_serialize(live, "output/live.json");
  live_bvpd_serialize(live_bvpd, "output/live_bvpd.json");
  storage_serialize(storage, "output/storage.json");

  live_free(live);
  live_bvpd_free(live_bvpd);
  frames_free(frames);
  storage_free(storage);

  return 0;
}
