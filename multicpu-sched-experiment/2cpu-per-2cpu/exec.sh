#!/bin/bash -eu

taskset -c 0,1 ../../bin/schedc 1 100 1 > 2core-1process.txt
taskset -c 0,1 ../../bin/schedc 2 100 1 > 2core-2process.txt
taskset -c 0,1 ../../bin/schedc 4 100 1 > 2core-4process.txt
