
builddir := justfile_directory() / 'bin'
win64_target := 'x86_64-pc-windows-gnu'
win32_target := 'i686-pc-windows-gnu'

server_name := 'nvda2speechd'
client_64_name := 'nvdaControllerClient64.dll'
client_32_name := 'nvdaControllerClient32.dll'
testclient_name := 'testclient'

server_output_path := builddir/server_name
client_64_output_path := builddir/client_64_name
client_32_output_path := builddir/client_32_name
testclient_output_path := builddir/testclient_name

@build: build-server build-client

[working-directory: 'src/server']
@build-server:
  cargo build --release -q
  mkdir -p "{{ builddir }}"
  cp target/release/nvda2speechd '{{ server_output_path }}'

@build-client: build-client-64 build-client-32

[working-directory: 'src/client']
@build-client-64:
  cargo build --release --target {{ win64_target }} -q
  mkdir -p "{{ builddir }}"
  cp '{{ "target" / win64_target / "release/nvda2speechd.dll"}}' '{{ client_64_output_path }}'

[working-directory: 'src/client']
@build-client-32:
  cargo build --release --target {{ win32_target }} -q
  mkdir -p "{{ builddir }}"
  cp '{{ "target" / win32_target / "release/nvda2speechd.dll"}}' '{{ client_32_output_path }}'

[working-directory: 'src/testclient']
@build-testclient:
  cargo build --release -q
  mkdir -p "{{ builddir }}"
  cp 'target/release/testclient' '{{ testclient_output_path }}'

@clean: clean-server clean-client clean-testclient

[working-directory: 'src/server']
@clean-server:
  cargo clean -q
  if [ -f '{{ server_output_path }}' ]; then rm '{{ server_output_path }}'; fi

@clean-client: clean-client-64 clean-client-32

[working-directory: 'src/client']
@clean-client-64:
  cargo clean -q
  if [ -f '{{ client_64_output_path }}' ]; then rm '{{ client_64_output_path }}'; fi

[working-directory: 'src/client']
@clean-client-32:
  cargo clean -q
  if [ -f '{{ client_32_output_path }}' ]; then rm '{{ client_32_output_path }}'; fi

[working-directory: 'src/testclient']
@clean-testclient:
  cargo clean -q
  if [ -f '{{ testclient_output_path }}' ]; then rm '{{ testclient_output_path }}'; fi

[working-directory: 'src/server']
@lint-server:
  cargo clippy --release -q

[working-directory: 'src/client']
@lint-client-64:
  cargo clippy --target {{ win64_target }} --release -q

[working-directory: 'src/client']
@lint-client-32:
  cargo clippy --target {{ win32_target }} --release -q

[working-directory: 'src/testclient']
@lint-testclient:
  cargo clippy --release -q

