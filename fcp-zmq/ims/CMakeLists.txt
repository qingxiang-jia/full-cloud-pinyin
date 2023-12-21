cmake_minimum_required(VERSION 3.20)
project(ims VERSION 0.0.1)

set(CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 17)
# Next three: https://github.com/protocolbuffers/protobuf/issues/14500
set(CMAKE_MODULE_LINKER_FLAGS "-Wl,--copy-dt-needed-entries")
set(CMAKE_SHARED_LINKER_FLAGS "-Wl,--copy-dt-needed-entries")
set(CMAKE_STATIC_LINKER_FLAGS "-Wl,--copy-dt-needed-entries")

find_package(Fcitx5Core REQUIRED)
find_package(cppzmq REQUIRED)
find_package(Protobuf REQUIRED)

# Setup some compiler option that is generally useful and compatible with Fcitx 5 (C++17)
include("${FCITX_INSTALL_CMAKECONFIG_DIR}/Fcitx5Utils/Fcitx5CompilerSettings.cmake")

add_library(ims SHARED ims.cpp msgs.pb.cc)
target_link_libraries(ims PRIVATE Fcitx5::Core cppzmq protobuf::libprotobuf)

install(TARGETS ims DESTINATION "${FCITX_INSTALL_LIBDIR}/fcitx5")

# Addon config file
install(FILES "${PROJECT_SOURCE_DIR}/ims-addon.conf" RENAME ims.conf DESTINATION "${FCITX_INSTALL_PKGDATADIR}/addon")

# Input Method registration file
install(FILES "${PROJECT_SOURCE_DIR}/ims.conf" DESTINATION "${FCITX_INSTALL_PKGDATADIR}/inputmethod")