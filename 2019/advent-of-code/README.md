
Yes, I'm doing it wrong, I believe in Linux distributions :)

The contents of my ~/.cargo/config takes crates only from pre-installed
Debian packages (or libraries otherwise added to this folder):

    [source]

    [source.debian-packages]
    directory = "/usr/share/cargo/registry"

    [source.crates-io]
    replace-with = "debian-packages"

This prevents network activity when compiling, more specifically, this
prevents downloading, building, and linking unverified code when running
cargo builds.

With this general approach, I do not include version numbers in my
Cargo.toml dependencies - I use whatever is on the system, allowing
implicit bug fixes and upgrades when the system is updated.
