HTTP_SERVER_URI := 0.0.0.0:8543
HTTP_SERVER_API_KEY := uOtXEZXYslKyB0n3g3xRmCaaNsAwB5KmgFcy1X7bbcbtS9dhOpKuhZ04Mfr2OKGL
# trace
RUST_LOG := trace,actix_server=debug,actix_web=debug
LOG_LEVEL := DEBUG
LOGFILE_LEVEL := DEBUG

build:
# @cargo build --locked --target x86_64-unknown-linux-gnu
	@cargo build

buildRelease:
	@cargo build --release --locked --target x86_64-unknown-linux-gnu

run:
	@RUST_LOG=$(RUST_LOG) LOG_LEVEL=$(LOG_LEVEL) LOG_LEVEL_FILE=$(LOG_LEVEL_FILE) cargo run

# startServer:
# 	@RUST_BACKTRACE=full \
#     BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
# 		HTTP_SERVER_URI=$(HTTP_SERVER_URI) \
# 		REACT_APP_SHOW_DEBUG_IN_CONSOLE_LOG=true \
# 		REACT_APP_HTTP_SERVER_API_KEY=$(HTTP_SERVER_API_KEY) \
# 		cargo run -- start-server

.PHONY: startServer
