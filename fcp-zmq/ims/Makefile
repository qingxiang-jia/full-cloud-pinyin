.PHONY: build install

init-debug:
	mkdir -p build
	cd build; \
	cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=1 -DCMAKE_BUILD_TYPE=Debug ..; \

init-debug-usr:
	mkdir -p build
	cd ./build; \
	cmake -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_EXPORT_COMPILE_COMMANDS=1 -DCMAKE_BUILD_TYPE=Debug ..

init-release-usr:
	mkdir -p build
	cd ./build; \
	cmake -DCMAKE_INSTALL_PREFIX=/usr -DCMAKE_EXPORT_COMPILE_COMMANDS=1 -DCMAKE_BUILD_TYPE=Release ..

build:
	cmake --build ./build

install:
	sudo cmake --install ./build

proto:
	protoc --cpp_out=. msgs.proto

clean:
	rm -rf build