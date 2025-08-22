# last_words

**last_words** is a small CLI tool designed to help you debug container startup issues quickly.
If you frequently start Docker containers (in Swarm, Compose, or automated tests), you know how frustrating it can be to track down why a container fails to start—especially when logs are not forwarded or are hard to access.

**last_words** attaches to the logs of matching containers and exits as soon as the container stops, making it easy to see the last output before a crash.

---

## Usage

Suppose you have a container whose name starts with `busy_box`:

```sh
last_words busy

```sh 
last_words busy
```
### Output: 
```
last words for docker container busy...
hello
world
```

- **last_words** will find all containers whose names start with `busy` and stream their logs to stdout.
- If the container exits, **last_words** will also exit, ensuring you see the final logs before the container stops.

### Docker Environments

**last_words** respects the `DOCKER_HOST` environment variable.

## Installation

### Manual

> **Note**: Make shure you hace [rust](https://www.rust-lang.org/tools/install) installed.

```sh
  git clone git@github.com:TiDe-bit/last-words.git && \
  cd last_words && \
  cargo install --path . && \
  cd .. && \
  rm -fr ./last_words
```

## Why last_words?

- **Simple**: Just provide a partial container name.
- **Fast**: No need to manually check logs or dig through log files.
- **Reliable**: Exits when the container does, so you never miss the last log lines.
