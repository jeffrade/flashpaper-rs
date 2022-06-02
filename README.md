# flashpaper-rs

### Rust implementation of [FlashPaper](https://github.com/AndrewPaglusch/FlashPaper)

### Build
```
cargo build
```

### Running
You'll need to set (and securely persist) a static key named `FLASHPAPER_STATIC_KEY`. Here's an example of how to generate and export one on the host machine:
```
export FLASHPAPER_STATIC_KEY=$(openssl rand -hex 32)
```

Then to start, simply:
```
cargo run
```

### Docker
To build:
```
docker build -t flashpaper-rs .
```
To run:
```
export FLASHPAPER_STATIC_KEY=$(openssl rand -hex 32)
docker run -it -p 8321:8321 --env FLASHPAPER_STATIC_KEY flashpaper-rs
```

### TODO
 - Database should be encrypted at rest
 - Improve RNG implementation
 - Use Argon2i instead of default Argon2
