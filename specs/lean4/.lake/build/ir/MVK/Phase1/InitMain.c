// Lean compiler output
// Module: MVK.Phase1.InitMain
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
lean_object* lean_nat_to_int(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS___closed__0;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0;
lean_object* lean_int_neg(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__1;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_FAILURE;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__0_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 12, .m_capacity = 12, .m_length = 11, .m_data = "initialized"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__2_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__3_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__3_value),((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__6_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__7;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = ","};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__8 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__8_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__8_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__9 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__9_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 13, .m_capacity = 13, .m_length = 12, .m_data = "printk_ready"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__10 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__10_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__10_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__11 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__11_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__12_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__12;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__13_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 11, .m_capacity = 11, .m_length = 10, .m_data = "arch_ready"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__13 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__13_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__14_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__13_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__14 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__14_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__15_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__15;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__16_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 17, .m_capacity = 17, .m_length = 16, .m_data = "page_alloc_ready"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__16 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__16_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__17_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__16_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__17 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__17_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__18_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__18;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__19_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 11, .m_capacity = 11, .m_length = 10, .m_data = "slab_ready"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__19 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__19_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__20_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__19_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__20 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__20_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__21_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__21 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__21_value;
lean_object* lean_string_length(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__22_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__22;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__24_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__24 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__24_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__25_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__21_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__25 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__25_value;
lean_object* l_Bool_repr___redArg(uint8_t);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_initial__state___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*0 + 8, .m_other = 0, .m_tag = 0}, .m_objs = {LEAN_SCALAR_PTR_LITERAL(0, 0, 0, 0, 0, 0, 0, 0)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_initial__state___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_initial__state___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_initial__state = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_initial__state___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_init__main__init();
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_init__main__init___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_init__main__exit();
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_init__main__exit___boxed(lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = "step"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__2_value),((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__3_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 6, .m_capacity = 6, .m_length = 5, .m_data = "state"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__6_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7;
lean_object* l_Nat_reprFast(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_boot__sequence__init___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 0}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_initial__state___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_boot__sequence__init___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_boot__sequence__init___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_boot__sequence__init = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_boot__sequence__init___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorIdx(uint8_t);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorIdx___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_toCtorIdx(uint8_t);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_toCtorIdx___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim(lean_object*, lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim(lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim(lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim(lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim(lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim(lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 40, .m_capacity = 40, .m_length = 39, .m_data = "MVK.Phase1.InitMain.BootStep.PrintkInit"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__1_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 39, .m_capacity = 39, .m_length = 38, .m_data = "MVK.Phase1.InitMain.BootStep.ArchSetup"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__2_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__3_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 43, .m_capacity = 43, .m_length = 42, .m_data = "MVK.Phase1.InitMain.BootStep.PageAllocInit"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__5_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 38, .m_capacity = 38, .m_length = 37, .m_data = "MVK.Phase1.InitMain.BootStep.SlabInit"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__6_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__7_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__6_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__7 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__7_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 38, .m_capacity = 38, .m_length = 37, .m_data = "MVK.Phase1.InitMain.BootStep.Complete"};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__8 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__8_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__8_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__9 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__9_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10;
lean_object* l_Repr_addAppParen(lean_object*, lean_object*);
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr(uint8_t, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep = (const lean_object*)&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep___closed__0_value;
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ofNat(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ofNat___boxed(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_InitMain_instDecidableEqBootStep(uint8_t, uint8_t);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instDecidableEqBootStep___boxed(lean_object*, lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__0;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__1_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__1;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__2_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__2;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__3_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__3;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__4_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__4;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___boxed(lean_object*);
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS___closed__0(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS___closed__0, &lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS___closed__0);
return x_1;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__1(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0, &lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0);
x_2 = lean_int_neg(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__1, &lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__1_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__1);
return x_1;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__7(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(15u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__12(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(16u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__15(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(14u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__18(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(20u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__22(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__0));
x_2 = lean_string_length(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__22, &lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__22_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__22);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg(lean_object* x_1) {
_start:
{
uint8_t x_2; uint8_t x_3; uint8_t x_4; uint8_t x_5; uint8_t x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; lean_object* x_10; lean_object* x_11; uint8_t x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; lean_object* x_32; lean_object* x_33; lean_object* x_34; lean_object* x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; lean_object* x_39; lean_object* x_40; lean_object* x_41; lean_object* x_42; lean_object* x_43; lean_object* x_44; lean_object* x_45; lean_object* x_46; lean_object* x_47; lean_object* x_48; lean_object* x_49; lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; lean_object* x_55; lean_object* x_56; lean_object* x_57; lean_object* x_58; lean_object* x_59; lean_object* x_60; lean_object* x_61; lean_object* x_62; 
x_2 = lean_ctor_get_uint8(x_1, 0);
x_3 = lean_ctor_get_uint8(x_1, 1);
x_4 = lean_ctor_get_uint8(x_1, 2);
x_5 = lean_ctor_get_uint8(x_1, 3);
x_6 = lean_ctor_get_uint8(x_1, 4);
x_7 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__5));
x_8 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__6));
x_9 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__7);
x_10 = l_Bool_repr___redArg(x_2);
x_11 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_11, 0, x_9);
lean_ctor_set(x_11, 1, x_10);
x_12 = 0;
x_13 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_13, 0, x_11);
lean_ctor_set_uint8(x_13, sizeof(void*)*1, x_12);
x_14 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_14, 0, x_8);
lean_ctor_set(x_14, 1, x_13);
x_15 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__9));
x_16 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_16, 0, x_14);
lean_ctor_set(x_16, 1, x_15);
x_17 = lean_box(1);
x_18 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_18, 0, x_16);
lean_ctor_set(x_18, 1, x_17);
x_19 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__11));
x_20 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_20, 0, x_18);
lean_ctor_set(x_20, 1, x_19);
x_21 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_21, 0, x_20);
lean_ctor_set(x_21, 1, x_7);
x_22 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__12, &lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__12_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__12);
x_23 = l_Bool_repr___redArg(x_3);
x_24 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_24, 0, x_22);
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
x_29 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__14));
x_30 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_30, 0, x_28);
lean_ctor_set(x_30, 1, x_29);
x_31 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_31, 0, x_30);
lean_ctor_set(x_31, 1, x_7);
x_32 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__15, &lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__15_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__15);
x_33 = l_Bool_repr___redArg(x_4);
x_34 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_34, 0, x_32);
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
x_39 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__17));
x_40 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_40, 0, x_38);
lean_ctor_set(x_40, 1, x_39);
x_41 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_41, 0, x_40);
lean_ctor_set(x_41, 1, x_7);
x_42 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__18, &lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__18_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__18);
x_43 = l_Bool_repr___redArg(x_5);
x_44 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_44, 0, x_42);
lean_ctor_set(x_44, 1, x_43);
x_45 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_45, 0, x_44);
lean_ctor_set_uint8(x_45, sizeof(void*)*1, x_12);
x_46 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_46, 0, x_41);
lean_ctor_set(x_46, 1, x_45);
x_47 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_47, 0, x_46);
lean_ctor_set(x_47, 1, x_15);
x_48 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_48, 0, x_47);
lean_ctor_set(x_48, 1, x_17);
x_49 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__20));
x_50 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_50, 0, x_48);
lean_ctor_set(x_50, 1, x_49);
x_51 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_51, 0, x_50);
lean_ctor_set(x_51, 1, x_7);
x_52 = l_Bool_repr___redArg(x_6);
x_53 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_53, 0, x_32);
lean_ctor_set(x_53, 1, x_52);
x_54 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_54, 0, x_53);
lean_ctor_set_uint8(x_54, sizeof(void*)*1, x_12);
x_55 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_55, 0, x_51);
lean_ctor_set(x_55, 1, x_54);
x_56 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23, &lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23);
x_57 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__24));
x_58 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_58, 0, x_57);
lean_ctor_set(x_58, 1, x_55);
x_59 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__25));
x_60 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_60, 0, x_58);
lean_ctor_set(x_60, 1, x_59);
x_61 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_61, 0, x_56);
lean_ctor_set(x_61, 1, x_60);
x_62 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_62, 0, x_61);
lean_ctor_set_uint8(x_62, sizeof(void*)*1, x_12);
return x_62;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg(x_1);
lean_dec_ref(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr(x_1, x_2);
lean_dec(x_2);
lean_dec_ref(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_init__main__init() {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS;
x_3 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_init__main__init___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_init__main__init();
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_init__main__exit() {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = lean_box(0);
x_3 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_init__main__exit___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_init__main__exit();
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(8u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(9u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg(lean_object* x_1) {
_start:
{
uint8_t x_2; 
x_2 = !lean_is_exclusive(x_1);
if (x_2 == 0)
{
lean_object* x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; uint8_t x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; 
x_3 = lean_ctor_get(x_1, 0);
x_4 = lean_ctor_get(x_1, 1);
x_5 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__5));
x_6 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__3));
x_7 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4);
x_8 = l_Nat_reprFast(x_3);
x_9 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_9, 0, x_8);
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
x_13 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__9));
x_14 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_14, 0, x_12);
lean_ctor_set(x_14, 1, x_13);
x_15 = lean_box(1);
x_16 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_16, 0, x_14);
lean_ctor_set(x_16, 1, x_15);
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__6));
x_18 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_18, 0, x_16);
lean_ctor_set(x_18, 1, x_17);
x_19 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_19, 0, x_18);
lean_ctor_set(x_19, 1, x_5);
x_20 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7);
x_21 = lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg(x_4);
lean_dec_ref(x_4);
x_22 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_22, 0, x_20);
lean_ctor_set(x_22, 1, x_21);
x_23 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_23, 0, x_22);
lean_ctor_set_uint8(x_23, sizeof(void*)*1, x_10);
x_24 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_24, 0, x_19);
lean_ctor_set(x_24, 1, x_23);
x_25 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23, &lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23);
x_26 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__24));
x_27 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_27, 0, x_26);
lean_ctor_set(x_27, 1, x_24);
x_28 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__25));
x_29 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_29, 0, x_27);
lean_ctor_set(x_29, 1, x_28);
x_30 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_30, 0, x_25);
lean_ctor_set(x_30, 1, x_29);
x_31 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_31, 0, x_30);
lean_ctor_set_uint8(x_31, sizeof(void*)*1, x_10);
return x_31;
}
else
{
lean_object* x_32; lean_object* x_33; lean_object* x_34; lean_object* x_35; lean_object* x_36; lean_object* x_37; lean_object* x_38; lean_object* x_39; uint8_t x_40; lean_object* x_41; lean_object* x_42; lean_object* x_43; lean_object* x_44; lean_object* x_45; lean_object* x_46; lean_object* x_47; lean_object* x_48; lean_object* x_49; lean_object* x_50; lean_object* x_51; lean_object* x_52; lean_object* x_53; lean_object* x_54; lean_object* x_55; lean_object* x_56; lean_object* x_57; lean_object* x_58; lean_object* x_59; lean_object* x_60; lean_object* x_61; 
x_32 = lean_ctor_get(x_1, 0);
x_33 = lean_ctor_get(x_1, 1);
lean_inc(x_33);
lean_inc(x_32);
lean_dec(x_1);
x_34 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__5));
x_35 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__3));
x_36 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__4);
x_37 = l_Nat_reprFast(x_32);
x_38 = lean_alloc_ctor(3, 1, 0);
lean_ctor_set(x_38, 0, x_37);
x_39 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_39, 0, x_36);
lean_ctor_set(x_39, 1, x_38);
x_40 = 0;
x_41 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_41, 0, x_39);
lean_ctor_set_uint8(x_41, sizeof(void*)*1, x_40);
x_42 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_42, 0, x_35);
lean_ctor_set(x_42, 1, x_41);
x_43 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__9));
x_44 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_44, 0, x_42);
lean_ctor_set(x_44, 1, x_43);
x_45 = lean_box(1);
x_46 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_46, 0, x_44);
lean_ctor_set(x_46, 1, x_45);
x_47 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__6));
x_48 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_48, 0, x_46);
lean_ctor_set(x_48, 1, x_47);
x_49 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_49, 0, x_48);
lean_ctor_set(x_49, 1, x_34);
x_50 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg___closed__7);
x_51 = lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg(x_33);
lean_dec_ref(x_33);
x_52 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_52, 0, x_50);
lean_ctor_set(x_52, 1, x_51);
x_53 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_53, 0, x_52);
lean_ctor_set_uint8(x_53, sizeof(void*)*1, x_40);
x_54 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_54, 0, x_49);
lean_ctor_set(x_54, 1, x_53);
x_55 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23, &lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__23);
x_56 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__24));
x_57 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_57, 0, x_56);
lean_ctor_set(x_57, 1, x_54);
x_58 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprInitMainState_repr___redArg___closed__25));
x_59 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_59, 0, x_57);
lean_ctor_set(x_59, 1, x_58);
x_60 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_60, 0, x_55);
lean_ctor_set(x_60, 1, x_59);
x_61 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_61, 0, x_60);
lean_ctor_set_uint8(x_61, sizeof(void*)*1, x_40);
return x_61;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase1_InitMain_instReprBootSequence_repr(x_1, x_2);
lean_dec(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorIdx(uint8_t x_1) {
_start:
{
switch (x_1) {
case 0:
{
lean_object* x_2; 
x_2 = lean_unsigned_to_nat(0u);
return x_2;
}
case 1:
{
lean_object* x_3; 
x_3 = lean_unsigned_to_nat(1u);
return x_3;
}
case 2:
{
lean_object* x_4; 
x_4 = lean_unsigned_to_nat(2u);
return x_4;
}
case 3:
{
lean_object* x_5; 
x_5 = lean_unsigned_to_nat(3u);
return x_5;
}
default: 
{
lean_object* x_6; 
x_6 = lean_unsigned_to_nat(4u);
return x_6;
}
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorIdx___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lean_unbox(x_1);
x_3 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorIdx(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_toCtorIdx(uint8_t x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorIdx(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_toCtorIdx___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lean_unbox(x_1);
x_3 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_toCtorIdx(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim(lean_object* x_1, lean_object* x_2, uint8_t x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_inc(x_5);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
uint8_t x_6; lean_object* x_7; 
x_6 = lean_unbox(x_3);
x_7 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorElim(x_1, x_2, x_6, x_4, x_5);
lean_dec(x_5);
lean_dec(x_2);
return x_7;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim(lean_object* x_1, uint8_t x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_inc(x_4);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; lean_object* x_6; 
x_5 = lean_unbox(x_2);
x_6 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PrintkInit_elim(x_1, x_5, x_3, x_4);
lean_dec(x_4);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim(lean_object* x_1, uint8_t x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_inc(x_4);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; lean_object* x_6; 
x_5 = lean_unbox(x_2);
x_6 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ArchSetup_elim(x_1, x_5, x_3, x_4);
lean_dec(x_4);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim(lean_object* x_1, uint8_t x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_inc(x_4);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; lean_object* x_6; 
x_5 = lean_unbox(x_2);
x_6 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_PageAllocInit_elim(x_1, x_5, x_3, x_4);
lean_dec(x_4);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim(lean_object* x_1, uint8_t x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_inc(x_4);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; lean_object* x_6; 
x_5 = lean_unbox(x_2);
x_6 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_SlabInit_elim(x_1, x_5, x_3, x_4);
lean_dec(x_4);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim(lean_object* x_1, uint8_t x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_inc(x_4);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; lean_object* x_6; 
x_5 = lean_unbox(x_2);
x_6 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_Complete_elim(x_1, x_5, x_3, x_4);
lean_dec(x_4);
return x_6;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(2u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr(uint8_t x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_10; lean_object* x_17; lean_object* x_24; lean_object* x_31; 
switch (x_1) {
case 0:
{
lean_object* x_38; uint8_t x_39; 
x_38 = lean_unsigned_to_nat(1024u);
x_39 = lean_nat_dec_le(x_38, x_2);
if (x_39 == 0)
{
lean_object* x_40; 
x_40 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10);
x_3 = x_40;
goto block_9;
}
else
{
lean_object* x_41; 
x_41 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0, &lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0);
x_3 = x_41;
goto block_9;
}
}
case 1:
{
lean_object* x_42; uint8_t x_43; 
x_42 = lean_unsigned_to_nat(1024u);
x_43 = lean_nat_dec_le(x_42, x_2);
if (x_43 == 0)
{
lean_object* x_44; 
x_44 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10);
x_10 = x_44;
goto block_16;
}
else
{
lean_object* x_45; 
x_45 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0, &lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0);
x_10 = x_45;
goto block_16;
}
}
case 2:
{
lean_object* x_46; uint8_t x_47; 
x_46 = lean_unsigned_to_nat(1024u);
x_47 = lean_nat_dec_le(x_46, x_2);
if (x_47 == 0)
{
lean_object* x_48; 
x_48 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10);
x_17 = x_48;
goto block_23;
}
else
{
lean_object* x_49; 
x_49 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0, &lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0);
x_17 = x_49;
goto block_23;
}
}
case 3:
{
lean_object* x_50; uint8_t x_51; 
x_50 = lean_unsigned_to_nat(1024u);
x_51 = lean_nat_dec_le(x_50, x_2);
if (x_51 == 0)
{
lean_object* x_52; 
x_52 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10);
x_24 = x_52;
goto block_30;
}
else
{
lean_object* x_53; 
x_53 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0, &lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0);
x_24 = x_53;
goto block_30;
}
}
default: 
{
lean_object* x_54; uint8_t x_55; 
x_54 = lean_unsigned_to_nat(1024u);
x_55 = lean_nat_dec_le(x_54, x_2);
if (x_55 == 0)
{
lean_object* x_56; 
x_56 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10, &lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__10);
x_31 = x_56;
goto block_37;
}
else
{
lean_object* x_57; 
x_57 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0, &lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE___closed__0);
x_31 = x_57;
goto block_37;
}
}
}
block_9:
{
lean_object* x_4; lean_object* x_5; uint8_t x_6; lean_object* x_7; lean_object* x_8; 
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__1));
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
block_16:
{
lean_object* x_11; lean_object* x_12; uint8_t x_13; lean_object* x_14; lean_object* x_15; 
x_11 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__3));
x_12 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_12, 0, x_10);
lean_ctor_set(x_12, 1, x_11);
x_13 = 0;
x_14 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_14, 0, x_12);
lean_ctor_set_uint8(x_14, sizeof(void*)*1, x_13);
x_15 = l_Repr_addAppParen(x_14, x_2);
return x_15;
}
block_23:
{
lean_object* x_18; lean_object* x_19; uint8_t x_20; lean_object* x_21; lean_object* x_22; 
x_18 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__5));
x_19 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_19, 0, x_17);
lean_ctor_set(x_19, 1, x_18);
x_20 = 0;
x_21 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_21, 0, x_19);
lean_ctor_set_uint8(x_21, sizeof(void*)*1, x_20);
x_22 = l_Repr_addAppParen(x_21, x_2);
return x_22;
}
block_30:
{
lean_object* x_25; lean_object* x_26; uint8_t x_27; lean_object* x_28; lean_object* x_29; 
x_25 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__7));
x_26 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_26, 0, x_24);
lean_ctor_set(x_26, 1, x_25);
x_27 = 0;
x_28 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_28, 0, x_26);
lean_ctor_set_uint8(x_28, sizeof(void*)*1, x_27);
x_29 = l_Repr_addAppParen(x_28, x_2);
return x_29;
}
block_37:
{
lean_object* x_32; lean_object* x_33; uint8_t x_34; lean_object* x_35; lean_object* x_36; 
x_32 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___closed__9));
x_33 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_33, 0, x_31);
lean_ctor_set(x_33, 1, x_32);
x_34 = 0;
x_35 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_35, 0, x_33);
lean_ctor_set_uint8(x_35, sizeof(void*)*1, x_34);
x_36 = l_Repr_addAppParen(x_35, x_2);
return x_36;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = lean_unbox(x_1);
x_4 = lp_mvk__specs_MVK_Phase1_InitMain_instReprBootStep_repr(x_3, x_2);
lean_dec(x_2);
return x_4;
}
}
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ofNat(lean_object* x_1) {
_start:
{
lean_object* x_2; uint8_t x_3; 
x_2 = lean_unsigned_to_nat(1u);
x_3 = lean_nat_dec_le(x_1, x_2);
if (x_3 == 0)
{
lean_object* x_4; uint8_t x_5; 
x_4 = lean_unsigned_to_nat(2u);
x_5 = lean_nat_dec_le(x_1, x_4);
if (x_5 == 0)
{
lean_object* x_6; uint8_t x_7; 
x_6 = lean_unsigned_to_nat(3u);
x_7 = lean_nat_dec_le(x_1, x_6);
if (x_7 == 0)
{
uint8_t x_8; 
x_8 = 4;
return x_8;
}
else
{
uint8_t x_9; 
x_9 = 3;
return x_9;
}
}
else
{
uint8_t x_10; 
x_10 = 2;
return x_10;
}
}
else
{
lean_object* x_11; uint8_t x_12; 
x_11 = lean_unsigned_to_nat(0u);
x_12 = lean_nat_dec_le(x_1, x_11);
if (x_12 == 0)
{
uint8_t x_13; 
x_13 = 1;
return x_13;
}
else
{
uint8_t x_14; 
x_14 = 0;
return x_14;
}
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ofNat___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ofNat(x_1);
lean_dec(x_1);
x_3 = lean_box(x_2);
return x_3;
}
}
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_InitMain_instDecidableEqBootStep(uint8_t x_1, uint8_t x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; uint8_t x_5; 
x_3 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorIdx(x_1);
x_4 = lp_mvk__specs_MVK_Phase1_InitMain_BootStep_ctorIdx(x_2);
x_5 = lean_nat_dec_eq(x_3, x_4);
lean_dec(x_4);
lean_dec(x_3);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_instDecidableEqBootStep___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; uint8_t x_4; uint8_t x_5; lean_object* x_6; 
x_3 = lean_unbox(x_1);
x_4 = lean_unbox(x_2);
x_5 = lp_mvk__specs_MVK_Phase1_InitMain_instDecidableEqBootStep(x_3, x_4);
x_6 = lean_box(x_5);
return x_6;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__0(void) {
_start:
{
uint8_t x_1; lean_object* x_2; lean_object* x_3; 
x_1 = 4;
x_2 = lean_box(x_1);
x_3 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__1(void) {
_start:
{
uint8_t x_1; lean_object* x_2; lean_object* x_3; 
x_1 = 3;
x_2 = lean_box(x_1);
x_3 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__2(void) {
_start:
{
uint8_t x_1; lean_object* x_2; lean_object* x_3; 
x_1 = 2;
x_2 = lean_box(x_1);
x_3 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__3(void) {
_start:
{
uint8_t x_1; lean_object* x_2; lean_object* x_3; 
x_1 = 1;
x_2 = lean_box(x_1);
x_3 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__4(void) {
_start:
{
uint8_t x_1; lean_object* x_2; lean_object* x_3; 
x_1 = 0;
x_2 = lean_box(x_1);
x_3 = lean_alloc_ctor(1, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step(lean_object* x_1) {
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
lean_object* x_12; 
x_12 = lean_box(0);
return x_12;
}
else
{
lean_object* x_13; 
x_13 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__0, &lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__0);
return x_13;
}
}
else
{
lean_object* x_14; 
x_14 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__1, &lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__1_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__1);
return x_14;
}
}
else
{
lean_object* x_15; 
x_15 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__2, &lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__2_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__2);
return x_15;
}
}
else
{
lean_object* x_16; 
x_16 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__3, &lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__3_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__3);
return x_16;
}
}
else
{
lean_object* x_17; 
x_17 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__4, &lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__4_once, _init_lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___closed__4);
return x_17;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_InitMain_step__to__boot__step(x_1);
lean_dec(x_1);
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_mvk__specs_MVK_Phase2_Common(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_mvk__specs_MVK_Phase1_InitMain(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mvk__specs_MVK_Phase2_Common(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS = _init_lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS();
lean_mark_persistent(lp_mvk__specs_MVK_Phase1_InitMain_SUCCESS);
lp_mvk__specs_MVK_Phase1_InitMain_FAILURE = _init_lp_mvk__specs_MVK_Phase1_InitMain_FAILURE();
lean_mark_persistent(lp_mvk__specs_MVK_Phase1_InitMain_FAILURE);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
