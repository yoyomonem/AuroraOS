# AuroraOS (REWORKED)

This is a WIP operating system made with Rust unlike the original which used assembly.

## File tree (diff)

The below is a result of a diff comparison between [yoyomonem/AuroraOS](https://github.com/yoyomonem/AuroraOS) and [CleanCode-developer/AuroraOS](https://github.com/CleanCode-developer/AuroraOS).

```diff
- bootloader.asm
+ .cargo/
+  └─ config.toml
+ src/
+  └─ main.rs
+ Cargo.toml
+ Cargo.lock
+ .gitignore
+ README.md
+ x86_64-aurora-os.json
```
