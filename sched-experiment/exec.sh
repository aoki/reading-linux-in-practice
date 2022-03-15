#!/bin/bash -eu

taskset -c 0 ../bin/schedc 1 100 1 > 1core-1process.txt
taskset -c 0 ../bin/schedc 2 100 1 > 1core-2process.txt
taskset -c 0 ../bin/schedc 4 100 1 > 1core-4process.txt
