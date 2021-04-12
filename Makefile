all: src/rshttpd
src/rshttpd: src/main.rs src/handle_client.rs
	rustc $< -o $@
	@strip $@
	@[ -e rshttpd ] || ln -s $@ ./rshttpd
clean:
	$(RM) src/rshttpd
