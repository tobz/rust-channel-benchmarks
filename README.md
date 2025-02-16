# Rust Channel Benchmarks
This is a highly modified fork of the crossbeam-channel benchmarks. to keep track of Kanal library stats in comparison with other competitors.
### Tests

* `seq`: A single thread sends `N` messages. Then it receives `N` messages.
* `spsc`: One thread sends `N` messages. Another thread receives `N` messages.
* `mpsc`: `T` threads send `N / T` messages each. One thread receives `N` messages.
* `mpmc`: `T` threads send `N / T` messages each. `T` other threads receive `N / T` messages each.
* `select_rx`: `T` threads send `N / T` messages each into a separate channel. Another thread receives `N` messages by selecting over the `T` channels.
* `select_both`: `T` threads send `N / T` messages each by selecting over `T` channels. `T` other threads receive `N / T` messages each by selecting over the `T` channels.

Default configuration:

- `N = 5000000`
- `T = 4`

### Running

Runs benchmarks, stores results into `*.csv` files in the target folder, and generates multiple png file for each test category:

```bash
# Results will be saved in `target`.
./run.sh
```

Dependencies:

- Rust (latest)
- Go
- Bash
- libcairo2-dev
- Python
  - pygal
  - cairosvg

### Contributing

You can follow [community benchmarks](https://github.com/fereidani/rust-channel-benchmarks/issues?q=label%3Abenchmark), and also share your results by opening an issue with the format shown in [results](#Results) section.

### Results


Machine: `AMD Ryzen Threadripper 2950X 16-Core Processor`<br />
Rust: `rustc 1.62.0`<br />
Go: `go version go1.18.3 linux/amd64`<br />
OS (`uname -a`): `Linux 5.13.0-35-generic #40~20.04.1-Ubuntu SMP Mon Mar 7 09:18:32 UTC 2022 x86_64 x86_64 x86_64 GNU/Linux`<br />
Date: July 15, 2022

![Benchmark bounded channel with size 0](https://i.imgur.com/NOP91jD.png)
![Benchmark bounded channel with size 1](https://i.imgur.com/MpsuWIi.png)
![Benchmark bounded channel with size n](https://i.imgur.com/9ebey2h.png)
![Benchmark unbounded channel](https://i.imgur.com/WgrFRtK.png)
