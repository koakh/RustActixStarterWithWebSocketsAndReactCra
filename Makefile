SHELL := /bin/bash
REACT_APP_PORT := 8081

run_server:
	RUST_BACKTRACE=full \
	BIND_ADDR=0.0.0.0:8080 \
	cargo run

run_client:
  # force to connect to es port on 8080
	cd app && PORT=$(REACT_APP_PORT) REACT_APP_PORT=$(REACT_APP_PORT) REACT_APP_PORT_WS=8080 npm start

.PHONY: run_server
