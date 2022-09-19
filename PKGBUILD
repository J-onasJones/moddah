pkgname="moddah"
pkgver="0.0.1"
pkgdesc="A modloader manager for QuiltMC, FabricMC and ForgeModLoader."
arch=(“x86_x64” “arm”)
depends=("curl")
makedepends=("cargo")
license=("Right to modify")

prepare() {
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all-features
}

check() {
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

package() {
    install -Dm0755 -t "$pkgdir/usr/bin/" "target/release/$pkgname"
}