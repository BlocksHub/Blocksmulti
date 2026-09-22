// swift-tools-version: 6.4
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Blockmulti",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "Blockmulti", targets: ["Blockmulti"])
    ],
    targets: [
        .binaryTarget(
            name: "BlockmultiFFI",
            url: "https://github.com/BlocksHub/Blocksmulti/releases/download/v0.1.1/BlockmultiFFI.xcframework.zip",
            checksum: "3d85a6fb470ac47d2ac23dd244711c43e620cc7b414ee20b04fa6e1ced201ebe"
        ),
        .target(name: "Blockmulti", dependencies: ["BlockmultiFFI"])
    ]
)