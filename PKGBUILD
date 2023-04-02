# Maintainer: Fabio Lenherr <dashie@dashie.org>

pkgname=HyprCalc
pkgver=0.2
pkgrel=1
arch=('x86_64')
pkgdir="/usr/bin/${pkgname}"
pkgdesc="A Calculator written with rust and gtk4"
depends=('rust' 'gtk4')
build() {
	cargo build --release
}

package() {
	cd ..
	install -Dm755 target/release/hyprcalc "$pkgdir"/usr/bin/hyprcalc
}
