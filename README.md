# conc-map-bench

conc-map-bench uses the bustle benchmarking harness. This is a port of the well regarded libcuckoo benchmark.

Implementations benchmarked
- RwLock Std HashMap
- CHashMap
- Contrie
- Flurry
- DashMap
- Evmap
- Crossbeam Skiplist
- SCC

```sh
> cargo tree | rg 'dashmap|chashmap|contrie|flurry|evmap|crossbeam-skiplist'
```
```
├── chashmap v2.2.2
├── contrie v0.1.4
├── crossbeam-skiplist v0.0.0 (https://github.com/crossbeam-rs/crossbeam#e1d64600)
├── dashmap v5.0.0
├── evmap v10.0.2
├── flurry v0.3.1
├── scc v0.6.5
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

## How to run it?

```sh
mv results results.bk
./scripts/bench.bash
./scripts/plot.bash
```

## Results

OS: MacOS Monterey
CPU: Apple M1

See the `results/` directory.

### Ready Heavy (fx hasher)
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
