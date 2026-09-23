// swift-tools-version: 6.4
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Blocksmulti",
    platforms: [.iOS(.v13)],
    products: [
        .library(name: "Blocksmulti", targets: ["Blocksmulti"])
    ],
    targets: [
        .binaryTarget(
            name: "BlocksmultiFFI",
            url: "PLACEHOLDER_URL",
            checksum: "PLACEHOLDER_CHECKSUM"
        ),
        .target(name: "Blocksmulti", dependencies: ["BlocksmultiFFI"])
    ]
)