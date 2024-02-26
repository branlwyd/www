# www

bran.land server code

# Build/deploy instructions

## Testing

To get a testing build that serves HTTP on port 8080:

```
$ cargo run --features test
```

## Releasing

To deploy, on the build machine:

```
$ cargo build --target x86_64-unknown-linux-musl --release
$ scp target/x86_64-unknown-linux-musl/release/www bran.land:www
```

On the server:

```
# chown www:www www
# setcap 'cap_net_bind_service=+ep' www
# mv www /home/www/www
# systemctl restart www
```