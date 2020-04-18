# rust-opensc-sys
Unsafe FFI bindings of [OpenSC](https://github.com/OpenSC/OpenSC) for the Rust programming language.

# Usage

OpenSC is designed to provide a PKCS#11 API and be used as middleware through a set of tools that come with it. This is also why installing OpenSC doesn't install development headers. Using these tools in a script-like manner also satisfies the terms of the LGPL. This crate is for when the tools aren't versatile enough and you want to interface directly with the OpenSC library from Rust, e. g. for talking to a only partially ISO/IEC 7816 conformant card.

These are unsafe FFI Wrappers created by bindgen, please take a look at the safe Rust interface (soon TM) instead of using these directly.

The OpenSC shared library must be installed on you system to build the crate and run the resulting binary.

Since OpenSC is included as a git submodule, when cloning run `git clone --recursive https://github.com/heinzelotto/rust-opensc-sys` to retrieve the submodule as well.

# Licensing

Please note that OpenSC is LGPL-2.1 licensed. This crate will link dynamically against OpenSC to ensure the requirements of the LGPL are satisfied. The crate itself (except the vendored copy of OpenSC) is licensed as MIT/Apache.
