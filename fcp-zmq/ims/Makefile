.PHONY: build install build-install

init:
	mkdir -p build
	cd build; \
	cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=1 -DCMAKE_BUILD_TYPE=Debug ..; \

init-usr:
	mkdir -p build
	cd ./build; \
	cmake -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_EXPORT_COMPILE_COMMANDS=1 -DCMAKE_BUILD_TYPE=Debug ..

build:
	cmake --build ./build

install:
	sudo cmake --install ./build

clean:
	rm -rf build