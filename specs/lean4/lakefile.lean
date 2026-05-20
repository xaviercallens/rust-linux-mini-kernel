import Lake
open Lake DSL

package mvk_specs where
  -- MVK v9.0.0 Formal Specifications
  -- Phase 1: Boot Subsystem (printk, arch_setup, init_main)
  -- Phase 2: Memory Subsystem (page_alloc, slab)
  precompileModules := true

@[default_target]
lean_lib MVK where
  roots := #[`MVK]
  globs := #[.submodules `MVK]
