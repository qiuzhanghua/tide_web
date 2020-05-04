# Tide

## Performance
### Hello (127k qps)
```text
➜  ~ wrk -t12 -c200 -d30s http://127.0.0.1:8080
Running 30s test @ http://127.0.0.1:8080
  12 threads and 200 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     3.24ms    9.05ms 150.37ms   95.23%
    Req/Sec    10.68k   678.75    12.72k    75.75%
  3838891 requests in 30.10s, 384.41MB read
  Socket errors: connect 0, read 19, write 0, timeout 0
Requests/sec: 127529.45
Transfer/sec:     12.77MB
```
### Redis(6.0 38k qps)
```text
➜  ~ wrk -t12 -c100 -d30s http://127.0.0.1:8080/redis
Running 30s test @ http://127.0.0.1:8080/redis
  12 threads and 100 connections
  Thread Stats   Avg      Stdev     Max   +/- Stdev
    Latency     2.51ms    1.29ms  11.15ms   68.40%
    Req/Sec     3.22k   127.83     3.61k    68.29%
  1156472 requests in 30.10s, 116.91MB read
Requests/sec:  38418.81
Transfer/sec:      3.88MB
```

## Computer
```text
iMac(Retina 5K, 27-inch, 2019) CPU 3GHz, 6 Core i5
Memory 32G 2667MHz DDR4
```
