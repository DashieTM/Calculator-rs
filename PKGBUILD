# Maintainer: Fabio Lenherr <dashie@dashie.org>

pkgname=oxicalc
pkgver=0.2
pkgrel=1
arch=('x86_64')
pkgdir="/usr/bin/${pkgname}"
pkgdesc="A small and simple calculator written with rust and gtk4"
depends=('rust' 'gtk4')
build() {
	cargo build --release
}

package() {
	cd ..
	install -Dm755 target/release/"$pkgname" "$pkgdir"/usr/bin/"$pkgname"
}
