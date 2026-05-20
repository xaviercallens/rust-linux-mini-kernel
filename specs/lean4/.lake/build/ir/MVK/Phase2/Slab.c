// Lean compiler output
// Module: MVK.Phase2.Slab
// Imports: public import Init public import MVK.Phase2.Common public import MVK.Phase2.PageAlloc
#include <lean/lean.h>
#if defined(__clang__)
#pragma clang diagnostic ignored "-Wunused-parameter"
#pragma clang diagnostic ignored "-Wunused-label"
#elif defined(__GNUC__) && !defined(__CLANG__)
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-label"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#endif
#ifdef __cplusplus
extern "C" {
#endif
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MIN__SIZE;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MAX__SIZE;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_NUM__CACHES;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(4096) << 1) | 1)),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(2048) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(1024) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(512) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__2_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__3_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(256) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__3_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(128) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(64) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__6_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(32) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__6_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__7 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__7_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES___closed__7_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 31, .m_capacity = 31, .m_length = 30, .m_data = "MVK.Phase2.Common.Pointer.null"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__1_value;
lean_object* lean_nat_to_int(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 32, .m_capacity = 32, .m_length = 31, .m_data = "MVK.Phase2.Common.Pointer.valid"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__5_value),((lean_object*)(((size_t)(1) << 1) | 1))}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__6_value;
lean_object* l_Repr_addAppParen(lean_object*, lean_object*);
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___boxed(lean_object*, lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__0_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "next"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__2_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__3_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__3_value),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__6_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__8 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__8_value;
lean_object* lean_string_length(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__9_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__9;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__11 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__11_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__12_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__8_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__12 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__12_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__0___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__1(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__1___boxed(lean_object*, lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 10, .m_capacity = 10, .m_length = 9, .m_data = "free_list"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__2_value),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__3_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = ","};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__6_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 9, .m_capacity = 9, .m_length = 8, .m_data = "num_free"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__7 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__7_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__7_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__8 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__8_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__9_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__9;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 12, .m_capacity = 12, .m_length = 11, .m_data = "num_objects"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__10 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__10_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__10_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__11 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__11_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_Slab_instReprSlab___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprKmemCache_repr_spec__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprKmemCache_repr_spec__0___boxed(lean_object*, lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 12, .m_capacity = 12, .m_length = 11, .m_data = "object_size"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__2_value),((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__3_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 11, .m_capacity = 11, .m_length = 10, .m_data = "slab_order"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__5_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__6_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__6;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 10, .m_capacity = 10, .m_length = 9, .m_data = "slab_list"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__7 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__7_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__7_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__8 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__8_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 12, .m_capacity = 12, .m_length = 11, .m_data = "total_slabs"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__9 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__9_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__9_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__10 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__10_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 14, .m_capacity = 14, .m_length = 13, .m_data = "total_objects"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__11 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__11_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__12_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__11_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__12 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__12_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__13_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__13;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__14_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 18, .m_capacity = 18, .m_length = 17, .m_data = "allocated_objects"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__14 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__14_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__15_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__14_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__15 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__15_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__16_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__16;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__kmem__cache(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_cache__size__for__idx(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_cache__size__for__idx___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___lam__0(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___lam__0___boxed(lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___lam__0___boxed, .m_arity = 1, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__0_value;
extern lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__1;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__order__for__size(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__order__for__size___boxed(lean_object*);
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx___lam__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx___lam__0___boxed(lean_object*, lean_object*);
lean_object* l_List_findIdx_x3f___redArg(lean_object*, lean_object*);
uint8_t lean_nat_dec_lt(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx(lean_object*);
lean_object* lean_nat_pow(lean_object*, lean_object*);
lean_object* lean_nat_mul(lean_object*, lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
lean_object* lean_nat_div(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab___boxed(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___lam__0(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___lam__0___boxed(lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___lam__0___boxed, .m_arity = 1, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec(lean_object*, lean_object*);
uint8_t lp_mvk__specs_MVK_Phase2_Common_instDecidableEqPointer_decEq___redArg(lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kfree__spec___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kfree__spec___redArg___boxed(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kfree__spec(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kfree__spec___boxed(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kzalloc__spec(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kzalloc__spec___boxed(lean_object*, lean_object*, lean_object*);
lean_object* lean_int_neg(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__stat__spec___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__stat__spec___closed__0;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__stat__spec(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__exit__spec(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__exit__spec___boxed(lean_object*, lean_object*);
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MIN__SIZE(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_unsigned_to_nat(32u);
return x_1;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MAX__SIZE(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_unsigned_to_nat(8192u);
return x_1;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_NUM__CACHES(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_unsigned_to_nat(8u);
return x_1;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(2u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_10; uint8_t x_11; 
x_10 = lean_unsigned_to_nat(1024u);
x_11 = lean_nat_dec_le(x_10, x_2);
if (x_11 == 0)
{
lean_object* x_12; 
x_12 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2);
x_3 = x_12;
goto block_9;
}
else
{
lean_object* x_13; 
x_13 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_3 = x_13;
goto block_9;
}
}
else
{
lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_26; uint8_t x_27; 
x_14 = lean_ctor_get(x_1, 0);
lean_inc(x_14);
if (lean_is_exclusive(x_1)) {
 lean_ctor_release(x_1, 0);
 x_15 = x_1;
} else {
 lean_dec_ref(x_1);
 x_15 = lean_box(0);
}
x_26 = lean_unsigned_to_nat(1024u);
x_27 = lean_nat_dec_le(x_26, x_2);
if (x_27 == 0)
{
lean_object* x_28; 
x_28 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2);
x_16 = x_28;
goto block_25;
}
else
{
lean_object* x_29; 
x_29 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_16 = x_29;
goto block_25;
}
block_25:
{
lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; uint8_t x_22; lean_object* x_23; lean_object* x_24; 
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__6));
x_18 = l_Nat_reprFast(x_14);
if (lean_is_scalar(x_15)) {
 x_19 = lean_alloc_ctor(3, 1, 0);
} else {
 x_19 = x_15;
 lean_ctor_set_tag(x_19, 3);
}
lean_ctor_set(x_19, 0, x_18);
x_20 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_20, 0, x_17);
lean_ctor_set(x_20, 1, x_19);
x_21 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_21, 0, x_16);
lean_ctor_set(x_21, 1, x_20);
x_22 = 0;
x_23 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_23, 0, x_21);
lean_ctor_set_uint8(x_23, sizeof(void*)*1, x_22);
x_24 = l_Repr_addAppParen(x_23, x_2);
return x_24;
}
}
block_9:
{
lean_object* x_4; lean_object* x_5; uint8_t x_6; lean_object* x_7; lean_object* x_8; 
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__1));
x_5 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_5, 0, x_3);
lean_ctor_set(x_5, 1, x_4);
x_6 = 0;
x_7 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_7, 0, x_5);
lean_ctor_set_uint8(x_7, sizeof(void*)*1, x_6);
x_8 = l_Repr_addAppParen(x_7, x_2);
return x_8;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(8u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__9(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__0));
x_2 = lean_string_length(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__9, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__9_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__9);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; uint8_t x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; 
x_2 = lean_ctor_get(x_1, 0);
lean_inc(x_2);
lean_dec_ref(x_1);
x_3 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__6));
x_4 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7);
x_5 = lean_unsigned_to_nat(0u);
x_6 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0(x_2, x_5);
x_7 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_7, 0, x_4);
lean_ctor_set(x_7, 1, x_6);
x_8 = 0;
x_9 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_9, 0, x_7);
lean_ctor_set_uint8(x_9, sizeof(void*)*1, x_8);
x_10 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_10, 0, x_3);
lean_ctor_set(x_10, 1, x_9);
x_11 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10);
x_12 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__11));
x_13 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_13, 0, x_12);
lean_ctor_set(x_13, 1, x_10);
x_14 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__12));
x_15 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_15, 0, x_13);
lean_ctor_set(x_15, 1, x_14);
x_16 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_16, 0, x_11);
lean_ctor_set(x_16, 1, x_15);
x_17 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_17, 0, x_16);
lean_ctor_set_uint8(x_17, sizeof(void*)*1, x_8);
return x_17;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__0(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_10; uint8_t x_11; 
x_10 = lean_unsigned_to_nat(1024u);
x_11 = lean_nat_dec_le(x_10, x_2);
if (x_11 == 0)
{
lean_object* x_12; 
x_12 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2);
x_3 = x_12;
goto block_9;
}
else
{
lean_object* x_13; 
x_13 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_3 = x_13;
goto block_9;
}
}
else
{
lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_26; uint8_t x_27; 
x_14 = lean_ctor_get(x_1, 0);
lean_inc(x_14);
if (lean_is_exclusive(x_1)) {
 lean_ctor_release(x_1, 0);
 x_15 = x_1;
} else {
 lean_dec_ref(x_1);
 x_15 = lean_box(0);
}
x_26 = lean_unsigned_to_nat(1024u);
x_27 = lean_nat_dec_le(x_26, x_2);
if (x_27 == 0)
{
lean_object* x_28; 
x_28 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2);
x_16 = x_28;
goto block_25;
}
else
{
lean_object* x_29; 
x_29 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_16 = x_29;
goto block_25;
}
block_25:
{
lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; uint8_t x_22; lean_object* x_23; lean_object* x_24; 
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__6));
x_18 = l_Nat_reprFast(x_14);
if (lean_is_scalar(x_15)) {
 x_19 = lean_alloc_ctor(3, 1, 0);
} else {
 x_19 = x_15;
 lean_ctor_set_tag(x_19, 3);
}
lean_ctor_set(x_19, 0, x_18);
x_20 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_20, 0, x_17);
lean_ctor_set(x_20, 1, x_19);
x_21 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_21, 0, x_16);
lean_ctor_set(x_21, 1, x_20);
x_22 = 0;
x_23 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_23, 0, x_21);
lean_ctor_set_uint8(x_23, sizeof(void*)*1, x_22);
x_24 = l_Repr_addAppParen(x_23, x_2);
return x_24;
}
}
block_9:
{
lean_object* x_4; lean_object* x_5; uint8_t x_6; lean_object* x_7; lean_object* x_8; 
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__1));
x_5 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_5, 0, x_3);
lean_ctor_set(x_5, 1, x_4);
x_6 = 0;
x_7 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_7, 0, x_5);
lean_ctor_set_uint8(x_7, sizeof(void*)*1, x_6);
x_8 = l_Repr_addAppParen(x_7, x_2);
return x_8;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__0___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__0(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__1(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_10; uint8_t x_11; 
x_10 = lean_unsigned_to_nat(1024u);
x_11 = lean_nat_dec_le(x_10, x_2);
if (x_11 == 0)
{
lean_object* x_12; 
x_12 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2);
x_3 = x_12;
goto block_9;
}
else
{
lean_object* x_13; 
x_13 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_3 = x_13;
goto block_9;
}
}
else
{
lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_26; uint8_t x_27; 
x_14 = lean_ctor_get(x_1, 0);
lean_inc(x_14);
if (lean_is_exclusive(x_1)) {
 lean_ctor_release(x_1, 0);
 x_15 = x_1;
} else {
 lean_dec_ref(x_1);
 x_15 = lean_box(0);
}
x_26 = lean_unsigned_to_nat(1024u);
x_27 = lean_nat_dec_le(x_26, x_2);
if (x_27 == 0)
{
lean_object* x_28; 
x_28 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2);
x_16 = x_28;
goto block_25;
}
else
{
lean_object* x_29; 
x_29 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_16 = x_29;
goto block_25;
}
block_25:
{
lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; uint8_t x_22; lean_object* x_23; lean_object* x_24; 
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__6));
x_18 = l_Nat_reprFast(x_14);
if (lean_is_scalar(x_15)) {
 x_19 = lean_alloc_ctor(3, 1, 0);
} else {
 x_19 = x_15;
 lean_ctor_set_tag(x_19, 3);
}
lean_ctor_set(x_19, 0, x_18);
x_20 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_20, 0, x_17);
lean_ctor_set(x_20, 1, x_19);
x_21 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_21, 0, x_16);
lean_ctor_set(x_21, 1, x_20);
x_22 = 0;
x_23 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_23, 0, x_21);
lean_ctor_set_uint8(x_23, sizeof(void*)*1, x_22);
x_24 = l_Repr_addAppParen(x_23, x_2);
return x_24;
}
}
block_9:
{
lean_object* x_4; lean_object* x_5; uint8_t x_6; lean_object* x_7; lean_object* x_8; 
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__1));
x_5 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_5, 0, x_3);
lean_ctor_set(x_5, 1, x_4);
x_6 = 0;
x_7 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_7, 0, x_5);
lean_ctor_set_uint8(x_7, sizeof(void*)*1, x_6);
x_8 = l_Repr_addAppParen(x_7, x_2);
return x_8;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__1___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__1(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(13u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__9(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(12u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(15u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; uint8_t x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; lean_object* x_32; lean_object* x_33; lean_object* x_34; lean_object* x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; lean_object* x_39; lean_object* x_40; lean_object* x_41; lean_object* x_42; lean_object* x_43; lean_object* x_44; lean_object* x_45; lean_object* x_46; lean_object* x_47; lean_object* x_48; lean_object* x_49; lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; lean_object* x_55; 
x_2 = lean_ctor_get(x_1, 0);
lean_inc(x_2);
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
x_4 = lean_ctor_get(x_1, 2);
lean_inc(x_4);
x_5 = lean_ctor_get(x_1, 3);
lean_inc(x_5);
lean_dec_ref(x_1);
x_6 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__5));
x_7 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__3));
x_8 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4);
x_9 = lean_unsigned_to_nat(0u);
x_10 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__0(x_2, x_9);
x_11 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_11, 0, x_8);
lean_ctor_set(x_11, 1, x_10);
x_12 = 0;
x_13 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_13, 0, x_11);
lean_ctor_set_uint8(x_13, sizeof(void*)*1, x_12);
x_14 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_14, 0, x_7);
lean_ctor_set(x_14, 1, x_13);
x_15 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__6));
x_16 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_16, 0, x_14);
lean_ctor_set(x_16, 1, x_15);
x_17 = lean_box(1);
x_18 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_18, 0, x_16);
lean_ctor_set(x_18, 1, x_17);
x_19 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__8));
x_20 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_20, 0, x_18);
lean_ctor_set(x_20, 1, x_19);
x_21 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_21, 0, x_20);
lean_ctor_set(x_21, 1, x_6);
x_22 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__9, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__9_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__9);
x_23 = l_Nat_reprFast(x_3);
x_24 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_24, 0, x_23);
x_25 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_25, 0, x_22);
lean_ctor_set(x_25, 1, x_24);
x_26 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_26, 0, x_25);
lean_ctor_set_uint8(x_26, sizeof(void*)*1, x_12);
x_27 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_27, 0, x_21);
lean_ctor_set(x_27, 1, x_26);
x_28 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_28, 0, x_27);
lean_ctor_set(x_28, 1, x_15);
x_29 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_29, 0, x_28);
lean_ctor_set(x_29, 1, x_17);
x_30 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__11));
x_31 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_31, 0, x_29);
lean_ctor_set(x_31, 1, x_30);
x_32 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_32, 0, x_31);
lean_ctor_set(x_32, 1, x_6);
x_33 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12);
x_34 = l_Nat_reprFast(x_4);
x_35 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_35, 0, x_34);
x_36 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_36, 0, x_33);
lean_ctor_set(x_36, 1, x_35);
x_37 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_37, 0, x_36);
lean_ctor_set_uint8(x_37, sizeof(void*)*1, x_12);
x_38 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_38, 0, x_32);
lean_ctor_set(x_38, 1, x_37);
x_39 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_39, 0, x_38);
lean_ctor_set(x_39, 1, x_15);
x_40 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_40, 0, x_39);
lean_ctor_set(x_40, 1, x_17);
x_41 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__2));
x_42 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_42, 0, x_40);
lean_ctor_set(x_42, 1, x_41);
x_43 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_43, 0, x_42);
lean_ctor_set(x_43, 1, x_6);
x_44 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__7);
x_45 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlab_repr_spec__1(x_5, x_9);
x_46 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_46, 0, x_44);
lean_ctor_set(x_46, 1, x_45);
x_47 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_47, 0, x_46);
lean_ctor_set_uint8(x_47, sizeof(void*)*1, x_12);
x_48 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_48, 0, x_43);
lean_ctor_set(x_48, 1, x_47);
x_49 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10);
x_50 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__11));
x_51 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_51, 0, x_50);
lean_ctor_set(x_51, 1, x_48);
x_52 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__12));
x_53 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_53, 0, x_51);
lean_ctor_set(x_53, 1, x_52);
x_54 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_54, 0, x_49);
lean_ctor_set(x_54, 1, x_53);
x_55 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_55, 0, x_54);
lean_ctor_set_uint8(x_55, sizeof(void*)*1, x_12);
return x_55;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprKmemCache_repr_spec__0(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
if (lean_obj_tag(x_1) == 0)
{
lean_object* x_10; uint8_t x_11; 
x_10 = lean_unsigned_to_nat(1024u);
x_11 = lean_nat_dec_le(x_10, x_2);
if (x_11 == 0)
{
lean_object* x_12; 
x_12 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2);
x_3 = x_12;
goto block_9;
}
else
{
lean_object* x_13; 
x_13 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_3 = x_13;
goto block_9;
}
}
else
{
lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_26; uint8_t x_27; 
x_14 = lean_ctor_get(x_1, 0);
lean_inc(x_14);
if (lean_is_exclusive(x_1)) {
 lean_ctor_release(x_1, 0);
 x_15 = x_1;
} else {
 lean_dec_ref(x_1);
 x_15 = lean_box(0);
}
x_26 = lean_unsigned_to_nat(1024u);
x_27 = lean_nat_dec_le(x_26, x_2);
if (x_27 == 0)
{
lean_object* x_28; 
x_28 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__2);
x_16 = x_28;
goto block_25;
}
else
{
lean_object* x_29; 
x_29 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_16 = x_29;
goto block_25;
}
block_25:
{
lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; uint8_t x_22; lean_object* x_23; lean_object* x_24; 
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__6));
x_18 = l_Nat_reprFast(x_14);
if (lean_is_scalar(x_15)) {
 x_19 = lean_alloc_ctor(3, 1, 0);
} else {
 x_19 = x_15;
 lean_ctor_set_tag(x_19, 3);
}
lean_ctor_set(x_19, 0, x_18);
x_20 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_20, 0, x_17);
lean_ctor_set(x_20, 1, x_19);
x_21 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_21, 0, x_16);
lean_ctor_set(x_21, 1, x_20);
x_22 = 0;
x_23 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_23, 0, x_21);
lean_ctor_set_uint8(x_23, sizeof(void*)*1, x_22);
x_24 = l_Repr_addAppParen(x_23, x_2);
return x_24;
}
}
block_9:
{
lean_object* x_4; lean_object* x_5; uint8_t x_6; lean_object* x_7; lean_object* x_8; 
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__1));
x_5 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_5, 0, x_3);
lean_ctor_set(x_5, 1, x_4);
x_6 = 0;
x_7 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_7, 0, x_5);
lean_ctor_set_uint8(x_7, sizeof(void*)*1, x_6);
x_8 = l_Repr_addAppParen(x_7, x_2);
return x_8;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprKmemCache_repr_spec__0___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprKmemCache_repr_spec__0(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__6(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(14u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__13(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(17u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__16(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(21u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; uint8_t x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; lean_object* x_32; lean_object* x_33; lean_object* x_34; lean_object* x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; lean_object* x_39; lean_object* x_40; lean_object* x_41; lean_object* x_42; lean_object* x_43; lean_object* x_44; lean_object* x_45; lean_object* x_46; lean_object* x_47; lean_object* x_48; lean_object* x_49; lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; lean_object* x_55; lean_object* x_56; lean_object* x_57; lean_object* x_58; lean_object* x_59; lean_object* x_60; lean_object* x_61; lean_object* x_62; lean_object* x_63; lean_object* x_64; lean_object* x_65; lean_object* x_66; lean_object* x_67; lean_object* x_68; lean_object* x_69; lean_object* x_70; lean_object* x_71; lean_object* x_72; lean_object* x_73; lean_object* x_74; lean_object* x_75; lean_object* x_76; lean_object* x_77; lean_object* x_78; lean_object* x_79; 
x_2 = lean_ctor_get(x_1, 0);
lean_inc(x_2);
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
x_4 = lean_ctor_get(x_1, 2);
lean_inc(x_4);
x_5 = lean_ctor_get(x_1, 3);
lean_inc(x_5);
x_6 = lean_ctor_get(x_1, 4);
lean_inc(x_6);
x_7 = lean_ctor_get(x_1, 5);
lean_inc(x_7);
lean_dec_ref(x_1);
x_8 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__5));
x_9 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__3));
x_10 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__12);
x_11 = l_Nat_reprFast(x_2);
x_12 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_12, 0, x_11);
x_13 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_13, 0, x_10);
lean_ctor_set(x_13, 1, x_12);
x_14 = 0;
x_15 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_15, 0, x_13);
lean_ctor_set_uint8(x_15, sizeof(void*)*1, x_14);
x_16 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_16, 0, x_9);
lean_ctor_set(x_16, 1, x_15);
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__6));
x_18 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_18, 0, x_16);
lean_ctor_set(x_18, 1, x_17);
x_19 = lean_box(1);
x_20 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_20, 0, x_18);
lean_ctor_set(x_20, 1, x_19);
x_21 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__5));
x_22 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_22, 0, x_20);
lean_ctor_set(x_22, 1, x_21);
x_23 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_23, 0, x_22);
lean_ctor_set(x_23, 1, x_8);
x_24 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__6, &lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__6_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__6);
x_25 = l_Nat_reprFast(x_3);
x_26 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_26, 0, x_25);
x_27 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_27, 0, x_24);
lean_ctor_set(x_27, 1, x_26);
x_28 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_28, 0, x_27);
lean_ctor_set_uint8(x_28, sizeof(void*)*1, x_14);
x_29 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_29, 0, x_23);
lean_ctor_set(x_29, 1, x_28);
x_30 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_30, 0, x_29);
lean_ctor_set(x_30, 1, x_17);
x_31 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_31, 0, x_30);
lean_ctor_set(x_31, 1, x_19);
x_32 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__8));
x_33 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_33, 0, x_31);
lean_ctor_set(x_33, 1, x_32);
x_34 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_34, 0, x_33);
lean_ctor_set(x_34, 1, x_8);
x_35 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlab_repr___redArg___closed__4);
x_36 = lean_unsigned_to_nat(0u);
x_37 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprKmemCache_repr_spec__0(x_4, x_36);
x_38 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_38, 0, x_35);
lean_ctor_set(x_38, 1, x_37);
x_39 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_39, 0, x_38);
lean_ctor_set_uint8(x_39, sizeof(void*)*1, x_14);
x_40 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_40, 0, x_34);
lean_ctor_set(x_40, 1, x_39);
x_41 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_41, 0, x_40);
lean_ctor_set(x_41, 1, x_17);
x_42 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_42, 0, x_41);
lean_ctor_set(x_42, 1, x_19);
x_43 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__10));
x_44 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_44, 0, x_42);
lean_ctor_set(x_44, 1, x_43);
x_45 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_45, 0, x_44);
lean_ctor_set(x_45, 1, x_8);
x_46 = l_Nat_reprFast(x_5);
x_47 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_47, 0, x_46);
x_48 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_48, 0, x_10);
lean_ctor_set(x_48, 1, x_47);
x_49 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_49, 0, x_48);
lean_ctor_set_uint8(x_49, sizeof(void*)*1, x_14);
x_50 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_50, 0, x_45);
lean_ctor_set(x_50, 1, x_49);
x_51 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_51, 0, x_50);
lean_ctor_set(x_51, 1, x_17);
x_52 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_52, 0, x_51);
lean_ctor_set(x_52, 1, x_19);
x_53 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__12));
x_54 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_54, 0, x_52);
lean_ctor_set(x_54, 1, x_53);
x_55 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_55, 0, x_54);
lean_ctor_set(x_55, 1, x_8);
x_56 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__13, &lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__13_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__13);
x_57 = l_Nat_reprFast(x_6);
x_58 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_58, 0, x_57);
x_59 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_59, 0, x_56);
lean_ctor_set(x_59, 1, x_58);
x_60 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_60, 0, x_59);
lean_ctor_set_uint8(x_60, sizeof(void*)*1, x_14);
x_61 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_61, 0, x_55);
lean_ctor_set(x_61, 1, x_60);
x_62 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_62, 0, x_61);
lean_ctor_set(x_62, 1, x_17);
x_63 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_63, 0, x_62);
lean_ctor_set(x_63, 1, x_19);
x_64 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__15));
x_65 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_65, 0, x_63);
lean_ctor_set(x_65, 1, x_64);
x_66 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_66, 0, x_65);
lean_ctor_set(x_66, 1, x_8);
x_67 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__16, &lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__16_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg___closed__16);
x_68 = l_Nat_reprFast(x_7);
x_69 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_69, 0, x_68);
x_70 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_70, 0, x_67);
lean_ctor_set(x_70, 1, x_69);
x_71 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_71, 0, x_70);
lean_ctor_set_uint8(x_71, sizeof(void*)*1, x_14);
x_72 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_72, 0, x_66);
lean_ctor_set(x_72, 1, x_71);
x_73 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10, &lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10_once, _init_lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__10);
x_74 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__11));
x_75 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_75, 0, x_74);
lean_ctor_set(x_75, 1, x_72);
x_76 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_instReprSlabObject_repr___redArg___closed__12));
x_77 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_77, 0, x_75);
lean_ctor_set(x_77, 1, x_76);
x_78 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_78, 0, x_73);
lean_ctor_set(x_78, 1, x_77);
x_79 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_79, 0, x_78);
lean_ctor_set_uint8(x_79, sizeof(void*)*1, x_14);
return x_79;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_instReprKmemCache_repr(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__kmem__cache(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_2 = lean_unsigned_to_nat(0u);
x_3 = lean_box(0);
x_4 = lean_alloc_ctor(0, 6, 0);
lean_ctor_set(x_4, 0, x_1);
lean_ctor_set(x_4, 1, x_2);
lean_ctor_set(x_4, 2, x_3);
lean_ctor_set(x_4, 3, x_2);
lean_ctor_set(x_4, 4, x_2);
lean_ctor_set(x_4, 5, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_cache__size__for__idx(lean_object* x_1) {
_start:
{
lean_object* x_2; uint8_t x_3; 
x_2 = lean_unsigned_to_nat(0u);
x_3 = lean_nat_dec_eq(x_1, x_2);
if (x_3 == 0)
{
lean_object* x_4; uint8_t x_5; 
x_4 = lean_unsigned_to_nat(1u);
x_5 = lean_nat_dec_eq(x_1, x_4);
if (x_5 == 0)
{
lean_object* x_6; uint8_t x_7; 
x_6 = lean_unsigned_to_nat(2u);
x_7 = lean_nat_dec_eq(x_1, x_6);
if (x_7 == 0)
{
lean_object* x_8; uint8_t x_9; 
x_8 = lean_unsigned_to_nat(3u);
x_9 = lean_nat_dec_eq(x_1, x_8);
if (x_9 == 0)
{
lean_object* x_10; uint8_t x_11; 
x_10 = lean_unsigned_to_nat(4u);
x_11 = lean_nat_dec_eq(x_1, x_10);
if (x_11 == 0)
{
lean_object* x_12; uint8_t x_13; 
x_12 = lean_unsigned_to_nat(5u);
x_13 = lean_nat_dec_eq(x_1, x_12);
if (x_13 == 0)
{
lean_object* x_14; uint8_t x_15; 
x_14 = lean_unsigned_to_nat(6u);
x_15 = lean_nat_dec_eq(x_1, x_14);
if (x_15 == 0)
{
lean_object* x_16; uint8_t x_17; 
x_16 = lean_unsigned_to_nat(7u);
x_17 = lean_nat_dec_eq(x_1, x_16);
if (x_17 == 0)
{
lean_object* x_18; 
x_18 = lean_unsigned_to_nat(32u);
return x_18;
}
else
{
lean_object* x_19; 
x_19 = lean_unsigned_to_nat(4096u);
return x_19;
}
}
else
{
lean_object* x_20; 
x_20 = lean_unsigned_to_nat(2048u);
return x_20;
}
}
else
{
lean_object* x_21; 
x_21 = lean_unsigned_to_nat(1024u);
return x_21;
}
}
else
{
lean_object* x_22; 
x_22 = lean_unsigned_to_nat(512u);
return x_22;
}
}
else
{
lean_object* x_23; 
x_23 = lean_unsigned_to_nat(256u);
return x_23;
}
}
else
{
lean_object* x_24; 
x_24 = lean_unsigned_to_nat(128u);
return x_24;
}
}
else
{
lean_object* x_25; 
x_25 = lean_unsigned_to_nat(64u);
return x_25;
}
}
else
{
lean_object* x_26; 
x_26 = lean_unsigned_to_nat(32u);
return x_26;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_cache__size__for__idx___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_Slab_cache__size__for__idx(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___lam__0(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = lp_mvk__specs_MVK_Phase2_Slab_cache__size__for__idx(x_1);
x_3 = lp_mvk__specs_MVK_Phase2_Slab_initial__kmem__cache(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___lam__0___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___lam__0(x_1);
lean_dec(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__1(void) {
_start:
{
lean_object* x_1; lean_object* x_2; uint8_t x_3; lean_object* x_4; 
x_1 = lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state;
x_2 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__0));
x_3 = 0;
x_4 = lean_alloc_ctor(0, 2, 1);
lean_ctor_set(x_4, 0, x_2);
lean_ctor_set(x_4, 1, x_1);
lean_ctor_set_uint8(x_4, sizeof(void*)*2, x_3);
return x_4;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__1, &lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__1_once, _init_lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state___closed__1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__order__for__size(lean_object* x_1) {
_start:
{
lean_object* x_2; uint8_t x_3; 
x_2 = lean_unsigned_to_nat(512u);
x_3 = lean_nat_dec_le(x_1, x_2);
if (x_3 == 0)
{
lean_object* x_4; uint8_t x_5; 
x_4 = lean_unsigned_to_nat(2048u);
x_5 = lean_nat_dec_le(x_1, x_4);
if (x_5 == 0)
{
lean_object* x_6; 
x_6 = lean_unsigned_to_nat(2u);
return x_6;
}
else
{
lean_object* x_7; 
x_7 = lean_unsigned_to_nat(1u);
return x_7;
}
}
else
{
lean_object* x_8; 
x_8 = lean_unsigned_to_nat(0u);
return x_8;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__order__for__size___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_Slab_slab__order__for__size(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx___lam__0(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; 
x_3 = lean_nat_dec_le(x_1, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx___lam__0___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx___lam__0(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
x_4 = lean_box(x_3);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; 
x_2 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx___lam__0___boxed), 2, 1);
lean_closure_set(x_2, 0, x_1);
x_3 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_CACHE__SIZES));
x_4 = l_List_findIdx_x3f___redArg(x_2, x_3);
if (lean_obj_tag(x_4) == 0)
{
lean_object* x_5; 
x_5 = lean_box(0);
return x_5;
}
else
{
uint8_t x_6; 
x_6 = !lean_is_exclusive(x_4);
if (x_6 == 0)
{
lean_object* x_7; lean_object* x_8; uint8_t x_9; 
x_7 = lean_ctor_get(x_4, 0);
x_8 = lean_unsigned_to_nat(8u);
x_9 = lean_nat_dec_lt(x_7, x_8);
if (x_9 == 0)
{
lean_object* x_10; 
lean_free_object(x_4);
lean_dec(x_7);
x_10 = lean_box(0);
return x_10;
}
else
{
return x_4;
}
}
else
{
lean_object* x_11; lean_object* x_12; uint8_t x_13; 
x_11 = lean_ctor_get(x_4, 0);
lean_inc(x_11);
lean_dec(x_4);
x_12 = lean_unsigned_to_nat(8u);
x_13 = lean_nat_dec_lt(x_11, x_12);
if (x_13 == 0)
{
lean_object* x_14; 
lean_dec(x_11);
x_14 = lean_box(0);
return x_14;
}
else
{
lean_object* x_15; 
x_15 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_15, 0, x_11);
return x_15;
}
}
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_3 = lean_unsigned_to_nat(4096u);
x_4 = lean_unsigned_to_nat(2u);
x_5 = lean_nat_pow(x_4, x_2);
x_6 = lean_nat_mul(x_3, x_5);
lean_dec(x_5);
x_7 = lean_unsigned_to_nat(32u);
x_8 = lean_nat_sub(x_6, x_7);
lean_dec(x_6);
x_9 = lean_nat_div(x_8, x_1);
lean_dec(x_8);
return x_9;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab(x_1, x_2);
lean_dec(x_2);
lean_dec(x_1);
return x_3;
}
}
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; uint8_t x_4; 
x_2 = lean_ctor_get(x_1, 4);
x_3 = lean_ctor_get(x_1, 5);
x_4 = lean_nat_dec_lt(x_3, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(x_1);
lean_dec_ref(x_1);
x_3 = lean_box(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___lam__0(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_2 = lp_mvk__specs_MVK_Phase2_Slab_cache__size__for__idx(x_1);
x_3 = lp_mvk__specs_MVK_Phase2_Slab_slab__order__for__size(x_2);
x_4 = lean_box(0);
x_5 = lean_unsigned_to_nat(0u);
x_6 = lean_alloc_ctor(0, 6, 0);
lean_ctor_set(x_6, 0, x_2);
lean_ctor_set(x_6, 1, x_3);
lean_ctor_set(x_6, 2, x_4);
lean_ctor_set(x_6, 3, x_5);
lean_ctor_set(x_6, 4, x_5);
lean_ctor_set(x_6, 5, x_5);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___lam__0___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___lam__0(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec(lean_object* x_1) {
_start:
{
uint8_t x_3; 
x_3 = lean_ctor_get_uint8(x_1, sizeof(void*)*2);
if (x_3 == 0)
{
uint8_t x_4; 
x_4 = !lean_is_exclusive(x_1);
if (x_4 == 0)
{
lean_object* x_5; lean_object* x_6; uint8_t x_7; lean_object* x_8; 
x_5 = lean_ctor_get(x_1, 0);
lean_dec(x_5);
x_6 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___closed__0));
x_7 = 1;
lean_ctor_set(x_1, 0, x_6);
lean_ctor_set_uint8(x_1, sizeof(void*)*2, x_7);
x_8 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_8, 0, x_1);
return x_8;
}
else
{
lean_object* x_9; lean_object* x_10; uint8_t x_11; lean_object* x_12; lean_object* x_13; 
x_9 = lean_ctor_get(x_1, 1);
lean_inc(x_9);
lean_dec(x_1);
x_10 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___closed__0));
x_11 = 1;
x_12 = lean_alloc_ctor(0, 2, 1);
lean_ctor_set(x_12, 0, x_10);
lean_ctor_set(x_12, 1, x_9);
lean_ctor_set_uint8(x_12, sizeof(void*)*2, x_11);
x_13 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_13, 0, x_12);
return x_13;
}
}
else
{
lean_object* x_14; 
x_14 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_14, 0, x_1);
return x_14;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_slab__init__spec(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
x_5 = lean_nat_dec_eq(x_4, x_1);
if (x_5 == 0)
{
lean_object* x_6; 
x_6 = lean_apply_1(x_2, x_4);
return x_6;
}
else
{
lean_dec(x_4);
lean_dec_ref(x_2);
lean_inc_ref(x_3);
return x_3;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0(x_1, x_2, x_3, x_4);
lean_dec_ref(x_3);
lean_dec(x_1);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; uint8_t x_8; 
x_4 = lean_ctor_get_uint8(x_2, sizeof(void*)*2);
x_5 = lean_ctor_get(x_2, 0);
x_6 = lean_ctor_get(x_2, 1);
lean_inc_ref(x_5);
lean_inc(x_1);
x_7 = lean_apply_1(x_5, x_1);
x_8 = !lean_is_exclusive(x_7);
if (x_8 == 0)
{
lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; uint8_t x_16; 
x_9 = lean_ctor_get(x_7, 0);
x_10 = lean_ctor_get(x_7, 1);
x_11 = lean_ctor_get(x_7, 2);
x_12 = lean_ctor_get(x_7, 3);
x_13 = lean_ctor_get(x_7, 4);
x_14 = lean_ctor_get(x_7, 5);
lean_inc_ref(x_6);
x_15 = lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec(x_10, x_6);
x_16 = !lean_is_exclusive(x_15);
if (x_16 == 0)
{
lean_object* x_17; uint8_t x_18; 
x_17 = lean_ctor_get(x_15, 0);
x_18 = !lean_is_exclusive(x_17);
if (x_18 == 0)
{
lean_object* x_19; lean_object* x_20; lean_object* x_21; uint8_t x_22; 
x_19 = lean_ctor_get(x_17, 0);
x_20 = lean_ctor_get(x_17, 1);
x_21 = lean_box(0);
x_22 = lp_mvk__specs_MVK_Phase2_Common_instDecidableEqPointer_decEq___redArg(x_19, x_21);
lean_dec(x_19);
if (x_22 == 0)
{
uint8_t x_23; 
lean_inc_ref(x_5);
x_23 = !lean_is_exclusive(x_2);
if (x_23 == 0)
{
lean_object* x_24; lean_object* x_25; uint8_t x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; lean_object* x_32; 
x_24 = lean_ctor_get(x_2, 1);
lean_dec(x_24);
x_25 = lean_ctor_get(x_2, 0);
lean_dec(x_25);
x_26 = 1;
x_27 = lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab(x_9, x_10);
x_28 = lean_unsigned_to_nat(1u);
x_29 = lean_nat_add(x_12, x_28);
lean_dec(x_12);
x_30 = lean_nat_add(x_13, x_27);
lean_dec(x_27);
lean_dec(x_13);
lean_ctor_set(x_7, 4, x_30);
lean_ctor_set(x_7, 3, x_29);
x_31 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_31, 0, x_1);
lean_closure_set(x_31, 1, x_5);
lean_closure_set(x_31, 2, x_7);
lean_ctor_set(x_2, 1, x_20);
lean_ctor_set(x_2, 0, x_31);
x_32 = lean_box(x_26);
lean_ctor_set(x_17, 1, x_2);
lean_ctor_set(x_17, 0, x_32);
return x_15;
}
else
{
uint8_t x_33; lean_object* x_34; lean_object* x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; lean_object* x_39; lean_object* x_40; 
lean_dec(x_2);
x_33 = 1;
x_34 = lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab(x_9, x_10);
x_35 = lean_unsigned_to_nat(1u);
x_36 = lean_nat_add(x_12, x_35);
lean_dec(x_12);
x_37 = lean_nat_add(x_13, x_34);
lean_dec(x_34);
lean_dec(x_13);
lean_ctor_set(x_7, 4, x_37);
lean_ctor_set(x_7, 3, x_36);
x_38 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_38, 0, x_1);
lean_closure_set(x_38, 1, x_5);
lean_closure_set(x_38, 2, x_7);
x_39 = lean_alloc_ctor(0, 2, 1);
lean_ctor_set(x_39, 0, x_38);
lean_ctor_set(x_39, 1, x_20);
lean_ctor_set_uint8(x_39, sizeof(void*)*2, x_4);
x_40 = lean_box(x_33);
lean_ctor_set(x_17, 1, x_39);
lean_ctor_set(x_17, 0, x_40);
return x_15;
}
}
else
{
uint8_t x_41; lean_object* x_42; 
lean_dec(x_20);
lean_free_object(x_7);
lean_dec(x_14);
lean_dec(x_13);
lean_dec(x_12);
lean_dec(x_11);
lean_dec(x_10);
lean_dec(x_9);
lean_dec(x_1);
x_41 = 0;
x_42 = lean_box(x_41);
lean_ctor_set(x_17, 1, x_2);
lean_ctor_set(x_17, 0, x_42);
return x_15;
}
}
else
{
lean_object* x_43; lean_object* x_44; lean_object* x_45; uint8_t x_46; 
x_43 = lean_ctor_get(x_17, 0);
x_44 = lean_ctor_get(x_17, 1);
lean_inc(x_44);
lean_inc(x_43);
lean_dec(x_17);
x_45 = lean_box(0);
x_46 = lp_mvk__specs_MVK_Phase2_Common_instDecidableEqPointer_decEq___redArg(x_43, x_45);
lean_dec(x_43);
if (x_46 == 0)
{
lean_object* x_47; uint8_t x_48; lean_object* x_49; lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; lean_object* x_55; lean_object* x_56; 
lean_inc_ref(x_5);
if (lean_is_exclusive(x_2)) {
 lean_ctor_release(x_2, 0);
 lean_ctor_release(x_2, 1);
 x_47 = x_2;
} else {
 lean_dec_ref(x_2);
 x_47 = lean_box(0);
}
x_48 = 1;
x_49 = lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab(x_9, x_10);
x_50 = lean_unsigned_to_nat(1u);
x_51 = lean_nat_add(x_12, x_50);
lean_dec(x_12);
x_52 = lean_nat_add(x_13, x_49);
lean_dec(x_49);
lean_dec(x_13);
lean_ctor_set(x_7, 4, x_52);
lean_ctor_set(x_7, 3, x_51);
x_53 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_53, 0, x_1);
lean_closure_set(x_53, 1, x_5);
lean_closure_set(x_53, 2, x_7);
if (lean_is_scalar(x_47)) {
 x_54 = lean_alloc_ctor(0, 2, 1);
} else {
 x_54 = x_47;
}
lean_ctor_set(x_54, 0, x_53);
lean_ctor_set(x_54, 1, x_44);
lean_ctor_set_uint8(x_54, sizeof(void*)*2, x_4);
x_55 = lean_box(x_48);
x_56 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_56, 0, x_55);
lean_ctor_set(x_56, 1, x_54);
lean_ctor_set(x_15, 0, x_56);
return x_15;
}
else
{
uint8_t x_57; lean_object* x_58; lean_object* x_59; 
lean_dec(x_44);
lean_free_object(x_7);
lean_dec(x_14);
lean_dec(x_13);
lean_dec(x_12);
lean_dec(x_11);
lean_dec(x_10);
lean_dec(x_9);
lean_dec(x_1);
x_57 = 0;
x_58 = lean_box(x_57);
x_59 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_59, 0, x_58);
lean_ctor_set(x_59, 1, x_2);
lean_ctor_set(x_15, 0, x_59);
return x_15;
}
}
}
else
{
lean_object* x_60; lean_object* x_61; lean_object* x_62; lean_object* x_63; lean_object* x_64; uint8_t x_65; 
x_60 = lean_ctor_get(x_15, 0);
lean_inc(x_60);
lean_dec(x_15);
x_61 = lean_ctor_get(x_60, 0);
lean_inc(x_61);
x_62 = lean_ctor_get(x_60, 1);
lean_inc(x_62);
if (lean_is_exclusive(x_60)) {
 lean_ctor_release(x_60, 0);
 lean_ctor_release(x_60, 1);
 x_63 = x_60;
} else {
 lean_dec_ref(x_60);
 x_63 = lean_box(0);
}
x_64 = lean_box(0);
x_65 = lp_mvk__specs_MVK_Phase2_Common_instDecidableEqPointer_decEq___redArg(x_61, x_64);
lean_dec(x_61);
if (x_65 == 0)
{
lean_object* x_66; uint8_t x_67; lean_object* x_68; lean_object* x_69; lean_object* x_70; lean_object* x_71; lean_object* x_72; lean_object* x_73; lean_object* x_74; lean_object* x_75; lean_object* x_76; 
lean_inc_ref(x_5);
if (lean_is_exclusive(x_2)) {
 lean_ctor_release(x_2, 0);
 lean_ctor_release(x_2, 1);
 x_66 = x_2;
} else {
 lean_dec_ref(x_2);
 x_66 = lean_box(0);
}
x_67 = 1;
x_68 = lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab(x_9, x_10);
x_69 = lean_unsigned_to_nat(1u);
x_70 = lean_nat_add(x_12, x_69);
lean_dec(x_12);
x_71 = lean_nat_add(x_13, x_68);
lean_dec(x_68);
lean_dec(x_13);
lean_ctor_set(x_7, 4, x_71);
lean_ctor_set(x_7, 3, x_70);
x_72 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_72, 0, x_1);
lean_closure_set(x_72, 1, x_5);
lean_closure_set(x_72, 2, x_7);
if (lean_is_scalar(x_66)) {
 x_73 = lean_alloc_ctor(0, 2, 1);
} else {
 x_73 = x_66;
}
lean_ctor_set(x_73, 0, x_72);
lean_ctor_set(x_73, 1, x_62);
lean_ctor_set_uint8(x_73, sizeof(void*)*2, x_4);
x_74 = lean_box(x_67);
if (lean_is_scalar(x_63)) {
 x_75 = lean_alloc_ctor(0, 2, 0);
} else {
 x_75 = x_63;
}
lean_ctor_set(x_75, 0, x_74);
lean_ctor_set(x_75, 1, x_73);
x_76 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_76, 0, x_75);
return x_76;
}
else
{
uint8_t x_77; lean_object* x_78; lean_object* x_79; lean_object* x_80; 
lean_dec(x_62);
lean_free_object(x_7);
lean_dec(x_14);
lean_dec(x_13);
lean_dec(x_12);
lean_dec(x_11);
lean_dec(x_10);
lean_dec(x_9);
lean_dec(x_1);
x_77 = 0;
x_78 = lean_box(x_77);
if (lean_is_scalar(x_63)) {
 x_79 = lean_alloc_ctor(0, 2, 0);
} else {
 x_79 = x_63;
}
lean_ctor_set(x_79, 0, x_78);
lean_ctor_set(x_79, 1, x_2);
x_80 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_80, 0, x_79);
return x_80;
}
}
}
else
{
lean_object* x_81; lean_object* x_82; lean_object* x_83; lean_object* x_84; lean_object* x_85; lean_object* x_86; lean_object* x_87; lean_object* x_88; lean_object* x_89; lean_object* x_90; lean_object* x_91; lean_object* x_92; lean_object* x_93; uint8_t x_94; 
x_81 = lean_ctor_get(x_7, 0);
x_82 = lean_ctor_get(x_7, 1);
x_83 = lean_ctor_get(x_7, 2);
x_84 = lean_ctor_get(x_7, 3);
x_85 = lean_ctor_get(x_7, 4);
x_86 = lean_ctor_get(x_7, 5);
lean_inc(x_86);
lean_inc(x_85);
lean_inc(x_84);
lean_inc(x_83);
lean_inc(x_82);
lean_inc(x_81);
lean_dec(x_7);
lean_inc_ref(x_6);
x_87 = lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec(x_82, x_6);
x_88 = lean_ctor_get(x_87, 0);
lean_inc(x_88);
if (lean_is_exclusive(x_87)) {
 lean_ctor_release(x_87, 0);
 x_89 = x_87;
} else {
 lean_dec_ref(x_87);
 x_89 = lean_box(0);
}
x_90 = lean_ctor_get(x_88, 0);
lean_inc(x_90);
x_91 = lean_ctor_get(x_88, 1);
lean_inc(x_91);
if (lean_is_exclusive(x_88)) {
 lean_ctor_release(x_88, 0);
 lean_ctor_release(x_88, 1);
 x_92 = x_88;
} else {
 lean_dec_ref(x_88);
 x_92 = lean_box(0);
}
x_93 = lean_box(0);
x_94 = lp_mvk__specs_MVK_Phase2_Common_instDecidableEqPointer_decEq___redArg(x_90, x_93);
lean_dec(x_90);
if (x_94 == 0)
{
lean_object* x_95; uint8_t x_96; lean_object* x_97; lean_object* x_98; lean_object* x_99; lean_object* x_100; lean_object* x_101; lean_object* x_102; lean_object* x_103; lean_object* x_104; lean_object* x_105; lean_object* x_106; 
lean_inc_ref(x_5);
if (lean_is_exclusive(x_2)) {
 lean_ctor_release(x_2, 0);
 lean_ctor_release(x_2, 1);
 x_95 = x_2;
} else {
 lean_dec_ref(x_2);
 x_95 = lean_box(0);
}
x_96 = 1;
x_97 = lp_mvk__specs_MVK_Phase2_Slab_objects__per__slab(x_81, x_82);
x_98 = lean_unsigned_to_nat(1u);
x_99 = lean_nat_add(x_84, x_98);
lean_dec(x_84);
x_100 = lean_nat_add(x_85, x_97);
lean_dec(x_97);
lean_dec(x_85);
x_101 = lean_alloc_ctor(0, 6, 0);
lean_ctor_set(x_101, 0, x_81);
lean_ctor_set(x_101, 1, x_82);
lean_ctor_set(x_101, 2, x_83);
lean_ctor_set(x_101, 3, x_99);
lean_ctor_set(x_101, 4, x_100);
lean_ctor_set(x_101, 5, x_86);
x_102 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_102, 0, x_1);
lean_closure_set(x_102, 1, x_5);
lean_closure_set(x_102, 2, x_101);
if (lean_is_scalar(x_95)) {
 x_103 = lean_alloc_ctor(0, 2, 1);
} else {
 x_103 = x_95;
}
lean_ctor_set(x_103, 0, x_102);
lean_ctor_set(x_103, 1, x_91);
lean_ctor_set_uint8(x_103, sizeof(void*)*2, x_4);
x_104 = lean_box(x_96);
if (lean_is_scalar(x_92)) {
 x_105 = lean_alloc_ctor(0, 2, 0);
} else {
 x_105 = x_92;
}
lean_ctor_set(x_105, 0, x_104);
lean_ctor_set(x_105, 1, x_103);
if (lean_is_scalar(x_89)) {
 x_106 = lean_alloc_ctor(0, 1, 0);
} else {
 x_106 = x_89;
}
lean_ctor_set(x_106, 0, x_105);
return x_106;
}
else
{
uint8_t x_107; lean_object* x_108; lean_object* x_109; lean_object* x_110; 
lean_dec(x_91);
lean_dec(x_86);
lean_dec(x_85);
lean_dec(x_84);
lean_dec(x_83);
lean_dec(x_82);
lean_dec(x_81);
lean_dec(x_1);
x_107 = 0;
x_108 = lean_box(x_107);
if (lean_is_scalar(x_92)) {
 x_109 = lean_alloc_ctor(0, 2, 0);
} else {
 x_109 = x_92;
}
lean_ctor_set(x_109, 0, x_108);
lean_ctor_set(x_109, 1, x_2);
if (lean_is_scalar(x_89)) {
 x_110 = lean_alloc_ctor(0, 1, 0);
} else {
 x_110 = x_89;
}
lean_ctor_set(x_110, 0, x_109);
return x_110;
}
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec(x_1, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
x_5 = lean_nat_dec_eq(x_4, x_1);
if (x_5 == 0)
{
lean_object* x_6; 
x_6 = lean_apply_1(x_2, x_4);
return x_6;
}
else
{
lean_dec(x_4);
lean_dec_ref(x_2);
lean_inc_ref(x_3);
return x_3;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0(x_1, x_2, x_3, x_4);
lean_dec_ref(x_3);
lean_dec(x_1);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; 
x_5 = lean_nat_dec_eq(x_4, x_1);
if (x_5 == 0)
{
lean_object* x_6; 
x_6 = lean_apply_1(x_2, x_4);
return x_6;
}
else
{
lean_dec(x_4);
lean_dec_ref(x_2);
lean_inc_ref(x_3);
return x_3;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1(x_1, x_2, x_3, x_4);
lean_dec_ref(x_3);
lean_dec(x_1);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_8; lean_object* x_9; uint8_t x_14; lean_object* x_224; uint8_t x_225; 
x_224 = lean_unsigned_to_nat(0u);
x_225 = lean_nat_dec_eq(x_1, x_224);
if (x_225 == 0)
{
lean_object* x_226; uint8_t x_227; 
x_226 = lean_unsigned_to_nat(8192u);
x_227 = lean_nat_dec_lt(x_226, x_1);
x_14 = x_227;
goto block_223;
}
else
{
x_14 = x_225;
goto block_223;
}
block_7:
{
lean_object* x_4; lean_object* x_5; lean_object* x_6; 
x_4 = lean_box(0);
x_5 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_5, 0, x_4);
lean_ctor_set(x_5, 1, x_2);
x_6 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_6, 0, x_5);
return x_6;
}
block_13:
{
lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_10 = lean_box(0);
x_11 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_11, 0, x_10);
lean_ctor_set(x_11, 1, x_8);
x_12 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_12, 0, x_11);
return x_12;
}
block_223:
{
if (x_14 == 0)
{
uint8_t x_15; 
x_15 = lean_ctor_get_uint8(x_2, sizeof(void*)*2);
if (x_15 == 0)
{
lean_dec(x_1);
goto block_7;
}
else
{
lean_object* x_16; lean_object* x_17; lean_object* x_18; 
x_16 = lean_ctor_get(x_2, 0);
x_17 = lean_ctor_get(x_2, 1);
x_18 = lp_mvk__specs_MVK_Phase2_Slab_find__cache__idx(x_1);
if (lean_obj_tag(x_18) == 0)
{
lean_object* x_19; lean_object* x_20; lean_object* x_21; 
x_19 = lean_box(0);
x_20 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_20, 0, x_19);
lean_ctor_set(x_20, 1, x_2);
x_21 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_21, 0, x_20);
return x_21;
}
else
{
uint8_t x_22; 
x_22 = !lean_is_exclusive(x_18);
if (x_22 == 0)
{
lean_object* x_23; lean_object* x_24; uint8_t x_25; 
x_23 = lean_ctor_get(x_18, 0);
lean_inc_ref(x_16);
lean_inc(x_23);
x_24 = lean_apply_1(x_16, x_23);
x_25 = lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(x_24);
if (x_25 == 0)
{
lean_object* x_26; 
lean_dec_ref(x_24);
lean_free_object(x_18);
lean_inc(x_23);
x_26 = lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec(x_23, x_2);
if (lean_obj_tag(x_26) == 0)
{
uint8_t x_27; 
x_27 = !lean_is_exclusive(x_26);
if (x_27 == 0)
{
lean_object* x_28; lean_object* x_29; uint8_t x_30; 
x_28 = lean_ctor_get(x_26, 0);
x_29 = lean_ctor_get(x_28, 0);
x_30 = lean_unbox(x_29);
if (x_30 == 0)
{
lean_object* x_31; 
lean_free_object(x_26);
lean_dec(x_23);
x_31 = lean_ctor_get(x_28, 1);
lean_inc(x_31);
lean_dec(x_28);
x_8 = x_31;
x_9 = lean_box(0);
goto block_13;
}
else
{
if (x_25 == 0)
{
uint8_t x_32; 
x_32 = !lean_is_exclusive(x_28);
if (x_32 == 0)
{
lean_object* x_33; lean_object* x_34; uint8_t x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; uint8_t x_39; 
x_33 = lean_ctor_get(x_28, 1);
x_34 = lean_ctor_get(x_28, 0);
lean_dec(x_34);
x_35 = lean_ctor_get_uint8(x_33, sizeof(void*)*2);
x_36 = lean_ctor_get(x_33, 0);
x_37 = lean_ctor_get(x_33, 1);
lean_inc_ref(x_36);
lean_inc(x_23);
x_38 = lean_apply_1(x_36, x_23);
x_39 = lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(x_38);
if (x_39 == 0)
{
lean_object* x_40; 
lean_dec_ref(x_38);
lean_dec(x_23);
x_40 = lean_box(0);
lean_ctor_set(x_28, 0, x_40);
return x_26;
}
else
{
uint8_t x_41; 
lean_inc_ref(x_37);
lean_inc_ref(x_36);
x_41 = !lean_is_exclusive(x_33);
if (x_41 == 0)
{
lean_object* x_42; lean_object* x_43; uint8_t x_44; 
x_42 = lean_ctor_get(x_33, 1);
lean_dec(x_42);
x_43 = lean_ctor_get(x_33, 0);
lean_dec(x_43);
x_44 = !lean_is_exclusive(x_38);
if (x_44 == 0)
{
lean_object* x_45; lean_object* x_46; lean_object* x_47; lean_object* x_48; lean_object* x_49; 
x_45 = lean_ctor_get(x_38, 5);
x_46 = lean_unsigned_to_nat(1u);
x_47 = lean_nat_add(x_45, x_46);
lean_dec(x_45);
lean_ctor_set(x_38, 5, x_47);
x_48 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_48, 0, x_23);
lean_closure_set(x_48, 1, x_36);
lean_closure_set(x_48, 2, x_38);
x_49 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
lean_ctor_set(x_33, 0, x_48);
lean_ctor_set(x_28, 0, x_49);
return x_26;
}
else
{
lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; lean_object* x_55; lean_object* x_56; lean_object* x_57; lean_object* x_58; lean_object* x_59; lean_object* x_60; 
x_50 = lean_ctor_get(x_38, 0);
x_51 = lean_ctor_get(x_38, 1);
x_52 = lean_ctor_get(x_38, 2);
x_53 = lean_ctor_get(x_38, 3);
x_54 = lean_ctor_get(x_38, 4);
x_55 = lean_ctor_get(x_38, 5);
lean_inc(x_55);
lean_inc(x_54);
lean_inc(x_53);
lean_inc(x_52);
lean_inc(x_51);
lean_inc(x_50);
lean_dec(x_38);
x_56 = lean_unsigned_to_nat(1u);
x_57 = lean_nat_add(x_55, x_56);
lean_dec(x_55);
x_58 = lean_alloc_ctor(0, 6, 0);
lean_ctor_set(x_58, 0, x_50);
lean_ctor_set(x_58, 1, x_51);
lean_ctor_set(x_58, 2, x_52);
lean_ctor_set(x_58, 3, x_53);
lean_ctor_set(x_58, 4, x_54);
lean_ctor_set(x_58, 5, x_57);
x_59 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_59, 0, x_23);
lean_closure_set(x_59, 1, x_36);
lean_closure_set(x_59, 2, x_58);
x_60 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
lean_ctor_set(x_33, 0, x_59);
lean_ctor_set(x_28, 0, x_60);
return x_26;
}
}
else
{
lean_object* x_61; lean_object* x_62; lean_object* x_63; lean_object* x_64; lean_object* x_65; lean_object* x_66; lean_object* x_67; lean_object* x_68; lean_object* x_69; lean_object* x_70; lean_object* x_71; lean_object* x_72; lean_object* x_73; 
lean_dec(x_33);
x_61 = lean_ctor_get(x_38, 0);
lean_inc(x_61);
x_62 = lean_ctor_get(x_38, 1);
lean_inc(x_62);
x_63 = lean_ctor_get(x_38, 2);
lean_inc(x_63);
x_64 = lean_ctor_get(x_38, 3);
lean_inc(x_64);
x_65 = lean_ctor_get(x_38, 4);
lean_inc(x_65);
x_66 = lean_ctor_get(x_38, 5);
lean_inc(x_66);
if (lean_is_exclusive(x_38)) {
 lean_ctor_release(x_38, 0);
 lean_ctor_release(x_38, 1);
 lean_ctor_release(x_38, 2);
 lean_ctor_release(x_38, 3);
 lean_ctor_release(x_38, 4);
 lean_ctor_release(x_38, 5);
 x_67 = x_38;
} else {
 lean_dec_ref(x_38);
 x_67 = lean_box(0);
}
x_68 = lean_unsigned_to_nat(1u);
x_69 = lean_nat_add(x_66, x_68);
lean_dec(x_66);
if (lean_is_scalar(x_67)) {
 x_70 = lean_alloc_ctor(0, 6, 0);
} else {
 x_70 = x_67;
}
lean_ctor_set(x_70, 0, x_61);
lean_ctor_set(x_70, 1, x_62);
lean_ctor_set(x_70, 2, x_63);
lean_ctor_set(x_70, 3, x_64);
lean_ctor_set(x_70, 4, x_65);
lean_ctor_set(x_70, 5, x_69);
x_71 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_71, 0, x_23);
lean_closure_set(x_71, 1, x_36);
lean_closure_set(x_71, 2, x_70);
x_72 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
x_73 = lean_alloc_ctor(0, 2, 1);
lean_ctor_set(x_73, 0, x_71);
lean_ctor_set(x_73, 1, x_37);
lean_ctor_set_uint8(x_73, sizeof(void*)*2, x_35);
lean_ctor_set(x_28, 1, x_73);
lean_ctor_set(x_28, 0, x_72);
return x_26;
}
}
}
else
{
lean_object* x_74; uint8_t x_75; lean_object* x_76; lean_object* x_77; lean_object* x_78; uint8_t x_79; 
x_74 = lean_ctor_get(x_28, 1);
lean_inc(x_74);
lean_dec(x_28);
x_75 = lean_ctor_get_uint8(x_74, sizeof(void*)*2);
x_76 = lean_ctor_get(x_74, 0);
x_77 = lean_ctor_get(x_74, 1);
lean_inc_ref(x_76);
lean_inc(x_23);
x_78 = lean_apply_1(x_76, x_23);
x_79 = lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(x_78);
if (x_79 == 0)
{
lean_object* x_80; lean_object* x_81; 
lean_dec_ref(x_78);
lean_dec(x_23);
x_80 = lean_box(0);
x_81 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_81, 0, x_80);
lean_ctor_set(x_81, 1, x_74);
lean_ctor_set(x_26, 0, x_81);
return x_26;
}
else
{
lean_object* x_82; lean_object* x_83; lean_object* x_84; lean_object* x_85; lean_object* x_86; lean_object* x_87; lean_object* x_88; lean_object* x_89; lean_object* x_90; lean_object* x_91; lean_object* x_92; lean_object* x_93; lean_object* x_94; lean_object* x_95; lean_object* x_96; 
lean_inc_ref(x_77);
lean_inc_ref(x_76);
if (lean_is_exclusive(x_74)) {
 lean_ctor_release(x_74, 0);
 lean_ctor_release(x_74, 1);
 x_82 = x_74;
} else {
 lean_dec_ref(x_74);
 x_82 = lean_box(0);
}
x_83 = lean_ctor_get(x_78, 0);
lean_inc(x_83);
x_84 = lean_ctor_get(x_78, 1);
lean_inc(x_84);
x_85 = lean_ctor_get(x_78, 2);
lean_inc(x_85);
x_86 = lean_ctor_get(x_78, 3);
lean_inc(x_86);
x_87 = lean_ctor_get(x_78, 4);
lean_inc(x_87);
x_88 = lean_ctor_get(x_78, 5);
lean_inc(x_88);
if (lean_is_exclusive(x_78)) {
 lean_ctor_release(x_78, 0);
 lean_ctor_release(x_78, 1);
 lean_ctor_release(x_78, 2);
 lean_ctor_release(x_78, 3);
 lean_ctor_release(x_78, 4);
 lean_ctor_release(x_78, 5);
 x_89 = x_78;
} else {
 lean_dec_ref(x_78);
 x_89 = lean_box(0);
}
x_90 = lean_unsigned_to_nat(1u);
x_91 = lean_nat_add(x_88, x_90);
lean_dec(x_88);
if (lean_is_scalar(x_89)) {
 x_92 = lean_alloc_ctor(0, 6, 0);
} else {
 x_92 = x_89;
}
lean_ctor_set(x_92, 0, x_83);
lean_ctor_set(x_92, 1, x_84);
lean_ctor_set(x_92, 2, x_85);
lean_ctor_set(x_92, 3, x_86);
lean_ctor_set(x_92, 4, x_87);
lean_ctor_set(x_92, 5, x_91);
x_93 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_93, 0, x_23);
lean_closure_set(x_93, 1, x_76);
lean_closure_set(x_93, 2, x_92);
x_94 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
if (lean_is_scalar(x_82)) {
 x_95 = lean_alloc_ctor(0, 2, 1);
} else {
 x_95 = x_82;
}
lean_ctor_set(x_95, 0, x_93);
lean_ctor_set(x_95, 1, x_77);
lean_ctor_set_uint8(x_95, sizeof(void*)*2, x_75);
x_96 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_96, 0, x_94);
lean_ctor_set(x_96, 1, x_95);
lean_ctor_set(x_26, 0, x_96);
return x_26;
}
}
}
else
{
lean_object* x_97; 
lean_free_object(x_26);
lean_dec(x_23);
x_97 = lean_ctor_get(x_28, 1);
lean_inc(x_97);
lean_dec(x_28);
x_8 = x_97;
x_9 = lean_box(0);
goto block_13;
}
}
}
else
{
lean_object* x_98; lean_object* x_99; uint8_t x_100; 
x_98 = lean_ctor_get(x_26, 0);
lean_inc(x_98);
lean_dec(x_26);
x_99 = lean_ctor_get(x_98, 0);
x_100 = lean_unbox(x_99);
if (x_100 == 0)
{
lean_object* x_101; 
lean_dec(x_23);
x_101 = lean_ctor_get(x_98, 1);
lean_inc(x_101);
lean_dec(x_98);
x_8 = x_101;
x_9 = lean_box(0);
goto block_13;
}
else
{
if (x_25 == 0)
{
lean_object* x_102; lean_object* x_103; uint8_t x_104; lean_object* x_105; lean_object* x_106; lean_object* x_107; uint8_t x_108; 
x_102 = lean_ctor_get(x_98, 1);
lean_inc(x_102);
if (lean_is_exclusive(x_98)) {
 lean_ctor_release(x_98, 0);
 lean_ctor_release(x_98, 1);
 x_103 = x_98;
} else {
 lean_dec_ref(x_98);
 x_103 = lean_box(0);
}
x_104 = lean_ctor_get_uint8(x_102, sizeof(void*)*2);
x_105 = lean_ctor_get(x_102, 0);
x_106 = lean_ctor_get(x_102, 1);
lean_inc_ref(x_105);
lean_inc(x_23);
x_107 = lean_apply_1(x_105, x_23);
x_108 = lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(x_107);
if (x_108 == 0)
{
lean_object* x_109; lean_object* x_110; lean_object* x_111; 
lean_dec_ref(x_107);
lean_dec(x_23);
x_109 = lean_box(0);
if (lean_is_scalar(x_103)) {
 x_110 = lean_alloc_ctor(0, 2, 0);
} else {
 x_110 = x_103;
}
lean_ctor_set(x_110, 0, x_109);
lean_ctor_set(x_110, 1, x_102);
x_111 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_111, 0, x_110);
return x_111;
}
else
{
lean_object* x_112; lean_object* x_113; lean_object* x_114; lean_object* x_115; lean_object* x_116; lean_object* x_117; lean_object* x_118; lean_object* x_119; lean_object* x_120; lean_object* x_121; lean_object* x_122; lean_object* x_123; lean_object* x_124; lean_object* x_125; lean_object* x_126; lean_object* x_127; 
lean_inc_ref(x_106);
lean_inc_ref(x_105);
if (lean_is_exclusive(x_102)) {
 lean_ctor_release(x_102, 0);
 lean_ctor_release(x_102, 1);
 x_112 = x_102;
} else {
 lean_dec_ref(x_102);
 x_112 = lean_box(0);
}
x_113 = lean_ctor_get(x_107, 0);
lean_inc(x_113);
x_114 = lean_ctor_get(x_107, 1);
lean_inc(x_114);
x_115 = lean_ctor_get(x_107, 2);
lean_inc(x_115);
x_116 = lean_ctor_get(x_107, 3);
lean_inc(x_116);
x_117 = lean_ctor_get(x_107, 4);
lean_inc(x_117);
x_118 = lean_ctor_get(x_107, 5);
lean_inc(x_118);
if (lean_is_exclusive(x_107)) {
 lean_ctor_release(x_107, 0);
 lean_ctor_release(x_107, 1);
 lean_ctor_release(x_107, 2);
 lean_ctor_release(x_107, 3);
 lean_ctor_release(x_107, 4);
 lean_ctor_release(x_107, 5);
 x_119 = x_107;
} else {
 lean_dec_ref(x_107);
 x_119 = lean_box(0);
}
x_120 = lean_unsigned_to_nat(1u);
x_121 = lean_nat_add(x_118, x_120);
lean_dec(x_118);
if (lean_is_scalar(x_119)) {
 x_122 = lean_alloc_ctor(0, 6, 0);
} else {
 x_122 = x_119;
}
lean_ctor_set(x_122, 0, x_113);
lean_ctor_set(x_122, 1, x_114);
lean_ctor_set(x_122, 2, x_115);
lean_ctor_set(x_122, 3, x_116);
lean_ctor_set(x_122, 4, x_117);
lean_ctor_set(x_122, 5, x_121);
x_123 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_123, 0, x_23);
lean_closure_set(x_123, 1, x_105);
lean_closure_set(x_123, 2, x_122);
x_124 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
if (lean_is_scalar(x_112)) {
 x_125 = lean_alloc_ctor(0, 2, 1);
} else {
 x_125 = x_112;
}
lean_ctor_set(x_125, 0, x_123);
lean_ctor_set(x_125, 1, x_106);
lean_ctor_set_uint8(x_125, sizeof(void*)*2, x_104);
if (lean_is_scalar(x_103)) {
 x_126 = lean_alloc_ctor(0, 2, 0);
} else {
 x_126 = x_103;
}
lean_ctor_set(x_126, 0, x_124);
lean_ctor_set(x_126, 1, x_125);
x_127 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_127, 0, x_126);
return x_127;
}
}
else
{
lean_object* x_128; 
lean_dec(x_23);
x_128 = lean_ctor_get(x_98, 1);
lean_inc(x_128);
lean_dec(x_98);
x_8 = x_128;
x_9 = lean_box(0);
goto block_13;
}
}
}
}
else
{
uint8_t x_129; 
lean_dec(x_23);
x_129 = !lean_is_exclusive(x_26);
if (x_129 == 0)
{
return x_26;
}
else
{
lean_object* x_130; lean_object* x_131; 
x_130 = lean_ctor_get(x_26, 0);
lean_inc(x_130);
lean_dec(x_26);
x_131 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_131, 0, x_130);
return x_131;
}
}
}
else
{
uint8_t x_132; 
lean_inc_ref(x_17);
lean_inc_ref(x_16);
x_132 = !lean_is_exclusive(x_2);
if (x_132 == 0)
{
lean_object* x_133; lean_object* x_134; uint8_t x_135; 
x_133 = lean_ctor_get(x_2, 1);
lean_dec(x_133);
x_134 = lean_ctor_get(x_2, 0);
lean_dec(x_134);
x_135 = !lean_is_exclusive(x_24);
if (x_135 == 0)
{
lean_object* x_136; lean_object* x_137; lean_object* x_138; lean_object* x_139; lean_object* x_140; lean_object* x_141; 
x_136 = lean_ctor_get(x_24, 5);
x_137 = lean_unsigned_to_nat(1u);
x_138 = lean_nat_add(x_136, x_137);
lean_dec(x_136);
lean_ctor_set(x_24, 5, x_138);
x_139 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1___boxed), 4, 3);
lean_closure_set(x_139, 0, x_23);
lean_closure_set(x_139, 1, x_16);
lean_closure_set(x_139, 2, x_24);
x_140 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
lean_ctor_set(x_2, 0, x_139);
x_141 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_141, 0, x_140);
lean_ctor_set(x_141, 1, x_2);
lean_ctor_set_tag(x_18, 0);
lean_ctor_set(x_18, 0, x_141);
return x_18;
}
else
{
lean_object* x_142; lean_object* x_143; lean_object* x_144; lean_object* x_145; lean_object* x_146; lean_object* x_147; lean_object* x_148; lean_object* x_149; lean_object* x_150; lean_object* x_151; lean_object* x_152; lean_object* x_153; 
x_142 = lean_ctor_get(x_24, 0);
x_143 = lean_ctor_get(x_24, 1);
x_144 = lean_ctor_get(x_24, 2);
x_145 = lean_ctor_get(x_24, 3);
x_146 = lean_ctor_get(x_24, 4);
x_147 = lean_ctor_get(x_24, 5);
lean_inc(x_147);
lean_inc(x_146);
lean_inc(x_145);
lean_inc(x_144);
lean_inc(x_143);
lean_inc(x_142);
lean_dec(x_24);
x_148 = lean_unsigned_to_nat(1u);
x_149 = lean_nat_add(x_147, x_148);
lean_dec(x_147);
x_150 = lean_alloc_ctor(0, 6, 0);
lean_ctor_set(x_150, 0, x_142);
lean_ctor_set(x_150, 1, x_143);
lean_ctor_set(x_150, 2, x_144);
lean_ctor_set(x_150, 3, x_145);
lean_ctor_set(x_150, 4, x_146);
lean_ctor_set(x_150, 5, x_149);
x_151 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1___boxed), 4, 3);
lean_closure_set(x_151, 0, x_23);
lean_closure_set(x_151, 1, x_16);
lean_closure_set(x_151, 2, x_150);
x_152 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
lean_ctor_set(x_2, 0, x_151);
x_153 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_153, 0, x_152);
lean_ctor_set(x_153, 1, x_2);
lean_ctor_set_tag(x_18, 0);
lean_ctor_set(x_18, 0, x_153);
return x_18;
}
}
else
{
lean_object* x_154; lean_object* x_155; lean_object* x_156; lean_object* x_157; lean_object* x_158; lean_object* x_159; lean_object* x_160; lean_object* x_161; lean_object* x_162; lean_object* x_163; lean_object* x_164; lean_object* x_165; lean_object* x_166; lean_object* x_167; 
lean_dec(x_2);
x_154 = lean_ctor_get(x_24, 0);
lean_inc(x_154);
x_155 = lean_ctor_get(x_24, 1);
lean_inc(x_155);
x_156 = lean_ctor_get(x_24, 2);
lean_inc(x_156);
x_157 = lean_ctor_get(x_24, 3);
lean_inc(x_157);
x_158 = lean_ctor_get(x_24, 4);
lean_inc(x_158);
x_159 = lean_ctor_get(x_24, 5);
lean_inc(x_159);
if (lean_is_exclusive(x_24)) {
 lean_ctor_release(x_24, 0);
 lean_ctor_release(x_24, 1);
 lean_ctor_release(x_24, 2);
 lean_ctor_release(x_24, 3);
 lean_ctor_release(x_24, 4);
 lean_ctor_release(x_24, 5);
 x_160 = x_24;
} else {
 lean_dec_ref(x_24);
 x_160 = lean_box(0);
}
x_161 = lean_unsigned_to_nat(1u);
x_162 = lean_nat_add(x_159, x_161);
lean_dec(x_159);
if (lean_is_scalar(x_160)) {
 x_163 = lean_alloc_ctor(0, 6, 0);
} else {
 x_163 = x_160;
}
lean_ctor_set(x_163, 0, x_154);
lean_ctor_set(x_163, 1, x_155);
lean_ctor_set(x_163, 2, x_156);
lean_ctor_set(x_163, 3, x_157);
lean_ctor_set(x_163, 4, x_158);
lean_ctor_set(x_163, 5, x_162);
x_164 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1___boxed), 4, 3);
lean_closure_set(x_164, 0, x_23);
lean_closure_set(x_164, 1, x_16);
lean_closure_set(x_164, 2, x_163);
x_165 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
x_166 = lean_alloc_ctor(0, 2, 1);
lean_ctor_set(x_166, 0, x_164);
lean_ctor_set(x_166, 1, x_17);
lean_ctor_set_uint8(x_166, sizeof(void*)*2, x_15);
x_167 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_167, 0, x_165);
lean_ctor_set(x_167, 1, x_166);
lean_ctor_set_tag(x_18, 0);
lean_ctor_set(x_18, 0, x_167);
return x_18;
}
}
}
else
{
lean_object* x_168; lean_object* x_169; uint8_t x_170; 
x_168 = lean_ctor_get(x_18, 0);
lean_inc(x_168);
lean_dec(x_18);
lean_inc_ref(x_16);
lean_inc(x_168);
x_169 = lean_apply_1(x_16, x_168);
x_170 = lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(x_169);
if (x_170 == 0)
{
lean_object* x_171; 
lean_dec_ref(x_169);
lean_inc(x_168);
x_171 = lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__grow__spec(x_168, x_2);
if (lean_obj_tag(x_171) == 0)
{
lean_object* x_172; lean_object* x_173; lean_object* x_174; uint8_t x_175; 
x_172 = lean_ctor_get(x_171, 0);
lean_inc(x_172);
if (lean_is_exclusive(x_171)) {
 lean_ctor_release(x_171, 0);
 x_173 = x_171;
} else {
 lean_dec_ref(x_171);
 x_173 = lean_box(0);
}
x_174 = lean_ctor_get(x_172, 0);
x_175 = lean_unbox(x_174);
if (x_175 == 0)
{
lean_object* x_176; 
lean_dec(x_173);
lean_dec(x_168);
x_176 = lean_ctor_get(x_172, 1);
lean_inc(x_176);
lean_dec(x_172);
x_8 = x_176;
x_9 = lean_box(0);
goto block_13;
}
else
{
if (x_170 == 0)
{
lean_object* x_177; lean_object* x_178; uint8_t x_179; lean_object* x_180; lean_object* x_181; lean_object* x_182; uint8_t x_183; 
x_177 = lean_ctor_get(x_172, 1);
lean_inc(x_177);
if (lean_is_exclusive(x_172)) {
 lean_ctor_release(x_172, 0);
 lean_ctor_release(x_172, 1);
 x_178 = x_172;
} else {
 lean_dec_ref(x_172);
 x_178 = lean_box(0);
}
x_179 = lean_ctor_get_uint8(x_177, sizeof(void*)*2);
x_180 = lean_ctor_get(x_177, 0);
x_181 = lean_ctor_get(x_177, 1);
lean_inc_ref(x_180);
lean_inc(x_168);
x_182 = lean_apply_1(x_180, x_168);
x_183 = lp_mvk__specs_MVK_Phase2_Slab_KmemCache_hasFree(x_182);
if (x_183 == 0)
{
lean_object* x_184; lean_object* x_185; lean_object* x_186; 
lean_dec_ref(x_182);
lean_dec(x_168);
x_184 = lean_box(0);
if (lean_is_scalar(x_178)) {
 x_185 = lean_alloc_ctor(0, 2, 0);
} else {
 x_185 = x_178;
}
lean_ctor_set(x_185, 0, x_184);
lean_ctor_set(x_185, 1, x_177);
if (lean_is_scalar(x_173)) {
 x_186 = lean_alloc_ctor(0, 1, 0);
} else {
 x_186 = x_173;
}
lean_ctor_set(x_186, 0, x_185);
return x_186;
}
else
{
lean_object* x_187; lean_object* x_188; lean_object* x_189; lean_object* x_190; lean_object* x_191; lean_object* x_192; lean_object* x_193; lean_object* x_194; lean_object* x_195; lean_object* x_196; lean_object* x_197; lean_object* x_198; lean_object* x_199; lean_object* x_200; lean_object* x_201; lean_object* x_202; 
lean_inc_ref(x_181);
lean_inc_ref(x_180);
if (lean_is_exclusive(x_177)) {
 lean_ctor_release(x_177, 0);
 lean_ctor_release(x_177, 1);
 x_187 = x_177;
} else {
 lean_dec_ref(x_177);
 x_187 = lean_box(0);
}
x_188 = lean_ctor_get(x_182, 0);
lean_inc(x_188);
x_189 = lean_ctor_get(x_182, 1);
lean_inc(x_189);
x_190 = lean_ctor_get(x_182, 2);
lean_inc(x_190);
x_191 = lean_ctor_get(x_182, 3);
lean_inc(x_191);
x_192 = lean_ctor_get(x_182, 4);
lean_inc(x_192);
x_193 = lean_ctor_get(x_182, 5);
lean_inc(x_193);
if (lean_is_exclusive(x_182)) {
 lean_ctor_release(x_182, 0);
 lean_ctor_release(x_182, 1);
 lean_ctor_release(x_182, 2);
 lean_ctor_release(x_182, 3);
 lean_ctor_release(x_182, 4);
 lean_ctor_release(x_182, 5);
 x_194 = x_182;
} else {
 lean_dec_ref(x_182);
 x_194 = lean_box(0);
}
x_195 = lean_unsigned_to_nat(1u);
x_196 = lean_nat_add(x_193, x_195);
lean_dec(x_193);
if (lean_is_scalar(x_194)) {
 x_197 = lean_alloc_ctor(0, 6, 0);
} else {
 x_197 = x_194;
}
lean_ctor_set(x_197, 0, x_188);
lean_ctor_set(x_197, 1, x_189);
lean_ctor_set(x_197, 2, x_190);
lean_ctor_set(x_197, 3, x_191);
lean_ctor_set(x_197, 4, x_192);
lean_ctor_set(x_197, 5, x_196);
x_198 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__0___boxed), 4, 3);
lean_closure_set(x_198, 0, x_168);
lean_closure_set(x_198, 1, x_180);
lean_closure_set(x_198, 2, x_197);
x_199 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
if (lean_is_scalar(x_187)) {
 x_200 = lean_alloc_ctor(0, 2, 1);
} else {
 x_200 = x_187;
}
lean_ctor_set(x_200, 0, x_198);
lean_ctor_set(x_200, 1, x_181);
lean_ctor_set_uint8(x_200, sizeof(void*)*2, x_179);
if (lean_is_scalar(x_178)) {
 x_201 = lean_alloc_ctor(0, 2, 0);
} else {
 x_201 = x_178;
}
lean_ctor_set(x_201, 0, x_199);
lean_ctor_set(x_201, 1, x_200);
if (lean_is_scalar(x_173)) {
 x_202 = lean_alloc_ctor(0, 1, 0);
} else {
 x_202 = x_173;
}
lean_ctor_set(x_202, 0, x_201);
return x_202;
}
}
else
{
lean_object* x_203; 
lean_dec(x_173);
lean_dec(x_168);
x_203 = lean_ctor_get(x_172, 1);
lean_inc(x_203);
lean_dec(x_172);
x_8 = x_203;
x_9 = lean_box(0);
goto block_13;
}
}
}
else
{
lean_object* x_204; lean_object* x_205; lean_object* x_206; 
lean_dec(x_168);
x_204 = lean_ctor_get(x_171, 0);
lean_inc(x_204);
if (lean_is_exclusive(x_171)) {
 lean_ctor_release(x_171, 0);
 x_205 = x_171;
} else {
 lean_dec_ref(x_171);
 x_205 = lean_box(0);
}
if (lean_is_scalar(x_205)) {
 x_206 = lean_alloc_ctor(1, 1, 0);
} else {
 x_206 = x_205;
}
lean_ctor_set(x_206, 0, x_204);
return x_206;
}
}
else
{
lean_object* x_207; lean_object* x_208; lean_object* x_209; lean_object* x_210; lean_object* x_211; lean_object* x_212; lean_object* x_213; lean_object* x_214; lean_object* x_215; lean_object* x_216; lean_object* x_217; lean_object* x_218; lean_object* x_219; lean_object* x_220; lean_object* x_221; lean_object* x_222; 
lean_inc_ref(x_17);
lean_inc_ref(x_16);
if (lean_is_exclusive(x_2)) {
 lean_ctor_release(x_2, 0);
 lean_ctor_release(x_2, 1);
 x_207 = x_2;
} else {
 lean_dec_ref(x_2);
 x_207 = lean_box(0);
}
x_208 = lean_ctor_get(x_169, 0);
lean_inc(x_208);
x_209 = lean_ctor_get(x_169, 1);
lean_inc(x_209);
x_210 = lean_ctor_get(x_169, 2);
lean_inc(x_210);
x_211 = lean_ctor_get(x_169, 3);
lean_inc(x_211);
x_212 = lean_ctor_get(x_169, 4);
lean_inc(x_212);
x_213 = lean_ctor_get(x_169, 5);
lean_inc(x_213);
if (lean_is_exclusive(x_169)) {
 lean_ctor_release(x_169, 0);
 lean_ctor_release(x_169, 1);
 lean_ctor_release(x_169, 2);
 lean_ctor_release(x_169, 3);
 lean_ctor_release(x_169, 4);
 lean_ctor_release(x_169, 5);
 x_214 = x_169;
} else {
 lean_dec_ref(x_169);
 x_214 = lean_box(0);
}
x_215 = lean_unsigned_to_nat(1u);
x_216 = lean_nat_add(x_213, x_215);
lean_dec(x_213);
if (lean_is_scalar(x_214)) {
 x_217 = lean_alloc_ctor(0, 6, 0);
} else {
 x_217 = x_214;
}
lean_ctor_set(x_217, 0, x_208);
lean_ctor_set(x_217, 1, x_209);
lean_ctor_set(x_217, 2, x_210);
lean_ctor_set(x_217, 3, x_211);
lean_ctor_set(x_217, 4, x_212);
lean_ctor_set(x_217, 5, x_216);
x_218 = lean_alloc_closure((void*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___lam__1___boxed), 4, 3);
lean_closure_set(x_218, 0, x_168);
lean_closure_set(x_218, 1, x_16);
lean_closure_set(x_218, 2, x_217);
x_219 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___closed__0));
if (lean_is_scalar(x_207)) {
 x_220 = lean_alloc_ctor(0, 2, 1);
} else {
 x_220 = x_207;
}
lean_ctor_set(x_220, 0, x_218);
lean_ctor_set(x_220, 1, x_17);
lean_ctor_set_uint8(x_220, sizeof(void*)*2, x_15);
x_221 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_221, 0, x_219);
lean_ctor_set(x_221, 1, x_220);
x_222 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_222, 0, x_221);
return x_222;
}
}
}
}
}
else
{
lean_dec(x_1);
goto block_7;
}
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec(x_1, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kfree__spec___redArg(lean_object* x_1) {
_start:
{
lean_object* x_3; 
x_3 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_3, 0, x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kfree__spec___redArg___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_kfree__spec___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kfree__spec(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_4; 
x_4 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_4, 0, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kfree__spec___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = lp_mvk__specs_MVK_Phase2_Slab_kfree__spec(x_1, x_2);
lean_dec(x_1);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kzalloc__spec(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_4; 
x_4 = lp_mvk__specs_MVK_Phase2_Slab_kmalloc__spec(x_1, x_2);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kzalloc__spec___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = lp_mvk__specs_MVK_Phase2_Slab_kzalloc__spec(x_1, x_2);
return x_4;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__stat__spec___closed__0(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_Slab_instReprSlabObject_repr_spec__0___closed__3);
x_2 = lean_int_neg(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__stat__spec(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; uint8_t x_4; 
x_3 = lean_unsigned_to_nat(8u);
x_4 = lean_nat_dec_lt(x_1, x_3);
if (x_4 == 0)
{
lean_object* x_5; 
lean_dec_ref(x_2);
lean_dec(x_1);
x_5 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__stat__spec___closed__0, &lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__stat__spec___closed__0_once, _init_lp_mvk__specs_MVK_Phase2_Slab_kmem__cache__stat__spec___closed__0);
return x_5;
}
else
{
lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; 
x_6 = lean_ctor_get(x_2, 0);
lean_inc_ref(x_6);
lean_dec_ref(x_2);
x_7 = lean_apply_1(x_6, x_1);
x_8 = lean_ctor_get(x_7, 5);
lean_inc(x_8);
lean_dec_ref(x_7);
x_9 = lean_nat_to_int(x_8);
return x_9;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__exit__spec(lean_object* x_1) {
_start:
{
uint8_t x_3; 
x_3 = lean_ctor_get_uint8(x_1, sizeof(void*)*2);
if (x_3 == 0)
{
lean_object* x_4; 
x_4 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_4, 0, x_1);
return x_4;
}
else
{
lean_object* x_5; lean_object* x_6; 
lean_dec_ref(x_1);
x_5 = lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state;
x_6 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_6, 0, x_5);
return x_6;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Slab_slab__exit__spec___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Slab_slab__exit__spec(x_1);
return x_3;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_mvk__specs_MVK_Phase2_Common(uint8_t builtin);
lean_object* initialize_mvk__specs_MVK_Phase2_PageAlloc(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_mvk__specs_MVK_Phase2_Slab(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mvk__specs_MVK_Phase2_Common(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mvk__specs_MVK_Phase2_PageAlloc(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MIN__SIZE = _init_lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MIN__SIZE();
lean_mark_persistent(lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MIN__SIZE);
lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MAX__SIZE = _init_lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MAX__SIZE();
lean_mark_persistent(lp_mvk__specs_MVK_Phase2_Slab_KMALLOC__MAX__SIZE);
lp_mvk__specs_MVK_Phase2_Slab_NUM__CACHES = _init_lp_mvk__specs_MVK_Phase2_Slab_NUM__CACHES();
lean_mark_persistent(lp_mvk__specs_MVK_Phase2_Slab_NUM__CACHES);
lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state = _init_lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state();
lean_mark_persistent(lp_mvk__specs_MVK_Phase2_Slab_initial__slab__state);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
