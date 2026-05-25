import re

with open('crates/kernel_types/src/lib.rs', 'r') as f:
    content = f.read()

content = content.replace("pub _rcuhead: *mut c_void,", "pub rcuhead: *mut c_void,")
content = content.replace("/// FFI boundary for KUnit", "/// FFI boundary for `KUnit`")

with open('crates/kernel_types/src/lib.rs', 'w') as f:
    f.write(content)
