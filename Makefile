c:
	cc -o bin/$$(basename $(f) .c)c $(f)
.PHONY: c

co3:
	cc -O3 -o bin/$$(basename $(f) .c)c $(f)
.PHONY: c

rs:
	rustc --out-dir bin $(f)
.PHONY: rs

