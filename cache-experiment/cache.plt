# Fig. 06-10
set xlabel 'mem size: 2^x (KB/s)'
set xtics 2

set ylabel 'time taken to access (ns/count)'
set ytics 2

set terminal png

set output "cache.png"
plot 'cache.dat' using 1:3 with points pointtype 6

# Fig. 06-11
set ylabel 'time taken to access (log: ns/count)'
set logscale y

set ytics 0.25
set output "06-11.png"
plot 'cache.dat' using 1:3 with points pointtype 6
