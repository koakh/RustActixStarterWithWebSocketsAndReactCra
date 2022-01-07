SHELL := /bin/bash
REACT_APP_PORT_WS := 8443
REACT_APP_PORT := 8444

# exposes api and embebbed frontend on port 8443
start_server:
	@RUST_BACKTRACE=full \
    BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
    cargo run

# exposes frontend on port 8444, require 3 ports bellow
start_client:
	@cd app && \
    HOST=127.0.0.1 \
    HTTPS=true \
    PORT=$(REACT_APP_PORT) \
    NODE_TLS_REJECT_UNAUTHORIZED="0" \
    REACT_APP_PORT=$(REACT_APP_PORT) \
    REACT_APP_PORT_WS=$(REACT_APP_PORT_WS) \
    npm start

# always remove last build to prevent stalled files
deb:
	@rm app/build -r || true \
		&& cargo deb -v

.PHONY: start_server
