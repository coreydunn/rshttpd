all: rshttpd
rshttpd: main.rs
	rustc $^ -o $@
	@strip $@
clean:
	$(RM) rshttpd
