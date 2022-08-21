# conc-map-bench

conc-map-bench uses the bustle benchmarking harness. This is a port of the well regarded libcuckoo benchmark.

Implementations benchmarked: SCC, DashMap, and flurry.

```sh
> cargo tree | rg 'dashmap|flurry|scc'
```
```
├── dashmap v5.3.4
├── flurry v0.4.0
├── scc v0.9.0
```

## Workloads

### Ready Heavy

A read heavy model with few inserts, removals and updates. Models caching of data in places such as webservers and disk page caches.
```
read   98%
insert  1%
remove  1%
```

### Exchange

Insert and remove heavy model that replicates a scenario where the map is used to exchange data.
```
read    10%
insert  40%
remove  40%
update  10%
```

### Rapid Grow

An insert heavy model that replicates load in a scenario where the map is used to gather large amounts of data under a short burst.
```
read    5%
insert 80%
remove  5%
update 10%
```

## How to Run?

```sh
mv results results.bk
./scripts/bench.bash
./scripts/plot.bash
```

## Results

OS: macOS 12.5.1

CPU: Apple M1

Rust: 1.63.0

See the `results/` directory.

### Read Heavy (fx hasher)
| | |
:-------------------------:|:-------------------------:
![](results/ReadHeavy.fx.throughput.svg) | ![](results/ReadHeavy.fx.latency.svg)

### Exchange (fx hasher)
| | |
:-------------------------:|:-------------------------:
![](results/Exchange.fx.throughput.svg) | ![](results/Exchange.fx.latency.svg)

### Rapid Grow (fx hasher)
| | |
:-------------------------:|:-------------------------:
![](results/RapidGrow.fx.throughput.svg) | ![](results/RapidGrow.fx.latency.svg)

### Read Heavy (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/ReadHeavy.std.throughput.svg) | ![](results/ReadHeavy.std.latency.svg)

### Exchange (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/Exchange.std.throughput.svg) | ![](results/Exchange.std.latency.svg)

### Rapid Grow (std hasher)
| | |
:-------------------------:|:-------------------------:
![](results/RapidGrow.std.throughput.svg) | ![](results/RapidGrow.std.latency.svg)
