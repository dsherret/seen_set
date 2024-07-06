# benchmark

Output:

```
1/SeenSet               time:   [30.031 µs 30.654 µs 31.307 µs]
                        change: [-54.415% -53.424% -52.409%] (p = 0.00 < 0.05)
                        Performance has improved.
1/HashSet (Clone)       time:   [65.831 µs 65.953 µs 66.110 µs]
                        change: [-0.1120% +0.2313% +0.5435%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
1/HashSet (Reference)   time:   [39.349 µs 39.435 µs 39.559 µs]
                        change: [-2.7807% -2.2346% -1.7204%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

10/SeenSet              time:   [35.520 µs 36.333 µs 37.144 µs]
                        change: [-44.481% -43.121% -41.788%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
10/HashSet (Clone)      time:   [69.079 µs 69.200 µs 69.364 µs]
                        change: [-1.1392% -0.9111% -0.6559%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 16 outliers among 100 measurements (16.00%)
  7 (7.00%) high mild
  9 (9.00%) high severe
10/HashSet (Reference)  time:   [40.811 µs 40.893 µs 40.995 µs]
                        change: [-1.5001% -1.2290% -0.9872%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  3 (3.00%) high mild
  7 (7.00%) high severe

100/SeenSet             time:   [92.498 µs 92.549 µs 92.619 µs]
                        change: [-28.151% -27.957% -27.779%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 9 outliers among 100 measurements (9.00%)
  1 (1.00%) high mild
  8 (8.00%) high severe
100/HashSet (Clone)     time:   [148.26 µs 148.46 µs 148.71 µs]
                        change: [-0.7499% -0.5601% -0.3483%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
100/HashSet (Reference) time:   [103.08 µs 103.14 µs 103.21 µs]
                        change: [-0.4118% -0.2888% -0.1831%] (p = 0.00 < 0.05)
                        Change within noise threshold.

1000/SeenSet            time:   [801.72 µs 802.56 µs 803.83 µs]
                        change: [-4.4104% -4.0968% -3.7981%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) high mild
  15 (15.00%) high severe
Benchmarking 1000/HashSet (Clone): Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.4s, enable flat sampling, or reduce sample count to 60.
1000/HashSet (Clone)    time:   [1.0609 ms 1.0630 ms 1.0665 ms]
                        change: [-0.5212% -0.3878% -0.2533%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 21 outliers among 100 measurements (21.00%)
  7 (7.00%) low mild
  2 (2.00%) high mild
  12 (12.00%) high severe
1000/HashSet (Reference)
                        time:   [897.63 µs 897.90 µs 898.20 µs]
                        change: [+0.8214% +0.9283% +1.0231%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

10000/SeenSet           time:   [8.0412 ms 8.0478 ms 8.0553 ms]
                        change: [-0.8027% -0.6640% -0.5285%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
10000/HashSet (Clone)   time:   [9.7976 ms 9.8344 ms 9.8735 ms]
                        change: [-1.8850% -1.4569% -0.9823%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
10000/HashSet (Reference)
                        time:   [8.7367 ms 8.7489 ms 8.7650 ms]
                        change: [-0.6188% -0.3164% -0.0370%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  4 (4.00%) low mild
  3 (3.00%) high mild
  6 (6.00%) high severe
```
