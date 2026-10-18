PLATFORM=$(shell uname -o)
ifeq ($(PLATFORM),Darwin)
	EXT=dylib
else
	EXT=so
endif

ARCH=$(shell uname -m)

release:
	cargo build --release --target $(target)
	cp target/${target}/release/libvtubetrackers_gd.$(EXT) addons/vtube_trackers/lib/libvtubetrackers_gd.release.$(ARCH).$(EXT)

package: release
