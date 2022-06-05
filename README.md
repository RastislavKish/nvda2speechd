# nvda2speechd

It is already possible to use SAPI in Wine for quite a some time, however, the default Microsoft voices are not particularly responsive or diverse in terms of supported languages, and installing others and configuring them can be challenging without sighted assistance.

nvda2speechd is a bridge, which can link applications capable of speaking through NVDA right into Speech dispatcher installed on the user's computer.

It consists of two parts - client and server. The client is a Windows dll library, that implements the interface of the widely used nvda controller client lib. When an application loads and uses this library, it behaves as the original, except under the hood, it opens a websocket connection to the server outside of Wine and forwards all requests.

The server is a Linux program that waits for connections from clients and translates their requests to Speech dispatcher.

This way, SD can be used with any Windows application supporting NVDA through the screenreader's controller library.

## Build

Both client and server are written in Rust, and can be compiled using cargo.

The important part is to build each with the right toolchain. The server needs to be built for Linux, the architecture doesn't play a role as far as it's supported by the user's system. libspeechd-dev apt package needs to be installed for compilation purposes.

The client is a Windows dll, and therefore needs to be compiled either right on Windows using Rust's MSVC (by default) toolchain, or on Linux, making use of mingw-w64. The architecture needs to match the one of the target application.

## Usage

After either compiling or downloading a pre-built version of the nvda2speechd client, rename the library to either nvdaControllerClient32.dll or nvdaControllerClient64.dll, according to the architecture. This step is not performed automatically to avoid confusion with the original library.

Then, replace the real NVDA controller dll with the one of nvda2speechd.

Launch the server in your Linux environment, and start the Windows application in Wine. The Speech should be routed to speech Dispatcher.

## Setting up the connection host and port

You can set the used host for the client by setting up an environment variable NVDA2SPEECHD_HOST to an address of format:

ws://localhost:3457

This is the default configuration, and you can set any address / port for the client to use.

The server also uses this variable for configuration. Though, just the port is used in this case, the address is always set to 127.0.0.1.

## License

This project is licensed under the terms of GNU General Public license, except for the client, which is licensed under the GNU Lesser General Public License.

