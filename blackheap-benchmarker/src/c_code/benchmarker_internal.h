#ifndef BLACKHEAP_BENCHMARKER_BENCHMARER_INTERNAL_H
#define BLACKHEAP_BENCHMARKER_BENCHMARER_INTERNAL_H

#include<stdlib.h>
#include"./benchmarker.h"

struct benchmark_state {
  void *buffer;
  int fd;
  size_t last_mem_offset;
  size_t last_file_offset;
  ssize_t (*io_op)(int fd, void *buf, size_t count);
};

/* init */
enum error_codes drop_page_cache();
enum error_codes init_state(const struct benchmark_config *config, struct benchmark_state *state);
enum error_codes init_file(const struct benchmark_config *config, struct benchmark_state *state);
enum error_codes init_results(const struct benchmark_config *config, struct benchmark_results *results);

/* Benchmarking helpers */
enum error_codes reread(const struct benchmark_config *config, const struct benchmark_state *state);
double timespec_to_double(const struct timespec *time);
void pick_next_mem_position(const struct benchmark_config *config, struct benchmark_state *state);
enum error_codes pick_next_file_position(const struct benchmark_config *config, struct benchmark_state *state);

/* Benchmarking function */
enum error_codes do_benchmark(const struct benchmark_config *config, struct benchmark_state *state, struct benchmark_results *results);



/* do_cleanup is best effort */
void do_cleanup(const struct benchmark_config *config, struct benchmark_state *state);

#endif
