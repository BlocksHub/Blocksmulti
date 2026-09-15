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
            url: "PLACEHOLDER_URL",
            checksum: "PLACEHOLDER_CHECKSUM"
        ),
        .target(name: "Blockmulti", dependencies: ["BlockmultiFFI"])
    ]
)