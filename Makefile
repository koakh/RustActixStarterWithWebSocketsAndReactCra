SHELL := /bin/bash
REACT_APP_PORT_WS := 8080
REACT_APP_PORT := 8081

# exposes api and embebbed frontend on port 8080
start_server:
	@RUST_BACKTRACE=full \
    BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
    cargo run

# exposes frontend on port 8081, require 3 ports bellow
start_client:
	@cd app && \
    HOST=127.0.0.1 \
    PORT=$(REACT_APP_PORT) \
    REACT_APP_PORT=$(REACT_APP_PORT) \
    REACT_APP_PORT_WS=$(REACT_APP_PORT_WS) \
    npm start

.PHONY: start_server
