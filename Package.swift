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
            url: "https://github.com/BlocksHub/Blocksmulti/releases/download/v0.1.3/BlocksmultiFFI.xcframework.zip",
            checksum: "1529cc06be28d3500f806a4c3b793920d45730ed515db2defd3df7f25cc28512"
        ),
        .target(name: "Blocksmulti", dependencies: ["BlocksmultiFFI"])
    ]
)