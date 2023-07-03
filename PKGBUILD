# Maintainer: Kirill Bezuglyi <fidelicura@gmail.com>
pkgname=pacfo
pkgver=1
pkgrel=1
pkgdesc="Prettify your pacman package info with user-friendly and lightweight CLI!"
arch=('x86_64')
url="https://github.com/fidelicura/pacfo"
license=('AGPL-3.0')
depends=('libc')
makedepends=('rust')
source=('pacfo::git://github.com/fidelicura/pacfo.git')
md5sums=('SKIP')

build() {
	cd "$pkgname"
	make
}

package() {
	cd "$pkgname"
  install -Dm755 ./release/pacfo "$pkgdir/usr/bin/pacfo"
  install -Dm644 ./README.md "$pkgdir/usr/share/doc/$pkgname"
}
