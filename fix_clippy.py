import re

with open('crates/kernel_types/src/lib.rs', 'r') as f:
    content = f.read()

# Fix docs backticks
content = content.replace("/// Socket buffer (packet buffer) - also aliased as sk_buff", "/// Socket buffer (packet buffer) - also aliased as `sk_buff`")
content = content.replace("/// XFRM (IPsec) mode skb callback", "/// XFRM (`IPsec`) mode skb callback")
content = content.replace("/// KUnit case structure for registering individual tests", "/// `KUnit` case structure for registering individual tests")
content = content.replace("/// KUnit suite structure representing a test suite", "/// `KUnit` suite structure representing a test suite")
content = content.replace("/// SafePageFrame is a zero-cost abstraction for a memory page frame.", "/// `SafePageFrame` is a zero-cost abstraction for a memory page frame.")

# Fix Safety docs
content = content.replace("pub unsafe extern \"C\" fn kunit_do_failed_assertion(", "/// # Safety\n/// FFI boundary for KUnit\npub unsafe extern \"C\" fn kunit_do_failed_assertion(")

# Fix if !message.is_null() to if message.is_null() else ...
old_if = """    if !message.is_null() {
        let mut len = 0;
        while *message.add(len) != 0 {
            len += 1;
        }
        let _ = write(2, message as *const c_void, len);
    } else {
        let null_str = b"null";
        let _ = write(2, null_str.as_ptr() as *const c_void, null_str.len());
    }"""
new_if = """    if message.is_null() {
        let null_str = b"null";
        let _ = write(2, null_str.as_ptr().cast::<c_void>(), null_str.len());
    } else {
        let mut len = 0;
        while *message.add(len) != 0 {
            len += 1;
        }
        let _ = write(2, message.cast::<c_void>(), len);
    }"""
content = content.replace(old_if, new_if)

# Fix ptr_as_ptr
content = content.replace("prefix.as_ptr() as *const c_void", "prefix.as_ptr().cast::<c_void>()")
content = content.replace("newline.as_ptr() as *const c_void", "newline.as_ptr().cast::<c_void>()")

# Fix lifetimes
content = content.replace("impl<'a> SafePageFrame<'a> {", "impl SafePageFrame<'_> {")

# Fix inline_always
content = content.replace("#[inline(always)]\n    pub fn new", "#[inline]\n    pub fn new")

# Fix pub underscore fields
content = content.replace("pub _metrics: [c_int; 17]", "pub metrics: [c_int; 17]")
content = content.replace("pub _mtu: c_ulong", "pub mtu: c_ulong")

with open('crates/kernel_types/src/lib.rs', 'w') as f:
    f.write(content)
