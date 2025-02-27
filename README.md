# nvda2speechd

It is already possible to use SAPI in Wine for quite a some time, however, the default Microsoft voices are not particularly responsive or diverse in terms of supported languages, and installing others and configuring them can be challenging without sighted assistance.

nvda2speechd is a bridge, which can link applications capable of speaking through NVDA right into Speech dispatcher installed on the user's computer.

It consists of two parts - client and server. The client is a Windows dll library, that implements the interface of the widely used nvda controller client lib. When an application loads and uses this library, it behaves as the original, except under the hood, it opens a websocket connection to the server outside of Wine and forwards all requests.

The server is a Linux program that waits for connections from clients and translates their requests to Speech dispatcher.

This way, SD can be used with any Windows application supporting NVDA through the screenreader's controller library.

## Build

### Dependencies

In order to build nvda2speechd, you need the following dependencies:

* Rust, see the [Rust installation page](https://www.rust-lang.org/tools/install) for installation instructions.
* libspeechd-dev
* Clang
* mingw-w64
* Rust windows toolchains
* [Just](https://github.com/casey/just), used for compilation

On Ubuntu, just run:

```
sudo apt update
sudo apt install libspeechd-dev clang mingw-w64
```

To get the apt dependencies.

And install the Rust Windows toolchains via rustup:

```
rustup target add x86_64-pc-windows-gnu
rustup target add i686-pc-windows-gnu
```

you can install Just using cargo:

```
cargo install just
```

### Compilation

When you have everything setup, clone and compile the project:

```
git clone https://github.com/RastislavKish/nvda2speechd
cd nvda2speechd
just build
```

See ```just --list``` for all available commands.

You can find the results in a newly created bin directory.

Also note when compiling with mingw-w64, for the 32 bit library, it may be also necessary to include libgcc_s_dw2-1.dll. In the Ubuntu's distribution of mingw-w64, you can find it under /usr/lib/gcc/i686-w64-mingw32/10-win32.

## Usage

After either compiling or downloading a pre-built version of the nvda2speechd client, replace the real NVDA controller dll with the one of nvda2speechd.

Launch the server in your Linux environment, and start the Windows application in Wine. The Speech should be routed to speech Dispatcher.

## Setting up the connection host and port

You can set the used host for the client by setting up an environment variable NVDA2SPEECHD_HOST to an address of format:

ws://localhost:3457

This is the default configuration, and you can set any address / port for the client to use.

The server also uses this variable for configuration. Though, just the port is used in this case, the address is always set to 127.0.0.1.

## License

This project is licensed under the terms of GNU General Public license, except for the client, which is licensed under the GNU Lesser General Public License.

