#!/usr/bin/env nu

const bridge_path = '../fcitx5-bridge'

def "main -h" [] {
    show_help
}

def "main --help" [] {
    show_help
}

def "main help" [] {
    show_help
}

def "main release" [type: string] {
    if $type != 'fcp' and $type != 'fcn' {
        print "Invalid release type"
        exit 1
    } else if check_assumption == false {
        exit 1
    }
    build_fcitx5_bridge # Currently, the build type is controller by editting CMakeLists.txt.
    if $type == 'fcp' {
        build_fcp
    } else if $type == 'fcn' {
        build_fcn
    }
    pack
}

def main [] {
    show_help
}

def show_help [] {
    print 'This script helps to build full-cloud-pinyin and fcitx5-bridge and release them.'
    print 'Assumption: this script sits inside the root folder of full-cloud-pinyin and fcitx5-bridge is at the same level of full-cloud-pinyin. It also assumes you have all dependencies ready for build both projects.'
    print ''
    print '-h, -help, help: to see this message again'
    print 'release fcp: to build both projects and generate files for full-cloud-pinyin to install.'
    print 'release fcn: to build both projects and generate files for full-cloud-nepali to install.'
}

def check_assumption [] {
    if ($bridge_path | path exists) {
        true
    }
    print 'Cannot find fcitx5-bridge.'
    false
}

def build_fcitx5_bridge [] {
    let main_dir = pwd
    cd $bridge_path
    make clean
    make init-release-local
    make build
    make install-local
}

def build_fcp [] {
    cargo build --release --features "fcp"
}

def build_fcn [] {
    cargo build --release --features "fcn"
}

def pack [] {
    mv -v $'($bridge_path)/build/binary' .
    mv -v binary to-pack
    mv -v ./target/release/fcp ./to-pack/
}
