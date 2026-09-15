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
            url: "https://github.com/BlocksHub/Blocksmulti/releases/download/v0.1.0/BlockmultiFFI.xcframework.zip",
            checksum: "a64fc7a4e675ea22ac95e4150a848113bc93bee46477fb044a75bf0f153f3463"
        ),
        .target(name: "Blockmulti", dependencies: ["BlockmultiFFI"])
    ]
)