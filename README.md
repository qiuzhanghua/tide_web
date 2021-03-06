# Tide

## Build

```bash
DATABASE_URL="mysql://app:app@localhost:3306/app" cargo build --release
target/release/tide_web
```

## Performance

QPS = query per second

```bash
sudo sysctl -w net.inet.tcp.msl=500
```

| Content    | TIME_WAIT=15s | TIME_WAIT=0.5s | Old Mac(0.5s) |
|:-----------|:--------------|:---------------|:--------------|
| Hello      | 100k          | 120k           | 55k           |
| Redis(6.0) | 28k           | 38k            | 15.5K         |
| MySQL      | 30k           | 31k            | 17.5K         |
| PostgreSQL | 26k           | 29K            | 14k           |

```text
➜  ~ wrk -t12 -c200 -d30s http://127.0.0.1:8080/
Running 30s test @ http://127.0.0.1:8080/
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     1.96ms  200.41us  14.83ms   96.16%
    Req/Sec     8.21k   151.16    10.33k    94.71%
  2948214 requests in 30.10s, 295.22MB read
Requests/sec:  97940.85
Transfer/sec:      9.81MB
➜  ~ wrk -t12 -c200 -d30s http://127.0.0.1:8080/redis
Running 30s test @ http://127.0.0.1:8080/redis
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     6.85ms    1.95ms  30.12ms   65.19%
    Req/Sec     2.35k   129.43     5.54k    98.06%
  842203 requests in 30.10s, 85.14MB read
Requests/sec:  27978.33
Transfer/sec:      2.83MB
➜  ~ wrk -t12 -c200 -d30s http://127.0.0.1:8080/my
Running 30s test @ http://127.0.0.1:8080/my
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.65ms    9.34ms 155.52ms   95.07%
    Req/Sec     2.49k   150.94     3.07k    71.31%
  893050 requests in 30.01s, 91.13MB read
Requests/sec:  29761.48
Transfer/sec:      3.04MB
➜  ~ wrk -t12 -c200 -d30s http://127.0.0.1:8080/pg
Running 30s test @ http://127.0.0.1:8080/pg
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     7.53ms    4.49ms 130.07ms   88.75%
    Req/Sec     2.21k   102.87     2.51k    76.36%
  792401 requests in 30.01s, 163.23MB read
  Socket errors: connect 0, read 30, write 0, timeout 0
Requests/sec:  26405.03
Transfer/sec:      5.44MB
```

## Env

### Hardware

```text
iMac(Retina 5K, 27-inch, 2019) CPU 3GHz, 6 Core i5
Memory 32G 2667MHz DDR4
```

### Old Mac
```text
MacBook Pro (Retina, Mid 2012)
CPU 2.6GHz, 4 core, Intel Core i7
Memory 16GB 1600 MHz DDR3
```

Deno 1.0(Hello) about 47K qps
```text
➜  ~ wrk -t12 -c200 -d30s http://127.0.0.1:8080/
Running 30s test @ http://127.0.0.1:8080/
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     4.12ms    2.08ms 130.63ms   99.01%
    Req/Sec     3.95k   293.50     7.81k    79.67%
  1413777 requests in 30.02s, 57.98MB read
  Socket errors: connect 0, read 90, write 0, timeout 0
Requests/sec:  47098.61
Transfer/sec:      1.93MB
```