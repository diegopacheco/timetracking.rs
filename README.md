### What the Program Does

The program tells you how many business days you have to work <BR>
How many hours did you worked, how many days you have to work <BR>,
based on how many houes you worked it let you know how much you need to do in avg. <BR>

Program Output
```bash
======================================
TIME_TRACKING.rs by Diego Pacheco
--------------------------------------
Project: work
Goal   : 200 working hours
======================================
 Today Is      :  2/5/2020
 Working Days  :  21
 Worked  Days  :  2
 Remain  Days  :  19
 Need to Work  :  190 hours total <<< 
 Need to Work  :  10.00 avg hours yet 
 AV 10 
 Hours Predictions 
 7h   :  147
 8h   :  168
 9h   :  189
 10h  :  210
=====================================
```

### Build
```bash
cargo build
```
### Run
```bash
cargo build -- release
./target/release/timetracking 10 0
Where: 10 is worked hours and 0 is the number of holidays
```

### Mixing with Bash
_hours.db
``` 
10
```
_holidays.db
``` 
0
```
h.sh
```
#!/bin/bash
HOURS=$(< ./_hours.db)
HOLIDAYS=$(< ./_holidays.db)
timetracking $HOURS $HOLIDAYS
```
Running 
```bash
chmod +x h.sh
./h.sh
```