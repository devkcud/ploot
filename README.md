# 🪲 Ploot

## 🗿 Overview

> [!IMPORTANT]
> This project is not meant to be used in production or any other kind of environment. It was created as a POC. Read more bellow.

**Ploot** is a web server project written in Rust, primarily serving as a _[Proof of Concept (POC)](https://en.wikipedia.org/wiki/Proof_of_concept)_ rather than being production-ready. It's designed for experimental purposes ~~and does not currently have plans for extensive development or deployment~~, I will continue to develop it. See moving section below.

## ✈️ Moving

I'm currently in the process of moving some projects/repositories over to my GitLab account, including this one. I plan to do more work there, experimenting with new features and exploring ideas I like and things I don't see (but I want lol) in other frameworks and web servers.

New repo (I will rewrite the entire project): [ploot](https://gitlab.com/patomcio/ploot)

> I've been using GitLab for a while, but I recently switched to GitHub because of a job change. However, I don't work there anymore.

## 📦️ Features

- Minimalistic web server implementation.
- Demonstrates basic HTTP request handling.
- Generic routes.
- Route groups handling.
- Simple responses and requests using custom constructors.
- Designed for educational and experimental purposes. (Following Rust guide book)
- Not intended for production use.

## ➕ Installation

To run **Ploot** locally, ensure you have Rust installed on your system (_Tested on 1.77.0-nightly_). Then, clone the repo and cd into the project directory:

```bash
git clone https://git.com/devkcud/ploot.git
cd ploot
```

Next, compile and run the project using _Cargo_:

```bash
cargo run
```

> [!NOTE]
> The project uses, by default, the port `1234`, it's a pretty common port. Be sure that it's available.

This will start the web server on the default port with the default settings.

### ❓️ Usage

Ploot offers basic functionality as a web server, without the extensive features found in larger frameworks. You can test its functionality by accessing the default route using a web browser or tools such as cURL, Postman, or Insomnia:

```bash
curl localhost:1234/user/pato
```

> [!WARNING]
> Sometimes, `curl` needs a protocol at the beginning (at least on some of my tests), so if you run into any issues, try: `curl http://localhost:1234/user/pato`

## 🤓 Contributing

As this project is primarily a POC and not intended for extensive development, contributions are not actively sought. However, feel free to fork the repository and experiment with the codebase as you see fit.

## 🫵 License

This project is provided without any explicit license. You are free to use the code as you see fit, with no restrictions or obligations. However, please be aware that the author of this project disclaims any responsibility or liability for any consequences resulting from the use of this code in a production (or any other kind of) environment.

> at least it doesn't use any unsafe keywords :P
