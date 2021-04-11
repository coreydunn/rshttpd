all: src/rshttpd
src/rshttpd: src/main.rs src/handle_client.rs
	rustc $< -o $@
	@strip $@
	@ln -s src/rshttpd ./rshttpd
clean:
	$(RM) src/rshttpd
