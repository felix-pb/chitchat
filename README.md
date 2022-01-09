# ChitChat

A simple web app to engage in trivial matters, i.e. to chitchat.

![screenshot](screenshot.png)

This project is built with:
- [Vue 2][1] and [Vuetify][2] for the frontend
- [Rust][3] and [Axum][4] for the backend
- [Docker][5] for packaging

Note: I'd be thrilled to use [Vue 3][6] and [Tailwind CSS][7].
I had prior experience with Vuetify so I used it here for quick prototyping.

[1]: https://vuejs.org/
[2]: https://vuetifyjs.com/en/
[3]: https://www.rust-lang.org/
[4]: https://github.com/tokio-rs/axum/
[5]: https://www.docker.com/
[6]: https://v3.vuejs.org/
[7]: https://tailwindcss.com/
[8]: https://tailwindui.com/

Top-level project structure:
- `/src`: contains the source code for the backend
- `/vue`: contains the source code for the frontend
- `/target` (.gitignored): contains the build artifacts for the backend when running `cargo build`
- `/static` (.gitignored): contains the build artifacts for the frontend when running `npm run build`
- `Dockerfile`: contains the instructions for packaging up the app
- `Makefile`: contains the aliases for frequently used commands

# Requirements

- Docker (latest): https://docs.docker.com/get-docker/
- Node (16.13.1): https://github.com/Schniz/fnm/ or https://github.com/nvm-sh/nvm/
- Rust (1.57.0): https://www.rust-lang.org/tools/install/

# How to build and run the app?

1. Build the app compose file:
```
docker compose build
```

2. Start the app compose file:
```
docker compose up
```

3. Use the app on http://localhost:3000.

4. Stop the app compose file:
```
docker compose down
```

# How to test the app?

1. Build the test compose file:
```
docker compose -f docker-compose.test.yaml build
```

2. Start the test compose file:
```
docker compose -f docker-compose.test.yaml up
```

3. Stop the test compose file:
```
docker compose -f docker-compose.test.yaml down
```

# How to format and lint the app?

1. Format and lint the backend code:
```
cargo fmt
cargo clippy
```

2. Format and lint the frontend code:
```
cd vue/
npm run lint
```
