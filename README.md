# rust-cache-impl

Implementation of a service in Rust for keeping a cache up to date and running side-effects when identifying new data depending on its status.

## Example Run

This shows how when the database is updated by an external process, the changes get propagated to the cache and relevant side-effects take place (such as "Finished" tasks getting removed from the cache).

https://github.com/user-attachments/assets/337d61c3-963a-4d7e-9ab3-4dd1d46442cd

