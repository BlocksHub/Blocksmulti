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
            url: "https://github.com/BlocksHub/Blocksmulti/releases/download/v0.1.2/BlocksmultiFFI.xcframework.zip",
            checksum: "343fad7948e6a5fc2777b7ed01df0f01e1b5e1e55720baebed7ce0e9a5af790f"
        ),
        .target(name: "Blocksmulti", dependencies: ["BlocksmultiFFI"])
    ]
)