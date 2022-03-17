set xlabel 't(ms)'
set ylabel 'process number'
set ytics 1
set terminal png

set output "1core-2process.png"
plot [] [0:3] '1core-2process.txt' using 2:1

# Progress
set key right bottom
set xlabel 't(ms)'
set ylabel 'progres'
set ytics 25
set terminal png

plot [] [0:100]  '< grep -e "^0" 1core-2process.txt' using 2:3
replot '< grep -e "^1" 1core-2process.txt' using 2:3
set output "1core-2process_progress.png"
replot
