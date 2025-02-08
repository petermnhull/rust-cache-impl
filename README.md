# rust-cache-impl

Implementation of a service in Rust for keeping a cache up to date and running side-effects when identifying new data depending on its status.

## Example Run

This shows how an external process updating state in a database gets propagated to the cache, including running side-effects and removing unneeded data.

https://github.com/user-attachments/assets/169a6fc0-41b4-41f9-9fb9-fc6aaa236976
