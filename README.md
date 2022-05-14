
./build.sh builds

./run.sh starts a local server for testing. Its uses [miniserve](https://crates.io/crates/miniserve).

View on [http://localhost:8080/browser/index.html](http://localhost:8080/browser/index.html)

## TODO

- Finish SQL commands in web worker code
- Better data format for returning rows
- Fix recursive mutex error
- Add 'n+1' example
- Error handling, right now invalid sql queries cause a panic