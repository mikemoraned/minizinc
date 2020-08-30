# Results from sample runs

```minizinc
int: nEmployees = 7;
int: nDays = 4 * 7; % 4 * 7 is taken "as one calendar month"
enum SHIFT = { MORNING, DAY_TIME, EVENING };
```

Best found after 5 minutes, optimising for consistency (number of consecutive shifts done by the same person):

```plain
1, MON: 7, 0, 5
2, TUE: 5, 0, 6
3, WED: 6, 0, 5
4, THU: 5, 0, 4
5, FRI: 4, 0, 3
6, SAT: 5, 5, 5
7, SUN: 5, 5, 5
8, MON: 5, 0, 4
9, TUE: 4, 0, 3
10, WED: 3, 0, 4
11, THU: 4, 0, 3
12, FRI: 3, 0, 4
13, SAT: 4, 4, 4
14, SUN: 4, 4, 2
15, MON: 2, 0, 1
16, TUE: 1, 0, 2
17, WED: 2, 0, 1
18, THU: 1, 0, 2
19, FRI: 2, 0, 1
20, SAT: 3, 3, 3
21, SUN: 3, 3, 3
22, MON: 2, 0, 1
23, TUE: 1, 0, 2
24, WED: 2, 0, 1
25, THU: 1, 0, 2
26, FRI: 2, 0, 1
27, SAT: 6, 6, 6
28, SUN: 6, 6, 6
commitment = [78, 89, 87, 96, 89, 64, 9], consistency = 38, avg_disparity = 26.2857142857143
```
