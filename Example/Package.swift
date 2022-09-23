// swift-tools-version: 5.5.0

// Example/Package.swift
import PackageDescription

let package = Package(
    name: "Example",
    dependencies: [
        .package(path: "../XuexiPackage")
    ],
    targets: [
        .executableTarget(
            name: "Example",
            dependencies: [
                .product(name: "XuexiPackage", package: "XuexiPackage")
            ]
        )
    ]
)