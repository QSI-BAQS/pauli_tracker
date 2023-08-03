#include <filesystem>
#include <stdio.h>

#include "pauli_tracker.h"

namespace fs = std::filesystem;

int main(void) {
  Map_psvbfx *storage = map_psvbfx_new();
  Frames_hmpsvbfx *frames = frames_hmpsvbfx_init(3);
  Live_hmpefx *live = live_hmpefx_init(8);
  Live_bvpd *another_live = live_bvpd_init(4);

  frames_hmpsvbfx_track_x(frames, 0);
  live_hmpefx_track_x(live, 0);
  live_bvpd_track_x(another_live, 0);
  frames_hmpsvbfx_cx(frames, 0, 1);
  live_hmpefx_cx(live, 0, 1);
  live_bvpd_cx(another_live, 0, 1);
  frames_hmpsvbfx_measure_and_store(frames, 1, storage);
  frames_hmpsvbfx_track_y(frames, 2);
  live_hmpefx_track_y(live, 2);
  live_bvpd_track_y(another_live, 2);
  frames_hmpsvbfx_measure_and_store_all(frames, storage);

  fs::create_directories("output");
  live_hmpefx_serialize(live, "output/live.json");
  live_bvpd_serialize(another_live, "output/live_bvpd.json");
  map_psvbfx_serialize(storage, "output/storage.json");

  live_hmpefx_free(live);
  live_bvpd_free(another_live);
  frames_hmpsvbfx_free(frames);
  map_psvbfx_free(storage);

  Vec_b *v = vec_b_new();
  printf("%d\n", vec_b_is_empty(v));
  vec_b_free(v);

  return 0;
}
