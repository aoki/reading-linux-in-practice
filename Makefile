rs:
	rustc --out-dir bin $(f)
.PHONY: rs

c:
	cc -o bin/$$(basename $(f) .c)c $(f)
.PHONY: c
