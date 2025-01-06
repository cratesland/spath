# CHANGELOG

All notable changes to this project will be documented in this file.

0.2.0 (2025-01-07)
==================

### Breaking changes

* Rework `Value` to three traits: `VariantValue`, `VariantArray`, and `VariantObject`. Now, we don't need to convert values between different semi-structure data implementation.

### New features

* Add a new feature flag `toml` for implementing `VariantValue` over toml's `Value`.
