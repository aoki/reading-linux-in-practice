set xlabel 'mem size: 2^x(KB)'
set logscale x
set xtics 2
// set format x "2^{%L}"

set ylabel 'time taken to access (ns/count)'
set ytics 2
set terminal png

set output "cache.png"
plot 'cache.dat'
