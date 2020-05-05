# Tide

## Build
```bash
DATABASE_URL="mysql://app:app@localhost:3306/app" cargo build --release
target/release/tide_web
```

## Performance

QPS = query per second

| Content    | QPS  |
|:-----------|:-----|
| Hello      | 100k |
| Redis(6.0) | 28k  |
| MySQL      | 30k  |
| PostgreSQL | 26k  |

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

##  Env
### Hardware
```text
iMac(Retina 5K, 27-inch, 2019) CPU 3GHz, 6 Core i5
Memory 32G 2667MHz DDR4
```

