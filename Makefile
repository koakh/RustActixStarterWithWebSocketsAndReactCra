SHELL := /bin/bash
REACT_APP_PORT_WS := 8080
REACT_APP_PORT := 8081

start_server:
	@RUST_BACKTRACE=full \
	BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
	cargo run

start_client:
	@cd app && PORT=$(REACT_APP_PORT) REACT_APP_PORT=$(REACT_APP_PORT) REACT_APP_PORT_WS=$(REACT_APP_PORT_WS) npm start

.PHONY: start_server
