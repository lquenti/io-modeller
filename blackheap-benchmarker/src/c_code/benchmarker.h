#ifndef BLACKHEAP_BENCHMARKER_BENCHMARER_H
#define BLACKHEAP_BENCHMARKER_BENCHMARER_H


#define MEMINFO "/proc/meminfo"

/* https://www.kernel.org/doc/Documentation/sysctl/vm.txt */
#define DROP_PAGE_CACHE "/proc/sys/vm/drop_caches"

#include<stdbool.h>
#include<stddef.h>

/* All possible access patterns */
enum access_pattern {
  ACCESS_PATTERN_CONST = 0,
  ACCESS_PATTERN_SEQUENTIAL = 1,
  ACCESS_PATTERN_RANDOM = 2,
};

struct benchmark_config {
  const char *filepath;
  const size_t memory_buffer_in_bytes;
  const size_t file_size_in_bytes;
  const size_t access_size_in_bytes;
  const size_t number_of_io_op_tests;
  const enum access_pattern access_pattern_in_memory;
  const enum access_pattern access_pattern_in_ile;
  const bool is_read_operation;
  /* Whether the file should be bloated up to file_size_in_bytes.
   *
   * In most cases, this should be true.
   * The only expections are special "files" that can't be made bigger like
   * special devices.
   */
  const bool prepare_file_size;
  const bool use_o_direct;
  const bool drop_cache_first;
  const bool do_reread;
  const bool delete_afterwards;
  const size_t restrict_free_ram_to;
};

struct benchmark_results {
  size_t length;
  double *durations;
};

enum error_codes {
  SUCCESS = 0,

  /* Linux operations that failed */
  MALLOC_FAILED = 1,
  OPEN_FAILED = 2,
  READ_FAILED = 3,
  WRITE_FAILED = 4,
  CLOSE_FAILED = 5,
  LSEEK_FAILED = 6,
  FSYNC_FAILED = 7,
  FSTAT_FAILED = 8,
  IO_OP_FAILED = 9,
  REMOVE_FAILED = 10
};

struct benchmark_results benchmark_file(const struct benchmark_config *config);

#endif
