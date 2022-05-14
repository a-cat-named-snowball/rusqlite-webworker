
./build.sh builds

./run.sh starts a local server for testing. Its uses [miniserve](https://crates.io/crates/miniserve).

View on [http://localhost:8080/browser/index.html](http://localhost:8080/browser/index.html)

## TODO

- Return queried rows from web worker instead of dummy data
- Fix recursive mutex error in main_thread callbacks
- Handle SQL errors instead of panicking


Here's my update, I've:
- implemented the n+1 example you decribed.
- changed the code so there won't be any pointers passed to and from javascript.
- added more comments and cleaned up the code so it's easier to follow.

I have two remaining problems to solve:

1. Reusing the connection to the web worker inside of a callback. I was getting a recursive mutex runtime error. There may be a work around or I may have to make some larger changes to the code in main_thread.

2. Converting the rows struct returned by Rusqlite into a format that be transmitted back to the main thread.