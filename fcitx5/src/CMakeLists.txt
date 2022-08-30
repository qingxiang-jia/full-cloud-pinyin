# Make sure it produce quwei.so instead of libquwei.so
add_library(quwei SHARED quwei.cpp)
target_link_libraries(quwei PRIVATE Fcitx5::Core Fcitx5::Module::Punctuation  Fcitx5::Module::QuickPhrase)
install(TARGETS quwei DESTINATION "${FCITX_INSTALL_LIBDIR}/fcitx5")

# Addon config file
# We need additional layer of conversion because we want PROJECT_VERSION in it.
configure_file(quwei-addon.conf.in.in quwei-addon.conf.in)
fcitx5_translate_desktop_file("${CMAKE_CURRENT_BINARY_DIR}/quwei-addon.conf.in" quwei-addon.conf)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/quwei-addon.conf" RENAME quwei.conf DESTINATION "${FCITX_INSTALL_PKGDATADIR}/addon")

# Input Method registration file
fcitx5_translate_desktop_file(quwei.conf.in quwei.conf)
install(FILES "${CMAKE_CURRENT_BINARY_DIR}/quwei.conf" DESTINATION "${FCITX_INSTALL_PKGDATADIR}/inputmethod")
