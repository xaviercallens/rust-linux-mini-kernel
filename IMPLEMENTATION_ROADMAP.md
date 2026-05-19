# Implementation Roadmap for Remaining Compilation Errors

## Current Status
- **117/125 packages compiling (93.6%)**
- **8 packages remaining**
- **163 errors total**

---

## Priority 1: Core Type Additions to kernel_types (Highest Impact)

### 1.1 Add Missing sk_buff Fields
**Impact:** Fixes ip6_checksum, fou6, and potentially others

```rust
// In crates/kernel_types/src/lib.rs, update sk_buff:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sk_buff {
    pub next: *mut sk_buff,
    pub prev: *mut sk_buff,
    pub tstamp: __u64,
    pub dev: *mut c_void,
    pub len: c_uint,
    pub data_len: c_uint,
    pub mac_len: __u16,
    pub hdr_len: __u16,
    pub csum: __u32,
    pub priority: __u32,
    pub protocol: __be16,
    pub cb: [__u8; 48],
    pub ip_summed: __u8,
    pub csum_level: __u8,
    pub csum_valid: __u8,
    pub csum_complete_sw: __u8,
    
    // ADD THESE CRITICAL FIELDS:
    pub data: *mut __u8,              // ← Add
    pub tail: __u16,                  // ← Add (offset from head)
    pub end: __u16,                   // ← Add (offset from head)
    pub head: *mut __u8,              // Already exists
    pub network_header: __u16,        // Already exists
    pub transport_header: __u16,      // Already exists
    pub transport_offset: c_int,      // Already exists
    pub network_header_len: c_uint,   // Already exists
    
    pub sk: *mut c_void,              // Already exists
    pub dst: *mut c_void,             // Already exists
}
```

### 1.2 Add msghdr and iovec
**Impact:** Fixes ip6_checksum import errors

```rust
// Add to kernel_types/src/lib.rs:

/// I/O vector for scatter-gather operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

/// Socket message header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msghdr {
    pub msg_name: *mut c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut c_void,
    pub msg_controllen: size_t,
    pub msg_flags: c_int,
}
```

### 1.3 Add list_head Type Alias
**Impact:** Fixes nf_conntrack_helper

```rust
// Add to kernel_types/src/lib.rs:

/// Linux kernel intrusive linked list node (alias for consistency)
pub type list_head = ListHead;

/// Hash list head (used in hash tables)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}
```

### 1.4 Add Crypto API Types
**Impact:** Fixes esp4 type errors

```rust
// Add to kernel_types/src/lib.rs:

/// Opaque crypto AEAD (Authenticated Encryption with Associated Data) type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_aead {
    _private: [u8; 0],
}

/// Opaque crypto transform type
#[repr(C)]
#[derive(Copy, Clone)]
pub struct crypto_tfm {
    _private: [u8; 0],
}

/// AEAD request structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aead_request {
    _private: [u8; 0],
}

/// GFP allocation flags type
pub type gfp_t = c_uint;

/// Common GFP flags
pub const GFP_KERNEL: gfp_t = 0xCC0;
pub const GFP_ATOMIC: gfp_t = 0x20;
```

### 1.5 Add Kernel Allocator Declarations
**Impact:** Fixes nf_conntrack_h323_main

```rust
// Add to kernel_types/src/lib.rs or as extern block:

extern "C" {
    /// Allocate kernel memory
    pub fn kmalloc(size: usize, flags: gfp_t) -> *mut c_void;
    
    /// Free kernel memory
    pub fn kfree(ptr: *mut c_void);
    
    /// Allocate and zero-initialize kernel memory
    pub fn kzalloc(size: usize, flags: gfp_t) -> *mut c_void;
    
    /// Allocate from a specific cache
    pub fn kmem_cache_alloc(cache: *mut c_void, flags: gfp_t) -> *mut c_void;
    
    /// Free to a specific cache
    pub fn kmem_cache_free(cache: *mut c_void, ptr: *mut c_void);
}
```

---

## Priority 2: Fix Protocol Array (fou6)

### 2.1 Change inet6_protos from Function to Static Array

```rust
// In crates/fou6/src/lib.rs or wherever it's declared:

// WRONG (current):
extern "C" {
    fn inet6_protos() -> [*const inet6_protocol; 256];
}

// CORRECT:
extern "C" {
    static inet6_protos: [*const inet6_protocol; 256];
}

// Usage changes from:
let handler = inet6_protos()[protocol];

// To:
let handler = inet6_protos[protocol];
```

---

## Priority 3: Remove Remaining Duplicates

### 3.1 UDP Package Duplicates

```bash
# In crates/udp/src/lib.rs, find and remove duplicates:
- sock_net (keep extern declaration)
- net_eq (keep extern declaration)  
- ipv6_addr_equal (keep extern declaration)
- ipv6_addr_any (keep extern declaration)
- udp_sk_bound_dev_eq (keep extern declaration)
```

### 3.2 ESP4 Package Duplicates

```bash
# In crates/esp4/src/lib.rs, find and remove duplicate definitions:
- crypto_aead_ivsize
- crypto_aead_alignmask
- crypto_tfm_ctx_alignment
- crypto_aead_reqsize
- aead_request_set_tfm

# Keep only extern declarations or move to kernel_types
```

---

## Priority 4: Fix IP Header Access (fou6)

### 4.1 Add version Field Access Helper

```rust
// The error "no field `version` on type `&*const kernel_types::iphdr`"
// suggests direct field access. Need to use bit manipulation:

#[inline]
pub unsafe fn ip_hdr_version(iph: *const iphdr) -> u8 {
    ((*iph).version >> 4) & 0x0F
}

// Or add version field to iphdr in kernel_types if it's missing
```

---

## Priority 5: Fix Documentation Comments (nf_conntrack_amanda)

### 5.1 Fix Doc Comment Syntax

```rust
// WRONG (causes E0753):
fn my_function() {
    //! This is wrong placement
}

// CORRECT:
/// This is correct placement
fn my_function() {
}
```

---

## Priority 6: Fix Missing Variables (nf_conntrack_amanda)

### 6.1 Add Missing Parameters

```rust
// Current function likely missing parameters:
pub unsafe extern "C" fn nf_conntrack_amanda_help(
    skb: *mut sk_buff,
    // ADD THESE:
    protoff: c_uint,        // ← Add
    ct: *mut nf_conn,       // ← Add
    ctinfo: c_uint,         // ← Add
) -> c_int {
    // Now dataoff can be calculated:
    let dataoff = protoff + 8; // Example calculation
    // ...
}
```

---

## Priority 7: Fix ip6_fib Type Mismatches

### 7.1 Check Return Type Conversions

```rust
// Likely issue: returning wrong type from function
// Check function signatures match between declaration and implementation

// Example fix pattern:
// If function returns *mut fib6_table but you're returning *mut c_void:
pub unsafe extern "C" fn fib6_get_table(net: *mut net, id: u32) -> *mut fib6_table {
    // Ensure cast is correct:
    let table = /* ... */;
    table as *mut fib6_table  // Not as *mut c_void
}
```

---

## Implementation Order

### Week 1: Core Types (Highest ROI)
1. Add sk_buff fields (data, tail, end)
2. Add msghdr and iovec
3. Add list_head alias
4. **Expected result:** 3-4 packages fixed

### Week 2: Crypto and Allocators
1. Add crypto API types
2. Add kernel allocator externs
3. Remove crypto duplicates in esp4
4. **Expected result:** 2 packages fixed

### Week 3: Protocol Arrays and Cleanup
1. Fix inet6_protos static array
2. Remove UDP duplicates
3. Fix documentation comments
4. **Expected result:** 2-3 packages fixed

### Week 4: Edge Cases
1. Fix ip6_fib type mismatches
2. Fix nf_conntrack_amanda parameters
3. Fix IP header version access
4. **Expected result:** Remaining packages fixed

---

## Testing Strategy

After each change:
```bash
# Check specific package
cargo check --package <package-name>

# Check workspace-wide impact
cargo check --workspace 2>&1 | grep "^error\[" | wc -l

# Count successful compilations
cargo build --workspace 2>&1 | grep "Finished" -c
```

---

## Success Metrics

- **Target:** 125/125 packages compiling (100%)
- **Current:** 117/125 (93.6%)
- **Remaining:** 8 packages
- **Estimated effort:** 4-6 weeks of focused work
- **Key milestone:** Hitting 120/125 (96%) after core type additions

---

## Risk Factors

1. **ABI Compatibility:** Must match Linux 5.10 LTS exactly
2. **Padding/Alignment:** C struct layout must be preserved
3. **Pointer Lifetimes:** Rust can't express kernel's pointer semantics
4. **Cascade Effects:** Each fix may expose 5-10 new errors

---

## Long-term Maintenance

1. **Kernel Version Tracking:** Pin to specific kernel version
2. **Testing Infrastructure:** Need kernel module test harness
3. **Documentation:** Each type needs safety documentation
4. **Upstream Coordination:** Consider rust-for-linux project alignment
