# Maintainer: Fabio Lenherr <dashie@dashie.org>

pkgname=oxicalc
pkgver=0.2
pkgrel=1
arch=('x86_64')
pkgdir="/usr/bin/${pkgname}"
pkgdesc="A small and simple calculator written with rust and gtk4"
depends=('rust' 'gtk4' 'gendesk')

build() {
	cargo build --release
}

package() {
	cd ..
	gendesk --pkgname "$pkgname" --pkgdesc "$pkgdesc" --name "OxiCalc" --categories "Utility;GTK;"
	install -Dm755 target/release/"$pkgname" "$pkgdir"/usr/bin/"$pkgname"
	install -Dm644 "$pkgname.desktop" "$pkgdir/usr/share/applications/$pkgname.desktop"
	install -Dm644 "$pkgname.svg" "$pkgdir/usr/share/pixmaps/$pkgname.svg"
}
