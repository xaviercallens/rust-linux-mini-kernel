## Description
<!-- Please include a summary of the change and which module/issue is fixed. Please also include relevant motivation and context. -->

Fixes # (issue number)

## Type of Change
<!-- Please delete options that are not relevant. -->
- [ ] Bug fix (non-breaking change which fixes a compilation error or logic bug)
- [ ] New feature (new module translation or pipeline addition)
- [ ] Breaking change (FFI modification that breaks existing dependencies)
- [ ] Documentation update (module READMEs, kernel integration guides)

## How Has This Been Tested?
<!-- Please describe the tests that you ran to verify your changes. Provide instructions so we can reproduce. -->
- [ ] `cargo check -p [module]` passes without errors
- [ ] `cargo check --workspace` passes (or does not regress)
- [ ] (Optional) Loaded into a QEMU Linux Kernel (`KERNEL_INTEGRATION_GUIDE.md`)

## Checklist:
- [ ] My code follows the style guidelines of this project (`cargo fmt`)
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas (`// SAFETY:` for unsafe blocks)
- [ ] I have made corresponding changes to the documentation
- [ ] My changes generate no new warnings (`cargo clippy -- -D warnings`)
