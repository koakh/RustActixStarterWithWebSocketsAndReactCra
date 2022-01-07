# README

## Start and Play with Project

follow [Test WebSocket and use HotRealod with React Project](NOTES.md#test-websocket-and-use-hotrealod-with-react-project)

or follow this quick notes

```shell
# start server with embbedded frontend on port 8080
$ make start_server

# minimal request
$ curl -X GET http://127.0.0.1:8080/hello

# open some frontend pages with console open at http://127.0.0.1:8080|8081 and test websockets
$ curl -X POST -H "Content-Type: application/json" -d '{"message": "hello after clear...."}' http://127.0.0.1:8080/ws-echo | jq
```

## Debug Projects

launch  `start_server` and launch F5 with "Attach to executable 'rust-react-starter'"
or  just launch F5 `"Debug executable 'rust-react-starter'"`

add breakpoint in `Ok(ok)` and test

```rust
match websocket_srv.send(message_to_client).await {
  Ok(ok) => debug!("{:?}", ok),
  Err(e) => error!("{:?}", e),
};
```

frontend must have server running in embbedded and not embbedded mode, both comunicate with port `8080`

```shell
$ make start_server
$ make start_client
```

add breakpoint to `console.log`

```typescript
switch (msgType) {
  case MessageToClientType.Echo:
    console.log(`data: [${JSON.stringify(data, undefined, 0)}]`);
    break;
```

api just launch F5 `"Debug React Frontend"`

```shell
# fire request
$ curl -X POST -H "Content-Type: application/json" -d '{"message": "hello after clear...."}' http://127.0.0.1:8080/ws-echo | jq
```
