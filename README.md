# Blackheap

This tool creates a performance model of your I/O speed and further allows to predict future preformance.

## Note

There is a **big recode** right now. But how does one eat an elephant...

## Progress
- [x] Start new repo
- [ ] Requirements Engineering
  - [x] Find out how to formalize what the code should do
    - fuck formalization, we use pseudocode
  - [ ] Look what the code did before by looking through...
    - [ ] <https://lquenti.github.io/blackheap/book/>
    - [ ] <https://gwdg.github.io/iofs/book/>
    - [ ] `blackheap-benchmark`
    - [ ] `blackheap-modeller`
    - [ ] `blackheap-frontend`
    - [ ] `preloadee`
    - [ ] All open issues
    - [ ] All closed isses
    - Unstructured ideas:
      - Has to be resumeable
      - be able to re-analyze raw data again
      - Provide machine generated README in the data
      - `blackheap-benchmarker` does not rely on bindgen compile time since this would create LLVM as dep
      - [ ] Also provide a standalone binary for the benchmarker?
      - [ ] Find a way to have C linting (all warnings, formatter, pedantic C standard)
- [ ] Design a high level architecture based on the requirements
- [ ] Start writing the benchmarker
  - [x] Figure out how C/Rust interop work with a simple test (do some computations in C, test in Rust)
  - [ ] Find a way to have C linting (all warnings, formatter, pedantic C standard)
    - [ ] Have a minimal C program that calls all functions
      - [ ] Build with `-fsanitize=address,undefined`
    - [ ] add `valgrind --leak-check=full`
  - [ ] 
- [ ] ...

- [ ] rewrite docs
  - [ ] Check whether there was some information in the old docs that is lost

## Architecture
Cargo workspace with the following crates
```
- blackheap-core (lib): stuff all other libraries need (like Definitions)
- blackheap-benchmarker (lib): C code with Rust wrapper
- blackheap-analyzer (lib): Analysis of the benchmarks
- blackheap (bin): The user facing code
```

High level workflow:
```
- user starts blackheap with config parameters
- blackheap checks which benchmarks are already done
  - If folder doesnt exist / was not provided, do all benchmarks
- based on those benchmarks, do the analysis
- Create (human-readable) plots
- Create (machine-readable) parsing data
```

Benchmarker:
```
- get input from Rust struct
- give output in Rust struct
- Rust part should support a to json function for persistence
```
