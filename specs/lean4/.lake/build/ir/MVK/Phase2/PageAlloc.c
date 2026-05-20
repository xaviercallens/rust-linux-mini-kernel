// Lean compiler output
// Module: MVK.Phase2.PageAlloc
// Imports: public import Init public import MVK.Phase2.Common
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
static const lean_string_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 31, .m_capacity = 31, .m_length = 30, .m_data = "MVK.Phase2.Common.Pointer.null"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__1_value;
lean_object* lean_nat_to_int(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3;
static const lean_string_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 32, .m_capacity = 32, .m_length = 31, .m_data = "MVK.Phase2.Common.Pointer.valid"};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__5_value),((lean_object*)(((size_t)(1) << 1) | 1))}};
static const lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__6_value;
lean_object* l_Repr_addAppParen(lean_object*, lean_object*);
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
lean_object* l_Nat_reprFast(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___boxed(lean_object*, lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__0_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "flags"};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__2_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__3_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__3_value),((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__6_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7;
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = ","};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__8 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__8_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__8_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__9 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__9_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "count"};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__10 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__10_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__10_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__11 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__11_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__12_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "order"};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__12 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__12_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__13_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__12_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__13 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__13_value;
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__14_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "next"};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__14 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__14_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__15_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__14_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__15 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__15_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16;
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__17_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__17 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__17_value;
lean_object* lean_string_length(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__18_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__18;
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__20_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__20 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__20_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__21_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__17_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__21 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__21_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPageList_repr_spec__0(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPageList_repr_spec__0___boxed(lean_object*, lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "head"};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__2_value),((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__3_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*4 + 0, .m_other = 4, .m_tag = 0}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page__list___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 0}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page__list___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page__list___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page__list = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page__list___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___lam__0(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___lam__0___boxed(lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___lam__0___boxed, .m_arity = 1, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___lam__0(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___lam__0___boxed(lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___lam__0___boxed, .m_arity = 1, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*3 + 8, .m_other = 3, .m_tag = 0}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___closed__0_value),((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__0_value),LEAN_SCALAR_PTR_LITERAL(0, 0, 0, 0, 0, 0, 0, 0)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__1_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__1_value;
lean_object* lean_nat_shiftl(lean_object*, lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree___closed__0;
lean_object* lean_nat_land(lean_object*, lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree___boxed(lean_object*);
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isAllocated(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isAllocated___boxed(lean_object*);
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*3 + 8, .m_other = 3, .m_tag = 0}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___closed__0_value),((lean_object*)(((size_t)(32768) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__0_value),LEAN_SCALAR_PTR_LITERAL(1, 0, 0, 0, 0, 0, 0, 0)}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec___boxed(lean_object*, lean_object*);
static const lean_ctor_object lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 1}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1))}};
static const lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec___closed__0_value;
lean_object* lp_mvk__specs_MVK_Phase2_Common_pages__for__order(lean_object*);
lean_object* lean_nat_sub(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec___boxed(lean_object*, lean_object*, lean_object*);
lean_object* lean_nat_add(lean_object*, lean_object*);
uint8_t lp_mvk__specs_MVK_Phase2_Common_instDecidableEqPointer_decEq___redArg(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_free__pages__spec(lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_free__pages__spec___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_nr__free__pages__spec(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_nr__free__pages__spec___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__size__spec;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__exit__spec(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__exit__spec___boxed(lean_object*);
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(2u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0(lean_object* x_1, lean_object* x_2) {
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
x_12 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2);
x_3 = x_12;
goto block_9;
}
else
{
lean_object* x_13; 
x_13 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3);
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
x_28 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2);
x_16 = x_28;
goto block_25;
}
else
{
lean_object* x_29; 
x_29 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3);
x_16 = x_29;
goto block_25;
}
block_25:
{
lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; uint8_t x_22; lean_object* x_23; lean_object* x_24; 
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__6));
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
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__1));
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
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(9u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(8u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__18(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__0));
x_2 = lean_string_length(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__18, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__18_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__18);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; uint8_t x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; lean_object* x_32; lean_object* x_33; lean_object* x_34; lean_object* x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; lean_object* x_39; lean_object* x_40; lean_object* x_41; lean_object* x_42; lean_object* x_43; lean_object* x_44; lean_object* x_45; lean_object* x_46; lean_object* x_47; lean_object* x_48; lean_object* x_49; lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; 
x_2 = lean_ctor_get(x_1, 0);
lean_inc(x_2);
x_3 = lean_ctor_get(x_1, 1);
lean_inc(x_3);
x_4 = lean_ctor_get(x_1, 2);
lean_inc(x_4);
x_5 = lean_ctor_get(x_1, 3);
lean_inc(x_5);
lean_dec_ref(x_1);
x_6 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__5));
x_7 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__6));
x_8 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7);
x_9 = l_Nat_reprFast(x_2);
x_10 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_10, 0, x_9);
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
x_15 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__9));
x_16 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_16, 0, x_14);
lean_ctor_set(x_16, 1, x_15);
x_17 = lean_box(1);
x_18 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_18, 0, x_16);
lean_ctor_set(x_18, 1, x_17);
x_19 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__11));
x_20 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_20, 0, x_18);
lean_ctor_set(x_20, 1, x_19);
x_21 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_21, 0, x_20);
lean_ctor_set(x_21, 1, x_6);
x_22 = l_Nat_reprFast(x_3);
x_23 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_23, 0, x_22);
x_24 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_24, 0, x_8);
lean_ctor_set(x_24, 1, x_23);
x_25 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_25, 0, x_24);
lean_ctor_set_uint8(x_25, sizeof(void*)*1, x_12);
x_26 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_26, 0, x_21);
lean_ctor_set(x_26, 1, x_25);
x_27 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_27, 0, x_26);
lean_ctor_set(x_27, 1, x_15);
x_28 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_28, 0, x_27);
lean_ctor_set(x_28, 1, x_17);
x_29 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__13));
x_30 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_30, 0, x_28);
lean_ctor_set(x_30, 1, x_29);
x_31 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_31, 0, x_30);
lean_ctor_set(x_31, 1, x_6);
x_32 = l_Nat_reprFast(x_4);
x_33 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_33, 0, x_32);
x_34 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_34, 0, x_8);
lean_ctor_set(x_34, 1, x_33);
x_35 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_35, 0, x_34);
lean_ctor_set_uint8(x_35, sizeof(void*)*1, x_12);
x_36 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_36, 0, x_31);
lean_ctor_set(x_36, 1, x_35);
x_37 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_37, 0, x_36);
lean_ctor_set(x_37, 1, x_15);
x_38 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_38, 0, x_37);
lean_ctor_set(x_38, 1, x_17);
x_39 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__15));
x_40 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_40, 0, x_38);
lean_ctor_set(x_40, 1, x_39);
x_41 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_41, 0, x_40);
lean_ctor_set(x_41, 1, x_6);
x_42 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16);
x_43 = lean_unsigned_to_nat(0u);
x_44 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0(x_5, x_43);
x_45 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_45, 0, x_42);
lean_ctor_set(x_45, 1, x_44);
x_46 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_46, 0, x_45);
lean_ctor_set_uint8(x_46, sizeof(void*)*1, x_12);
x_47 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_47, 0, x_41);
lean_ctor_set(x_47, 1, x_46);
x_48 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19);
x_49 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__20));
x_50 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_50, 0, x_49);
lean_ctor_set(x_50, 1, x_47);
x_51 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__21));
x_52 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_52, 0, x_50);
lean_ctor_set(x_52, 1, x_51);
x_53 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_53, 0, x_48);
lean_ctor_set(x_53, 1, x_52);
x_54 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_54, 0, x_53);
lean_ctor_set_uint8(x_54, sizeof(void*)*1, x_12);
return x_54;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPageList_repr_spec__0(lean_object* x_1, lean_object* x_2) {
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
x_12 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2);
x_3 = x_12;
goto block_9;
}
else
{
lean_object* x_13; 
x_13 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3);
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
x_28 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__2);
x_16 = x_28;
goto block_25;
}
else
{
lean_object* x_29; 
x_29 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3, &lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3_once, _init_lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__3);
x_16 = x_29;
goto block_25;
}
block_25:
{
lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; uint8_t x_22; lean_object* x_23; lean_object* x_24; 
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__6));
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
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPage_repr_spec__0___closed__1));
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
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPageList_repr_spec__0___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPageList_repr_spec__0(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg(lean_object* x_1) {
_start:
{
uint8_t x_2; 
x_2 = !lean_is_exclusive(x_1);
if (x_2 == 0)
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; uint8_t x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; lean_object* x_32; 
x_3 = lean_ctor_get(x_1, 0);
x_4 = lean_ctor_get(x_1, 1);
x_5 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__5));
x_6 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__3));
x_7 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16);
x_8 = lean_unsigned_to_nat(0u);
x_9 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPageList_repr_spec__0(x_3, x_8);
lean_ctor_set_tag(x_1, 4);
lean_ctor_set(x_1, 1, x_9);
lean_ctor_set(x_1, 0, x_7);
x_10 = 0;
x_11 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_11, 0, x_1);
lean_ctor_set_uint8(x_11, sizeof(void*)*1, x_10);
x_12 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_12, 0, x_6);
lean_ctor_set(x_12, 1, x_11);
x_13 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__9));
x_14 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_14, 0, x_12);
lean_ctor_set(x_14, 1, x_13);
x_15 = lean_box(1);
x_16 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_16, 0, x_14);
lean_ctor_set(x_16, 1, x_15);
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__11));
x_18 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_18, 0, x_16);
lean_ctor_set(x_18, 1, x_17);
x_19 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_19, 0, x_18);
lean_ctor_set(x_19, 1, x_5);
x_20 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7);
x_21 = l_Nat_reprFast(x_4);
x_22 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_22, 0, x_21);
x_23 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_23, 0, x_20);
lean_ctor_set(x_23, 1, x_22);
x_24 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_24, 0, x_23);
lean_ctor_set_uint8(x_24, sizeof(void*)*1, x_10);
x_25 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_25, 0, x_19);
lean_ctor_set(x_25, 1, x_24);
x_26 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19);
x_27 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__20));
x_28 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_28, 0, x_27);
lean_ctor_set(x_28, 1, x_25);
x_29 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__21));
x_30 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_30, 0, x_28);
lean_ctor_set(x_30, 1, x_29);
x_31 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_31, 0, x_26);
lean_ctor_set(x_31, 1, x_30);
x_32 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_32, 0, x_31);
lean_ctor_set_uint8(x_32, sizeof(void*)*1, x_10);
return x_32;
}
else
{
lean_object* x_33; lean_object* x_34; lean_object* x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; lean_object* x_39; lean_object* x_40; uint8_t x_41; lean_object* x_42; lean_object* x_43; lean_object* x_44; lean_object* x_45; lean_object* x_46; lean_object* x_47; lean_object* x_48; lean_object* x_49; lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; lean_object* x_55; lean_object* x_56; lean_object* x_57; lean_object* x_58; lean_object* x_59; lean_object* x_60; lean_object* x_61; lean_object* x_62; lean_object* x_63; 
x_33 = lean_ctor_get(x_1, 0);
x_34 = lean_ctor_get(x_1, 1);
lean_inc(x_34);
lean_inc(x_33);
lean_dec(x_1);
x_35 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__5));
x_36 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg___closed__3));
x_37 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__16);
x_38 = lean_unsigned_to_nat(0u);
x_39 = lp_mvk__specs_MVK_Phase2_Common_instReprPointer_repr___at___00MVK_Phase2_PageAlloc_instReprPageList_repr_spec__0(x_33, x_38);
x_40 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_40, 0, x_37);
lean_ctor_set(x_40, 1, x_39);
x_41 = 0;
x_42 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_42, 0, x_40);
lean_ctor_set_uint8(x_42, sizeof(void*)*1, x_41);
x_43 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_43, 0, x_36);
lean_ctor_set(x_43, 1, x_42);
x_44 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__9));
x_45 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_45, 0, x_43);
lean_ctor_set(x_45, 1, x_44);
x_46 = lean_box(1);
x_47 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_47, 0, x_45);
lean_ctor_set(x_47, 1, x_46);
x_48 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__11));
x_49 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_49, 0, x_47);
lean_ctor_set(x_49, 1, x_48);
x_50 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_50, 0, x_49);
lean_ctor_set(x_50, 1, x_35);
x_51 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__7);
x_52 = l_Nat_reprFast(x_34);
x_53 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_53, 0, x_52);
x_54 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_54, 0, x_51);
lean_ctor_set(x_54, 1, x_53);
x_55 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_55, 0, x_54);
lean_ctor_set_uint8(x_55, sizeof(void*)*1, x_41);
x_56 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_56, 0, x_50);
lean_ctor_set(x_56, 1, x_55);
x_57 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19, &lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__19);
x_58 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__20));
x_59 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_59, 0, x_58);
lean_ctor_set(x_59, 1, x_56);
x_60 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPage_repr___redArg___closed__21));
x_61 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_61, 0, x_59);
lean_ctor_set(x_61, 1, x_60);
x_62 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_62, 0, x_57);
lean_ctor_set(x_62, 1, x_61);
x_63 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_63, 0, x_62);
lean_ctor_set_uint8(x_63, sizeof(void*)*1, x_41);
return x_63;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_PageAlloc_instReprPageList_repr(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___lam__0(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page__list));
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___lam__0___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_PageAlloc_initial__free__area___lam__0(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___lam__0(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_initial__page));
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___lam__0___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___lam__0(x_1);
lean_dec(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree___closed__0(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_nat_shiftl(x_1, x_1);
return x_2;
}
}
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree(lean_object* x_1) {
_start:
{
lean_object* x_2; lean_object* x_3; lean_object* x_4; lean_object* x_5; uint8_t x_6; 
x_2 = lean_ctor_get(x_1, 0);
x_3 = lean_obj_once(&lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree___closed__0, &lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree___closed__0_once, _init_lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree___closed__0);
x_4 = lean_nat_land(x_2, x_3);
x_5 = lean_unsigned_to_nat(0u);
x_6 = lean_nat_dec_eq(x_4, x_5);
lean_dec(x_4);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree(x_1);
lean_dec_ref(x_1);
x_3 = lean_box(x_2);
return x_3;
}
}
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isAllocated(lean_object* x_1) {
_start:
{
uint8_t x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isFree(x_1);
if (x_2 == 0)
{
uint8_t x_3; 
x_3 = 1;
return x_3;
}
else
{
uint8_t x_4; 
x_4 = 0;
return x_4;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isAllocated___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lp_mvk__specs_MVK_Phase2_PageAlloc_Page_isAllocated(x_1);
lean_dec_ref(x_1);
x_3 = lean_box(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec(lean_object* x_1) {
_start:
{
uint8_t x_3; 
x_3 = lean_ctor_get_uint8(x_1, sizeof(void*)*3);
if (x_3 == 0)
{
lean_object* x_4; lean_object* x_5; 
lean_dec_ref(x_1);
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec___closed__0));
x_5 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_5, 0, x_4);
return x_5;
}
else
{
lean_object* x_6; 
x_6 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_6, 0, x_1);
return x_6;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__init__spec(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_8; 
x_8 = lean_ctor_get_uint8(x_2, sizeof(void*)*3);
if (x_8 == 0)
{
goto block_7;
}
else
{
lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; uint8_t x_13; 
x_9 = lean_ctor_get(x_2, 0);
x_10 = lean_ctor_get(x_2, 1);
x_11 = lean_ctor_get(x_2, 2);
x_12 = lean_unsigned_to_nat(11u);
x_13 = lean_nat_dec_le(x_12, x_1);
if (x_13 == 0)
{
lean_object* x_14; uint8_t x_15; 
x_14 = lp_mvk__specs_MVK_Phase2_Common_pages__for__order(x_1);
x_15 = lean_nat_dec_le(x_14, x_10);
if (x_15 == 0)
{
lean_object* x_16; lean_object* x_17; lean_object* x_18; 
lean_dec(x_14);
x_16 = lean_box(0);
x_17 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_17, 0, x_16);
lean_ctor_set(x_17, 1, x_2);
x_18 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_18, 0, x_17);
return x_18;
}
else
{
uint8_t x_19; 
lean_inc_ref(x_11);
lean_inc(x_10);
lean_inc_ref(x_9);
x_19 = !lean_is_exclusive(x_2);
if (x_19 == 0)
{
lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; 
x_20 = lean_ctor_get(x_2, 2);
lean_dec(x_20);
x_21 = lean_ctor_get(x_2, 1);
lean_dec(x_21);
x_22 = lean_ctor_get(x_2, 0);
lean_dec(x_22);
x_23 = lean_nat_sub(x_10, x_14);
lean_dec(x_14);
lean_dec(x_10);
lean_ctor_set(x_2, 1, x_23);
x_24 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec___closed__0));
x_25 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_25, 0, x_24);
lean_ctor_set(x_25, 1, x_2);
x_26 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_26, 0, x_25);
return x_26;
}
else
{
lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; 
lean_dec(x_2);
x_27 = lean_nat_sub(x_10, x_14);
lean_dec(x_14);
lean_dec(x_10);
x_28 = lean_alloc_ctor(0, 3, 1);
lean_ctor_set(x_28, 0, x_9);
lean_ctor_set(x_28, 1, x_27);
lean_ctor_set(x_28, 2, x_11);
lean_ctor_set_uint8(x_28, sizeof(void*)*3, x_8);
x_29 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec___closed__0));
x_30 = lean_alloc_ctor(0, 2, 0);
lean_ctor_set(x_30, 0, x_29);
lean_ctor_set(x_30, 1, x_28);
x_31 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_31, 0, x_30);
return x_31;
}
}
}
else
{
goto block_7;
}
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
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
lean_object* x_4; 
x_4 = lp_mvk__specs_MVK_Phase2_PageAlloc_alloc__pages__spec(x_1, x_2);
lean_dec(x_1);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_free__pages__spec(lean_object* x_1, lean_object* x_2, lean_object* x_3) {
_start:
{
uint8_t x_5; lean_object* x_24; uint8_t x_25; 
x_24 = lean_box(0);
x_25 = lp_mvk__specs_MVK_Phase2_Common_instDecidableEqPointer_decEq___redArg(x_1, x_24);
if (x_25 == 0)
{
uint8_t x_26; 
x_26 = lean_ctor_get_uint8(x_3, sizeof(void*)*3);
if (x_26 == 0)
{
lean_object* x_27; 
x_27 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_27, 0, x_3);
return x_27;
}
else
{
x_5 = x_25;
goto block_23;
}
}
else
{
x_5 = x_25;
goto block_23;
}
block_23:
{
if (x_5 == 0)
{
lean_object* x_6; uint8_t x_7; 
x_6 = lean_unsigned_to_nat(11u);
x_7 = lean_nat_dec_le(x_6, x_2);
if (x_7 == 0)
{
uint8_t x_8; 
x_8 = !lean_is_exclusive(x_3);
if (x_8 == 0)
{
lean_object* x_9; lean_object* x_10; lean_object* x_11; lean_object* x_12; 
x_9 = lean_ctor_get(x_3, 1);
x_10 = lp_mvk__specs_MVK_Phase2_Common_pages__for__order(x_2);
x_11 = lean_nat_add(x_9, x_10);
lean_dec(x_10);
lean_dec(x_9);
lean_ctor_set(x_3, 1, x_11);
x_12 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_12, 0, x_3);
return x_12;
}
else
{
uint8_t x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; 
x_13 = lean_ctor_get_uint8(x_3, sizeof(void*)*3);
x_14 = lean_ctor_get(x_3, 0);
x_15 = lean_ctor_get(x_3, 1);
x_16 = lean_ctor_get(x_3, 2);
lean_inc(x_16);
lean_inc(x_15);
lean_inc(x_14);
lean_dec(x_3);
x_17 = lp_mvk__specs_MVK_Phase2_Common_pages__for__order(x_2);
x_18 = lean_nat_add(x_15, x_17);
lean_dec(x_17);
lean_dec(x_15);
x_19 = lean_alloc_ctor(0, 3, 1);
lean_ctor_set(x_19, 0, x_14);
lean_ctor_set(x_19, 1, x_18);
lean_ctor_set(x_19, 2, x_16);
lean_ctor_set_uint8(x_19, sizeof(void*)*3, x_13);
x_20 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_20, 0, x_19);
return x_20;
}
}
else
{
lean_object* x_21; 
x_21 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_21, 0, x_3);
return x_21;
}
}
else
{
lean_object* x_22; 
x_22 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_22, 0, x_3);
return x_22;
}
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_free__pages__spec___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_object* x_5; 
x_5 = lp_mvk__specs_MVK_Phase2_PageAlloc_free__pages__spec(x_1, x_2, x_3);
lean_dec(x_2);
lean_dec(x_1);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_nr__free__pages__spec(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lean_ctor_get(x_1, 1);
lean_inc(x_2);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_nr__free__pages__spec___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_PageAlloc_nr__free__pages__spec(x_1);
lean_dec_ref(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase2_PageAlloc_page__size__spec(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_unsigned_to_nat(4096u);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__exit__spec(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = ((lean_object*)(lp_mvk__specs_MVK_Phase2_PageAlloc_initial__state___closed__1));
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__exit__spec___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase2_PageAlloc_page__alloc__exit__spec(x_1);
lean_dec_ref(x_1);
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_mvk__specs_MVK_Phase2_Common(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_mvk__specs_MVK_Phase2_PageAlloc(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mvk__specs_MVK_Phase2_Common(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_mvk__specs_MVK_Phase2_PageAlloc_page__size__spec = _init_lp_mvk__specs_MVK_Phase2_PageAlloc_page__size__spec();
lean_mark_persistent(lp_mvk__specs_MVK_Phase2_PageAlloc_page__size__spec);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
