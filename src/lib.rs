// Copyright © 2017–2021 Trevor Spiteri

// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of
// the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License and a copy of the GNU General Public License along with
// this program. If not, see <https://www.gnu.org/licenses/>.

/*!
# Rust low-level bindings for GMP, MPFR and MPC

The gmp-mpfr-sys crate provides Rust FFI bindings to the following
[GNU] arbitrary-precision libraries:

  * [GMP] for integers and rational numbers,
  * [MPFR] for floating-point numbers, and
  * [MPC] for complex numbers.

The source of the three libraries is included in the package.

The gmp-mpfr-sys crate is free software: you can redistribute it
and/or modify it under the terms of the GNU Lesser General Public
License as published by the Free Software Foundation, either version 3
of the License, or (at your option) any later version. See the full
text of the [GNU LGPL] and [GNU GPL] for details.

## Basic features

This crate contains three modules:

  * [`gmp`] provides external FFI bindings to [GMP].
  * [`mpfr`] provides external FFI bindings to [MPFR].
  * [`mpc`] provides external FFI bindings to [MPC].

The versions provided by this crate release are [GMP] version 6.2.1,
[MPFR] version 4.1.0-p13, and [MPC] version 1.2.1.

If you want a high-level API, consider using [Rug][rug crate], a crate
which provides integers and floating-point numbers with arbitrary
precision and correct rounding:

  * [`Integer`] is a bignum integer with arbitrary precision.
  * [`Rational`] is a bignum rational number with arbitrary precision.
  * [`Float`] is a multi-precision floating-point number with correct
    rounding.
  * [`Complex`] is a multi-precision complex number with correct
    rounding.

### Name prefixes

Since modules and enumerated types provide namespacing, most prefixes
in the C names are removed. However, when the prefix is not a whole
word it is not removed. For example [`mp_set_memory_functions`]
becomes [`gmp::set_memory_functions`], but [`mpz_init`] becomes
[`gmp::mpz_init`] not `gmp::z_init`, and [`MPFR_RNDN`] in
[`enum MPFR_RND_T`] becomes [`mpfr::rnd_t::RNDN`] not
`mpfr::rnd_t::N`. Also, the types [`mpfr::mpfr_t`] and [`mpc::mpc_t`]
are *not* shortened to `mpfr::t` or `mpc::t`.

### Types

Unlike in the C libraries, the types [`gmp::mpz_t`], [`gmp::mpq_t`],
[`gmp::mpf_t`], [`gmp::randstate_t`], [`mpfr::mpfr_t`] and
[`mpc::mpc_t`] are defined directly as structs, not as single-element
arrays.

### Undocumented or obsolete functions

The bindings do not cover undocumented or obsolete functions and
macros.

## Using gmp-mpfr-sys

The gmp-mpfr-sys crate is available on [crates.io][sys crate]. To use
gmp-mpfr-sys in your crate, add it as a dependency inside
[*Cargo.toml*]:

```toml
[dependencies]
gmp-mpfr-sys = "1.4"
```

This crate required rustc version 1.37.0 or later.

If the C libraries have a major version bump with some deprecated
functions removed, but no features are removed in the Rust bindings,
then gmp-mpfr-sys will have a minor version bump rather than a major
version bump. This allows more compatiblity across crates that use the
Rust bindings but do not use the C libraries directly.

If on the other hand a dependent crate makes use of internal
implementation details, or includes a C library that directly uses the
header (*.h*) and library (*.a*) files built using C, it can be a good
idea to depend on version `"~1.4"` instead of version `"1.4"` in order
to ensure backwards compatibility at the C level as well.

## Optional features

The gmp-mpfr-sys crate has two optional features:

 1. `mpfr`, enabled by default. Required to include the [MPFR]
    library.
 2. `mpc`, enabled by default. Required to include the [MPC] library.
    This feature requires the `mpfr` feature.

The [GMP] library is always included.

The two optional features are enabled by default; to use features
selectively, you can add the dependency like this to [*Cargo.toml*]:

```toml
[dependencies.gmp-mpfr-sys]
version = "1.4"
default-features = false
features = ["mpfr"]
```

Here only the `mpfr` feature is selected.

## Experimental optional features

It is not considered a breaking change if experimental features are
removed. The removal of experimental features would however require a
minor version bump.

Experimental features may also not work on all platforms.

There are three experimental feature:

 1. `use-system-libs`, disabled by default. Using this feature, the
    system libraries for [GMP], and [MPFR] and [MPC] if enabled, will
    be used instead of building them from source. The major versions
    of the system libraries must be equal to those provided by the
    crate, and the minor versions of the system libraries must be
    greater or equal to those provided by the crate. There are no
    restriction on the patch version.
 2. `force-cross`, disabled by default. Without this feature, the
    build will fail if cross compilation is detected, because cross
    compilation is not tested or supported and can lead to silent
    failures that are hard to debug, especially if this crate is an
    indirect dependency. As an exception, cross compiling from x86_64
    to i686 does not need this feature. (Compiling on MinGW does not
    have this exception because MinGW does not support cross
    compilation from 64-bit to 32-bit.)
 3. `c-no-tests`, disabled by default. Using this feature will skip
    testing the C libraries. This is not advised; the risk that the
    GMP sources are miscompiled is unfortunately quite high. And if
    they indeed are miscompiled, the tests are very likely to trigger
    the compiler-introduced bug.

## Metadata

The gmp-mpfr-sys crate passes some metadata to its dependents:

 1. `DEP_GMP_LIMB_BITS` contains the number of bits per limb, which is
    32 or 64.
 2. `DEP_GMP_OUT_DIR` contains the path of a directory that contains
    two subdirectories: the first subdirectory is named *lib* and
    contains the generated library (*.a*) files, and the second
    subdirectory is named *include* and contains the corresponding
    header (*.h*) files.
 3. `DEP_GMP_LIB_DIR` contains the path of the *lib* subdirectory of
    the `DEP_GMP_OUT_DIR` directory.
 4. `DEP_GMP_INCLUDE_DIR` contains the path of the *include*
    subdirectory of the `DEP_GMP_OUT_DIR` directory.

A dependent crate can use these environment variables in its build
script.

## Building on GNU/Linux

To build on GNU/Linux, simply make sure you have `diffutils`, `gcc`,
`m4` and `make` installed on your system. For example on Fedora:

```sh
sudo dnf install diffutils gcc m4 make
```

## Building on macOS

To build on macOS, you need the command-line developer tools. To
install them, run the following command in a terminal:

```sh
xcode-select --install
```

## Building on Windows

You can build on Windows with the Rust GNU toolchain and an up-to-date
MSYS2 installation. Some steps for a 64-bit environment are listed
below. (32-bit: Changes for a 32-bit environment are written in
brackets like this comment.)

To install MSYS2:

 1. Install MSYS2 using the [installer][msys].

 2. Launch the MSYS2 MinGW 64-bit terminal from the start
    menu. (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)

 3. Install the required tools.

    ```sh
    pacman -S pacman-mirrors
    pacman -S diffutils m4 make mingw-w64-x86_64-gcc
    ```

    (32-bit: Install `mingw-w64-i686-gcc` instead of
    `mingw-w64-x86_64-gcc`.)

Then, to build a crate with a dependency on this crate:

 1. Launch the MSYS2 MinGW 64-bit terminal from the start menu.
    (32-bit: Launch the MSYS2 MinGW 32-bit terminal instead.)

 2. Change to the crate directory.

 3. Build the crate using `cargo`.

## Cross compilation

While some cross compilation is possible, it is not tested
automatically, and may not work. Merge requests that improve cross
compilation are accepted.

The experimental feature `force-cross` must be enabled for cross
compilation. There is one case which is allowed even without the
feature: when the only difference between host and target is that the
host is x86_64 and the target is i686.

## Caching the built C libraries

Building the C libraries can take some time. In order to save
compilation time, the built libraries are cached in the user’s cache
directory as follows:

  * on GNU/Linux: inside `$XDG_CACHE_HOME/gmp-mpfr-sys` or
    `$HOME/.cache/gmp-mpfr-sys`
  * on macOS: inside `$HOME/Library/Caches/gmp-mpfr-sys`
  * on Windows: inside `{FOLDERID_LocalAppData}\gmp-mpfr-sys`

To use a different directory, you can set the environment variable
`GMP_MPFR_SYS_CACHE` to the desired cache directory. Setting the
`GMP_MPFR_SYS_CACHE` variable to an empty string or to a single
underscore (`"_"`)  will disable caching.

[*Cargo.toml*]: https://doc.rust-lang.org/cargo/guide/dependencies.html
[GMP]: https://gmplib.org/
[GNU GPL]: https://www.gnu.org/licenses/gpl-3.0.html
[GNU LGPL]: https://www.gnu.org/licenses/lgpl-3.0.en.html
[GNU]: https://www.gnu.org/
[MPC]: http://www.multiprecision.org/mpc/
[MPFR]: https://www.mpfr.org/
[`Complex`]: https://docs.rs/rug/&#42;/rug/struct.Complex.html
[`Float`]: https://docs.rs/rug/&#42;/rug/struct.Float.html
[`Integer`]: https://docs.rs/rug/&#42;/rug/struct.Integer.html
[`MPFR_RNDN`]: C/MPFR/constant.MPFR_Basics.html#Rounding-Modes
[`Rational`]: https://docs.rs/rug/&#42;/rug/struct.Rational.html
[`enum MPFR_RND_T`]: C/MPFR/constant.MPFR_Basics.html#index-mpfr_005frnd_005ft
[`mp_set_memory_functions`]: C/GMP/constant.Custom_Allocation.html#index-mp_005fset_005fmemory_005ffunctions
[`mpz_init`]: C/GMP/constant.Integer_Functions.html#index-mpz_005finit
[msys]: https://www.msys2.org/
[rug crate]: https://crates.io/crates/rug
[sys crate]: https://crates.io/crates/gmp-mpfr-sys
*/
#![no_std]
#![warn(missing_docs)]
#![doc(html_root_url = "https://docs.rs/gmp-mpfr-sys/~1.4")]
#![doc(html_logo_url = "data:image/svg+xml;base64,
PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiPz4KPCEtLSBDcmVhdGVkIHdpdGggSW5rc2NhcGUgKGh0dHA6Ly93
d3cuaW5rc2NhcGUub3JnLykgLS0+Cjxzdmcgd2lkdGg9IjEyOCIgaGVpZ2h0PSIxMjgiIHZlcnNpb249IjEuMSIgdmlld0JveD0i
MCAwIDMzLjg2NjY2NiAzMy44NjY2NjgiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6Y2M9Imh0dHA6
Ly9jcmVhdGl2ZWNvbW1vbnMub3JnL25zIyIgeG1sbnM6ZGM9Imh0dHA6Ly9wdXJsLm9yZy9kYy9lbGVtZW50cy8xLjEvIiB4bWxu
czpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPgogPG1ldGFkYXRhPgogIDxyZGY6UkRG
PgogICA8Y2M6V29yayByZGY6YWJvdXQ9IiI+CiAgICA8ZGM6Zm9ybWF0PmltYWdlL3N2Zyt4bWw8L2RjOmZvcm1hdD4KICAgIDxk
Yzp0eXBlIHJkZjpyZXNvdXJjZT0iaHR0cDovL3B1cmwub3JnL2RjL2RjbWl0eXBlL1N0aWxsSW1hZ2UiLz4KICAgIDxkYzp0aXRs
ZS8+CiAgIDwvY2M6V29yaz4KICA8L3JkZjpSREY+CiA8L21ldGFkYXRhPgogPGcgdHJhbnNmb3JtPSJ0cmFuc2xhdGUoMCAtMjYz
LjEzKSI+CiAgPHBhdGggZD0ibTMzLjg2NyAyODAuMDdhMTYuOTMzIDE2LjkzMyAwIDAgMSAtMTYuOTMzIDE2LjkzMyAxNi45MzMg
MTYuOTMzIDAgMCAxIC0xNi45MzMgLTE2LjkzMyAxNi45MzMgMTYuOTMzIDAgMCAxIDE2LjkzMyAtMTYuOTMzIDE2LjkzMyAxNi45
MzMgMCAwIDEgMTYuOTMzIDE2LjkzM3oiIGZpbGw9IiNmZmZmYzAiLz4KICA8ZyBmaWxsPSJub25lIiBzdHJva2U9IiMwMDAiIHN0
cm9rZS13aWR0aD0iLjI2NDU4cHgiPgogICA8ZyBhcmlhLWxhYmVsPSI2Ij4KICAgIDxwYXRoIGQ9Im0xNC40NzEgMjg0LjA3YzIu
MjAxMyAwIDQuMDM1OC0xLjQ2NzYgNC4wMzU4LTMuOTYyNCAwLTIuNDk0OC0xLjU0MDktMy41OTU1LTMuNTIyMS0zLjU5NTUtMC42
MjM3MSAwLTEuNjE0MyAwLjQwMzU4LTIuMTgzIDEuMTU1NyAwLjEyODQxLTIuMzQ4MSAxLjAwODktMy4xMzY5IDIuMTQ2My0zLjEz
NjkgMC42NjA0IDAgMS4zOTQyIDAuNDAzNTggMS43NjExIDAuODA3MTZsMS42NTEtMS44NzExYy0wLjc3MDQ3LTAuNzcwNDctMS45
ODEyLTEuNDY3Ni0zLjYzMjItMS40Njc2LTIuNDk0OCAwLTQuODA2MiAyLjAxNzktNC44MDYyIDYuMjM3MXMyLjMxMTQgNS44MzM1
IDQuNTQ5NCA1LjgzMzV6bS0wLjA3MzM4LTIuMzQ4MWMtMC41Njg2OCAwLTEuMjY1OC0wLjM4NTI0LTEuNTIyNi0yLjAxNzkgMC4z
NjY4OS0wLjY5NzA5IDAuOTM1NTctMC45OTA2IDEuNDg1OS0wLjk5MDYgMC42NjA0IDAgMS4yODQxIDAuMjc1MTYgMS4yODQxIDEu
Mzk0MiAwIDEuMjI5MS0wLjU4NzAyIDEuNjE0My0xLjI0NzQgMS42MTQzeiIgZmlsbD0iIzAwMTAzMCIgc3Ryb2tlPSJub25lIi8+
CiAgIDwvZz4KICAgPGcgdHJhbnNmb3JtPSJyb3RhdGUoMTUuNTE1KSIgYXJpYS1sYWJlbD0iMiI+CiAgICA8cGF0aCBkPSJtOTgu
MDM1IDI2Ny45NWg1LjA0NjF2LTEuMzk5OGgtMS40NDVjLTAuMzM4NjcgMC0wLjgzNTM4IDAuMDQ1Mi0xLjE5NjYgMC4wOTAzIDEu
MTYyOC0xLjIxOTIgMi4yOTE2LTIuNTA2MSAyLjI5MTYtMy43MTQgMC0xLjM3NzItMC45NTk1NS0yLjI4MDQtMi4zNzA3LTIuMjgw
NC0xLjAyNzMgMC0xLjY5MzMgMC4zODM4Mi0yLjQwNDUgMS4xNDAybDAuOTAzMTEgMC44OTE4MmMwLjM2MTI0LTAuMzgzODIgMC43
NTYzNi0wLjczMzc4IDEuMjc1Ni0wLjczMzc4IDAuNjIwODkgMCAxLjAxNiAwLjM4MzgzIDEuMDE2IDEuMDgzNyAwIDEuMDE2LTEu
Mjg2OSAyLjI0NjUtMy4xMTU3IDMuOTczN3oiIGZpbGw9IiMwMDEwMzAiIGZpbGwtb3BhY2l0eT0iLjk3MjU1IiBzdHJva2U9Im5v
bmUiLz4KICAgPC9nPgogICA8ZyB0cmFuc2Zvcm09InJvdGF0ZSgtMTEuMDMzKSIgYXJpYS1sYWJlbD0iOCI+CiAgICA8cGF0aCBk
PSJtLTM4LjkyIDI5MC43NmMxLjI3NDIgMCAyLjEyMzctMC43NDA4MyAyLjEyMzctMS43MDg4IDAtMC44Mzk2Mi0wLjUwMzc3LTEu
MzAzOS0xLjA4NjYtMS42Mjk4di0wLjAzOTVjMC40MDQ5OS0wLjI5NjMzIDAuODM5NjEtMC44MTk4NSAwLjgzOTYxLTEuNDQyMiAw
LTEuMDE3NC0wLjcyMTA4LTEuNjk5LTEuODQ3MS0xLjY5OS0xLjA4NjYgMC0xLjg5NjUgMC42NTE5My0xLjg5NjUgMS42NjkzIDAg
MC42NjE4MSAwLjM4NTIzIDEuMTI2MSAwLjg2OTI0IDEuNDcxOHYwLjAzOTVjLTAuNTkyNjcgMC4zMTYwOS0xLjEzNTkgMC44Mjk3
NC0xLjEzNTkgMS42MTAxIDAgMS4wMTc0IDAuOTA4NzYgMS43Mjg2IDIuMTMzNiAxLjcyODZ6bTAuNDE0ODctMy42NjQ2Yy0wLjcw
MTMyLTAuMjc2NTgtMS4yNjQ0LTAuNTUzMTYtMS4yNjQ0LTEuMTg1MyAwLTAuNTQzMjggMC4zNzUzNi0wLjg1OTM2IDAuODU5Mzct
MC44NTkzNiAwLjU4Mjc5IDAgMC45Mjg1MSAwLjQwNDk4IDAuOTI4NTEgMC45NTgxNCAwIDAuMzk1MTEtMC4xODc2OCAwLjc1MDcx
LTAuNTIzNTIgMS4wODY2em0tMC4zOTUxMSAyLjg1NDdjLTAuNjQyMDYgMC0xLjE1NTctMC40MTQ4Ni0xLjE1NTctMS4wMzcyIDAt
MC40ODQwMSAwLjI4NjQ2LTAuODg5IDAuNjgxNTctMS4xNzU1IDAuODQ5NDkgMC4zNDU3MiAxLjUxMTMgMC41OTI2NyAxLjUxMTMg
MS4yNzQyIDAgMC41ODI3OS0wLjQzNDYyIDAuOTM4MzktMS4wMzcyIDAuOTM4Mzl6IiBmaWxsPSIjMDAxMDMwIiBmaWxsLW9wYWNp
dHk9Ii45NDExOCIgc3Ryb2tlPSJub25lIi8+CiAgIDwvZz4KICAgPGcgdHJhbnNmb3JtPSJyb3RhdGUoNi41MDA4KSIgYXJpYS1s
YWJlbD0iMyI+CiAgICA8cGF0aCBkPSJtMzkuMzAyIDI4My42NGMxLjAzMjkgMCAxLjg4ODEtMC41NzU3MyAxLjg4ODEtMS41NTc5
IDAtMC43MTEyLTAuNDgyNi0xLjE2ODQtMS4xMTc2LTEuMzM3N3YtMC4wMzM5YzAuNTg0Mi0wLjIyODYgMC45Mzk4LTAuNjM1IDAu
OTM5OC0xLjIzNjEgMC0wLjkxNDQtMC43MTEyLTEuNDIyNC0xLjc0NDEtMS40MjI0LTAuNjQzNDcgMC0xLjE1OTkgMC4yNzA5My0x
LjYxNzEgMC42NzczM2wwLjQ5OTUzIDAuNjAxMTRjMC4zMzAyLTAuMzA0OCAwLjY2MDQtMC41MDggMS4wODM3LTAuNTA4IDAuNDkx
MDcgMCAwLjc5NTg3IDAuMjcwOTMgMC43OTU4NyAwLjcxOTY3IDAgMC40OTk1My0wLjM0NzEzIDAuODYzNi0xLjQwNTUgMC44NjM2
djAuNzExMmMxLjIyNzcgMCAxLjU4MzMgMC4zNTU2IDEuNTgzMyAwLjkxNDQgMCAwLjUwOC0wLjQwNjQgMC44MTI4LTAuOTkwNiAw
LjgxMjgtMC41NDE4NyAwLTAuOTU2NzMtMC4yNjI0Ny0xLjI3ODUtMC41OTI2N2wtMC40NjU2NyAwLjYyNjUzYzAuMzgxIDAuNDIz
MzQgMC45NTY3MyAwLjc2MiAxLjgyODggMC43NjJ6IiBmaWxsPSIjMDAxMDMwIiBmaWxsLW9wYWNpdHk9Ii44Nzg0MyIgc3Ryb2tl
PSJub25lIi8+CiAgIDwvZz4KICAgPGcgdHJhbnNmb3JtPSJyb3RhdGUoOC4zNTYpIiBhcmlhLWxhYmVsPSIxIj4KICAgIDxwYXRo
IGQ9Im00Ni40MDUgMjY4LjloMy4yNDI3di0wLjc5NTg3aC0xLjA1ODN2LTQuNTg4OWgtMC43MjgxM2MtMC4zMzg2NyAwLjIwMzIt
MC43MTEyIDAuMzM4NjYtMS4yNDQ2IDAuNDQwMjZ2MC42MDk2aDAuOTkwNnYzLjUzOTFoLTEuMjAyM3oiIGZpbGw9IiMwMDEwMzAi
IGZpbGwtb3BhY2l0eT0iLjc1Mjk0IiBzdHJva2U9Im5vbmUiLz4KICAgPC9nPgogICA8ZyB0cmFuc2Zvcm09InJvdGF0ZSgxMi44
NjEpIiBhcmlhLWxhYmVsPSI4Ij4KICAgIDxwYXRoIGQ9Im04NS4wMzYgMjYxLjYzYzEuMDkyMiAwIDEuODIwMy0wLjYzNSAxLjgy
MDMtMS40NjQ3IDAtMC43MTk2Ny0wLjQzMTgtMS4xMTc2LTAuOTMxMzMtMS4zOTd2LTAuMDMzOWMwLjM0NzEzLTAuMjU0IDAuNzE5
NjctMC43MDI3MyAwLjcxOTY3LTEuMjM2MSAwLTAuODcyMDctMC42MTgwNy0xLjQ1NjMtMS41ODMzLTEuNDU2My0wLjkzMTMzIDAt
MS42MjU2IDAuNTU4OC0xLjYyNTYgMS40MzA5IDAgMC41NjcyNiAwLjMzMDIgMC45NjUyIDAuNzQ1MDcgMS4yNjE1djAuMDMzOWMt
MC41MDggMC4yNzA5My0wLjk3MzY3IDAuNzExMi0wLjk3MzY3IDEuMzgwMSAwIDAuODcyMDcgMC43Nzg5MyAxLjQ4MTcgMS44Mjg4
IDEuNDgxN3ptMC4zNTU2LTMuMTQxMWMtMC42MDExMy0wLjIzNzA3LTEuMDgzNy0wLjQ3NDE0LTEuMDgzNy0xLjAxNiAwLTAuNDY1
NjcgMC4zMjE3My0wLjczNjYgMC43MzY2LTAuNzM2NiAwLjQ5OTUzIDAgMC43OTU4NyAwLjM0NzEzIDAuNzk1ODcgMC44MjEyNiAw
IDAuMzM4NjctMC4xNjA4NyAwLjY0MzQ3LTAuNDQ4NzMgMC45MzEzNHptLTAuMzM4NjcgMi40NDY5Yy0wLjU1MDMzIDAtMC45OTA2
LTAuMzU1Ni0wLjk5MDYtMC44ODkgMC0wLjQxNDg2IDAuMjQ1NTMtMC43NjIgMC41ODQyLTEuMDA3NSAwLjcyODEzIDAuMjk2MzMg
MS4yOTU0IDAuNTA4IDEuMjk1NCAxLjA5MjIgMCAwLjQ5OTUzLTAuMzcyNTMgMC44MDQzMy0wLjg4OSAwLjgwNDMzeiIgZmlsbD0i
IzAwMTAzMCIgZmlsbC1vcGFjaXR5PSIuNjI3NDUiIHN0cm9rZT0ibm9uZSIvPgogICA8L2c+CiAgIDxnIHRyYW5zZm9ybT0icm90
YXRlKDQuMzA5OSkiIGFyaWEtbGFiZWw9IjUiPgogICAgPHBhdGggZD0ibTQ2LjM0MSAyODkuNDljMC45OTA2IDAgMS44OTY1LTAu
Njc3MzQgMS44OTY1LTEuODU0MiAwLTEuMTU5OS0wLjc3MDQ3LTEuNjg0OS0xLjY5MzMtMS42ODQ5LTAuMjc5NCAwLTAuNDgyNiAw
LjA2NzctMC43MTEyIDAuMTc3OGwwLjExMDA3LTEuMzAzOWgyLjAzMnYtMC44MjEyN2gtMi44Nzg3bC0wLjE2MDg3IDIuNjU4NSAw
LjQ2NTY3IDAuMjk2MzNjMC4zMjE3My0wLjIxMTY3IDAuNTE2NDctMC4zMDQ4IDAuODYzNi0wLjMwNDggMC41OTI2NyAwIDAuOTkw
NiAwLjM2NDA3IDAuOTkwNiAxLjAwNzUgMCAwLjY1MTk0LTAuNDQwMjcgMS4wMzI5LTEuMDQxNCAxLjAzMjktMC41NDE4NyAwLTAu
OTM5OC0wLjI3MDk0LTEuMjYxNS0wLjU3NTc0bC0wLjQ0ODczIDAuNjI2NTRjMC4zOTc5MyAwLjM5NzkzIDAuOTY1MiAwLjc0NTA3
IDEuODM3MyAwLjc0NTA3eiIgZmlsbD0iIzAwMTAzMCIgZmlsbC1vcGFjaXR5PSIuNTAxOTYiIHN0cm9rZT0ibm9uZSIvPgogICA8
L2c+CiAgPC9nPgogIDxnIGZpbGw9IiMwMDEwMzAiIGZpbGwtb3BhY2l0eT0iLjM3NjQ3IiBzdHJva2U9IiMwMDAiIHN0cm9rZS13
aWR0aD0iLjI2NDU4cHgiIGFyaWEtbGFiZWw9IjMiPgogICA8cGF0aCBkPSJtOS44ODU5IDI5My40NmMxLjAzMjkgMCAxLjg4ODEt
MC41NzU3NCAxLjg4ODEtMS41NTc5IDAtMC43MTEyLTAuNDgyNi0xLjE2ODQtMS4xMTc2LTEuMzM3N3YtMC4wMzM5YzAuNTg0Mi0w
LjIyODYgMC45Mzk4LTAuNjM1IDAuOTM5OC0xLjIzNjEgMC0wLjkxNDQtMC43MTEyLTEuNDIyNC0xLjc0NDEtMS40MjI0LTAuNjQz
NDcgMC0xLjE1OTkgMC4yNzA5My0xLjYxNzEgMC42NzczM2wwLjQ5OTUzIDAuNjAxMTNjMC4zMzAyLTAuMzA0OCAwLjY2MDQtMC41
MDggMS4wODM3LTAuNTA4IDAuNDkxMDcgMCAwLjc5NTg3IDAuMjcwOTQgMC43OTU4NyAwLjcxOTY3IDAgMC40OTk1My0wLjM0NzEz
IDAuODYzNi0xLjQwNTUgMC44NjM2djAuNzExMmMxLjIyNzcgMCAxLjU4MzMgMC4zNTU2IDEuNTgzMyAwLjkxNDQgMCAwLjUwOC0w
LjQwNjQgMC44MTI4LTAuOTkwNiAwLjgxMjgtMC41NDE4NyAwLTAuOTU2NzMtMC4yNjI0Ny0xLjI3ODUtMC41OTI2N2wtMC40NjU2
NyAwLjYyNjU0YzAuMzgxIDAuNDIzMzMgMC45NTY3MyAwLjc2MiAxLjgyODggMC43NjJ6IiBmaWxsPSIjMDAxMDMwIiBmaWxsLW9w
YWNpdHk9Ii4zNzY0NyIgc3Ryb2tlPSJub25lIi8+CiAgPC9nPgogIDxnIGZpbGw9Im5vbmUiIHN0cm9rZT0iIzAwMCIgc3Ryb2tl
LXdpZHRoPSIuMjY0NThweCI+CiAgIDxnIHRyYW5zZm9ybT0icm90YXRlKC0xMS4zNTIpIiBhcmlhLWxhYmVsPSIwIj4KICAgIDxw
YXRoIGQ9Im0tNTEuNDcxIDI3Ni4xN2MxLjExNzYgMCAxLjgyODgtMC45OTkwNyAxLjgyODgtMi44MTk0IDAtMS44MTE5LTAuNzEx
Mi0yLjc2ODYtMS44Mjg4LTIuNzY4NnMtMS44Mjg4IDAuOTQ4MjctMS44Mjg4IDIuNzY4NiAwLjcxMTIgMi44MTk0IDEuODI4OCAy
LjgxOTR6bTAtMC43NjJjLTAuNTE2NDcgMC0wLjg5NzQ3LTAuNTMzNC0wLjg5NzQ3LTIuMDU3NHMwLjM4MS0yLjAwNjYgMC44OTc0
Ny0yLjAwNjZjMC41MjQ5MyAwIDAuODk3NDcgMC40ODI2IDAuODk3NDcgMi4wMDY2cy0wLjM3MjUzIDIuMDU3NC0wLjg5NzQ3IDIu
MDU3NHoiIGZpbGw9IiMwMDEwMzAiIGZpbGwtb3BhY2l0eT0iLjI1MDk4IiBzdHJva2U9Im5vbmUiLz4KICAgPC9nPgogICA8ZyB0
cmFuc2Zvcm09InJvdGF0ZSgyMi41MDYpIiBhcmlhLWxhYmVsPSI3Ij4KICAgIDxwYXRoIGQ9Im0xMTguMjEgMjQzLjA4aDAuOTkw
NmMwLjA5MzEtMi4wOTk3IDAuMzEzMjctMy4yMjU4IDEuNTc0OC00Ljc5MjF2LTAuNTkyNjdoLTMuNjE1M3YwLjgyMTI3aDIuNTQ4
NWMtMS4wNDE0IDEuNDQ3OC0xLjQwNTUgMi42NTAxLTEuNDk4NiA0LjU2MzV6IiBmaWxsPSIjMDAxMDMwIiBmaWxsLW9wYWNpdHk9
Ii4xMjU0OSIgc3Ryb2tlPSJub25lIi8+CiAgIDwvZz4KICAgPGcgdHJhbnNmb3JtPSJyb3RhdGUoLTkuNzI3MykiIGFyaWEtbGFi
ZWw9IjEiPgogICAgPHBhdGggZD0ibS0xOC4yOTkgMjgyLjc5aDMuMjQyN3YtMC43OTU4N2gtMS4wNTgzdi00LjU4ODloLTAuNzI4
MTNjLTAuMzM4NjcgMC4yMDMyLTAuNzExMiAwLjMzODY3LTEuMjQ0NiAwLjQ0MDI3djAuNjA5NmgwLjk5MDZ2My41MzkxaC0xLjIw
MjN6IiBmaWxsPSIjMDAxMDMwIiBmaWxsLW9wYWNpdHk9Ii4wNjI3NDUiIHN0cm9rZT0ibm9uZSIvPgogICA8L2c+CiAgPC9nPgog
IDxnIGZpbGw9IiMyNDEwMzAiIGFyaWEtbGFiZWw9Ii4iPgogICA8cGF0aCBkPSJtMjAuOTIgMjgzLjk4YzAuNjU0NzYgMCAxLjEy
ODktMC41MTkyOSAxLjEyODktMS4xNzQgMC0wLjY1NDc2LTAuNDc0MTMtMS4xNzQtMS4xMjg5LTEuMTc0LTAuNjU0NzYgMC0xLjEy
ODkgMC41MTkyOS0xLjEyODkgMS4xNzQgMCAwLjY1NDc1IDAuNDc0MTMgMS4xNzQgMS4xMjg5IDEuMTc0eiIgZmlsbD0iIzAwMTAz
MCIvPgogIDwvZz4KIDwvZz4KPC9zdmc+Cg==
")]
#![doc(test(attr(deny(warnings))))]
#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]
#![cfg_attr(unsafe_in_unsafe, warn(unsafe_op_in_unsafe_fn))]
#![cfg_attr(not(unsafe_in_unsafe), allow(unused_unsafe))]
#![allow(
    clippy::missing_safety_doc,
    clippy::unnecessary_cast,
    clippy::useless_conversion
)]

pub mod gmp;
#[cfg(feature = "mpc")]
pub mod mpc;
#[cfg(feature = "mpfr")]
pub mod mpfr;

#[cfg(extended_key_value_attributes)]
pub mod C;

#[cfg(test)]
mod tests {
    use core::{slice, str};
    use libc::c_char;

    pub unsafe fn str_from_cstr<'a>(cstr: *const c_char) -> &'a str {
        let s = unsafe { slice::from_raw_parts(cstr as *const u8, libc::strlen(cstr)) };
        str::from_utf8(s).expect("version not utf8")
    }
}
