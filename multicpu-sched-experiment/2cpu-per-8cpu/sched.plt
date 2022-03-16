set xlabel 't(ms)'
set ylabel 'process number'
set ytics 1
set terminal png

set output "2core-1process.png"
plot [] [0:3] '2core-1process.txt' using 2:1

set output "2core-2process.png"
plot [] [0:3] '2core-2process.txt' using 2:1

set output "2core-4process.png"
plot [] [0:3] '2core-4process.txt' using 2:1

# Progress
set key right bottom
set xlabel 't(ms)'
set ylabel 'progres'
set ytics 25
set terminal png

set output "2core-1process_progress.png"
plot [] [0:100] '2core-1process.txt' using 2:3

plot [] [0:100]  '< grep -e "^0" 2core-2process.txt' using 2:3
replot '< grep -e "^1" 2core-2process.txt' using 2:3
set output "2core-2process_progress.png"
replot


plot [] [0:100]  '< grep -e "^0" 2core-4process.txt' using 2:3
replot '< grep -e "^1" 2core-4process.txt' using 2:3
replot '< grep -e "^2" 2core-4process.txt' using 2:3
replot '< grep -e "^3" 2core-4process.txt' using 2:3
set output "2core-4process_progress.png"
replot
