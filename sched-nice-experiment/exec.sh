#!/bin/bash -eu

taskset -c 0 ../bin/sched_nicec 100 1 > 1core-2process.txt
