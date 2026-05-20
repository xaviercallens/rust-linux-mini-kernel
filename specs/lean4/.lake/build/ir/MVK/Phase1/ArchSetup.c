// Lean compiler output
// Module: MVK.Phase1.ArchSetup
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
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS___closed__0_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS___closed__0;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorIdx(uint8_t);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorIdx___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_toCtorIdx(uint8_t);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_toCtorIdx___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim(lean_object*, lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim___boxed(lean_object*, lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim(lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim(lean_object*, uint8_t, lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim___boxed(lean_object*, lean_object*, lean_object*, lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 44, .m_capacity = 44, .m_length = 43, .m_data = "MVK.Phase1.ArchSetup.InterruptState.Enabled"};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__1_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 45, .m_capacity = 45, .m_length = 44, .m_data = "MVK.Phase1.ArchSetup.InterruptState.Disabled"};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__2_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__3_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5;
lean_object* l_Repr_addAppParen(lean_object*, lean_object*);
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr(uint8_t, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState___closed__0_value;
uint8_t lean_nat_dec_le(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ofNat(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ofNat___boxed(lean_object*);
uint8_t lean_nat_dec_eq(lean_object*, lean_object*);
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_ArchSetup_instDecidableEqInterruptState(uint8_t, uint8_t);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instDecidableEqInterruptState___boxed(lean_object*, lean_object*);
static const lean_string_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = "{ "};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__0_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__1_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 12, .m_capacity = 12, .m_length = 11, .m_data = "initialized"};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__1 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__1_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__2_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__1_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__2 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__2_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__3_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)(((size_t)(0) << 1) | 1)),((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__2_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__3 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__3_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__4_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 5, .m_capacity = 5, .m_length = 4, .m_data = " := "};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__4 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__4_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__5_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__4_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__5 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__5_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__6_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*2 + 0, .m_other = 2, .m_tag = 5}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__3_value),((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__5_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__6 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__6_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__7_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__7;
static const lean_string_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__8_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 2, .m_capacity = 2, .m_length = 1, .m_data = ","};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__8 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__8_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__9_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__8_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__9 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__9_value;
static const lean_string_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__10_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 11, .m_capacity = 11, .m_length = 10, .m_data = "interrupts"};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__10 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__10_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__11_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__10_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__11 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__11_value;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__12_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__12;
static const lean_string_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__13_value = {.m_header = {.m_rc = 0, .m_cs_sz = 0, .m_other = 0, .m_tag = 249}, .m_size = 3, .m_capacity = 3, .m_length = 2, .m_data = " }"};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__13 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__13_value;
lean_object* lean_string_length(lean_object*);
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__14_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__14;
static lean_once_cell_t lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__15_once = LEAN_ONCE_CELL_INITIALIZER;
static lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__15;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__16_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__0_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__16 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__16_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__17_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*1 + 0, .m_other = 1, .m_tag = 3}, .m_objs = {((lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__13_value)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__17 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__17_value;
lean_object* l_Bool_repr___redArg(uint8_t);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___boxed(lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr(lean_object*, lean_object*);
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___boxed(lean_object*, lean_object*);
static const lean_closure_object lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_closure_object) + sizeof(void*)*0, .m_other = 0, .m_tag = 245}, .m_fun = (void*)lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___boxed, .m_arity = 2, .m_num_fixed = 0, .m_objs = {} };
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_initial__arch__state___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*0 + 8, .m_other = 0, .m_tag = 0}, .m_objs = {LEAN_SCALAR_PTR_LITERAL(0, 0, 0, 0, 0, 0, 0, 0)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_initial__arch__state___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_initial__arch__state___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_initial__arch__state = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_initial__arch__state___closed__0_value;
static const lean_ctor_object lp_mvk__specs_MVK_Phase1_ArchSetup_safe__arch__state___closed__0_value = {.m_header = {.m_rc = 0, .m_cs_sz = sizeof(lean_ctor_object) + sizeof(void*)*0 + 8, .m_other = 0, .m_tag = 0}, .m_objs = {LEAN_SCALAR_PTR_LITERAL(1, 1, 0, 0, 0, 0, 0, 0)}};
static const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_safe__arch__state___closed__0 = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_safe__arch__state___closed__0_value;
LEAN_EXPORT const lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_safe__arch__state = (const lean_object*)&lp_mvk__specs_MVK_Phase1_ArchSetup_safe__arch__state___closed__0_value;
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_arch__setup__exit();
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_arch__setup__exit___boxed(lean_object*);
static lean_object* _init_lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS___closed__0(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(0u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS(void) {
_start:
{
lean_object* x_1; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS___closed__0, &lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS___closed__0_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS___closed__0);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorIdx(uint8_t x_1) {
_start:
{
if (x_1 == 0)
{
lean_object* x_2; 
x_2 = lean_unsigned_to_nat(0u);
return x_2;
}
else
{
lean_object* x_3; 
x_3 = lean_unsigned_to_nat(1u);
return x_3;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorIdx___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lean_unbox(x_1);
x_3 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorIdx(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_toCtorIdx(uint8_t x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorIdx(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_toCtorIdx___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lean_unbox(x_1);
x_3 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_toCtorIdx(x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim(lean_object* x_1, lean_object* x_2, uint8_t x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
lean_inc(x_5);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4, lean_object* x_5) {
_start:
{
uint8_t x_6; lean_object* x_7; 
x_6 = lean_unbox(x_3);
x_7 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorElim(x_1, x_2, x_6, x_4, x_5);
lean_dec(x_5);
lean_dec(x_2);
return x_7;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim(lean_object* x_1, uint8_t x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_inc(x_4);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; lean_object* x_6; 
x_5 = lean_unbox(x_2);
x_6 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Enabled_elim(x_1, x_5, x_3, x_4);
lean_dec(x_4);
return x_6;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim___redArg(lean_object* x_1) {
_start:
{
lean_inc(x_1);
return x_1;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim___redArg(x_1);
lean_dec(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim(lean_object* x_1, uint8_t x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
lean_inc(x_4);
return x_4;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim___boxed(lean_object* x_1, lean_object* x_2, lean_object* x_3, lean_object* x_4) {
_start:
{
uint8_t x_5; lean_object* x_6; 
x_5 = lean_unbox(x_2);
x_6 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_Disabled_elim(x_1, x_5, x_3, x_4);
lean_dec(x_4);
return x_6;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(2u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(1u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr(uint8_t x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; lean_object* x_10; 
if (x_1 == 0)
{
lean_object* x_17; uint8_t x_18; 
x_17 = lean_unsigned_to_nat(1024u);
x_18 = lean_nat_dec_le(x_17, x_2);
if (x_18 == 0)
{
lean_object* x_19; 
x_19 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4, &lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4);
x_3 = x_19;
goto block_9;
}
else
{
lean_object* x_20; 
x_20 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5, &lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5);
x_3 = x_20;
goto block_9;
}
}
else
{
lean_object* x_21; uint8_t x_22; 
x_21 = lean_unsigned_to_nat(1024u);
x_22 = lean_nat_dec_le(x_21, x_2);
if (x_22 == 0)
{
lean_object* x_23; 
x_23 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4, &lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__4);
x_10 = x_23;
goto block_16;
}
else
{
lean_object* x_24; 
x_24 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5, &lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__5);
x_10 = x_24;
goto block_16;
}
}
block_9:
{
lean_object* x_4; lean_object* x_5; uint8_t x_6; lean_object* x_7; lean_object* x_8; 
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__1));
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
x_11 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___closed__3));
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
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; lean_object* x_4; 
x_3 = lean_unbox(x_1);
x_4 = lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr(x_3, x_2);
lean_dec(x_2);
return x_4;
}
}
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ofNat(lean_object* x_1) {
_start:
{
lean_object* x_2; uint8_t x_3; 
x_2 = lean_unsigned_to_nat(0u);
x_3 = lean_nat_dec_le(x_1, x_2);
if (x_3 == 0)
{
uint8_t x_4; 
x_4 = 1;
return x_4;
}
else
{
uint8_t x_5; 
x_5 = 0;
return x_5;
}
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ofNat___boxed(lean_object* x_1) {
_start:
{
uint8_t x_2; lean_object* x_3; 
x_2 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ofNat(x_1);
lean_dec(x_1);
x_3 = lean_box(x_2);
return x_3;
}
}
LEAN_EXPORT uint8_t lp_mvk__specs_MVK_Phase1_ArchSetup_instDecidableEqInterruptState(uint8_t x_1, uint8_t x_2) {
_start:
{
lean_object* x_3; lean_object* x_4; uint8_t x_5; 
x_3 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorIdx(x_1);
x_4 = lp_mvk__specs_MVK_Phase1_ArchSetup_InterruptState_ctorIdx(x_2);
x_5 = lean_nat_dec_eq(x_3, x_4);
lean_dec(x_4);
lean_dec(x_3);
return x_5;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instDecidableEqInterruptState___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
uint8_t x_3; uint8_t x_4; uint8_t x_5; lean_object* x_6; 
x_3 = lean_unbox(x_1);
x_4 = lean_unbox(x_2);
x_5 = lp_mvk__specs_MVK_Phase1_ArchSetup_instDecidableEqInterruptState(x_3, x_4);
x_6 = lean_box(x_5);
return x_6;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__7(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(15u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__12(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_unsigned_to_nat(14u);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__14(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__0));
x_2 = lean_string_length(x_1);
return x_2;
}
}
static lean_object* _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__15(void) {
_start:
{
lean_object* x_1; lean_object* x_2; 
x_1 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__14, &lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__14_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__14);
x_2 = lean_nat_to_int(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg(lean_object* x_1) {
_start:
{
uint8_t x_2; uint8_t x_3; lean_object* x_4; lean_object* x_5; lean_object* x_6; lean_object* x_7; lean_object* x_8; lean_object* x_9; uint8_t x_10; lean_object* x_11; lean_object* x_12; lean_object* x_13; lean_object* x_14; lean_object* x_15; lean_object* x_16; lean_object* x_17; lean_object* x_18; lean_object* x_19; lean_object* x_20; lean_object* x_21; lean_object* x_22; lean_object* x_23; lean_object* x_24; lean_object* x_25; lean_object* x_26; lean_object* x_27; lean_object* x_28; lean_object* x_29; lean_object* x_30; lean_object* x_31; 
x_2 = lean_ctor_get_uint8(x_1, 0);
x_3 = lean_ctor_get_uint8(x_1, 1);
x_4 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__5));
x_5 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__6));
x_6 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__7, &lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__7_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__7);
x_7 = lean_unsigned_to_nat(0u);
x_8 = l_Bool_repr___redArg(x_2);
x_9 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_9, 0, x_6);
lean_ctor_set(x_9, 1, x_8);
x_10 = 0;
x_11 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_11, 0, x_9);
lean_ctor_set_uint8(x_11, sizeof(void*)*1, x_10);
x_12 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_12, 0, x_5);
lean_ctor_set(x_12, 1, x_11);
x_13 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__9));
x_14 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_14, 0, x_12);
lean_ctor_set(x_14, 1, x_13);
x_15 = lean_box(1);
x_16 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_16, 0, x_14);
lean_ctor_set(x_16, 1, x_15);
x_17 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__11));
x_18 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_18, 0, x_16);
lean_ctor_set(x_18, 1, x_17);
x_19 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_19, 0, x_18);
lean_ctor_set(x_19, 1, x_4);
x_20 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__12, &lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__12_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__12);
x_21 = lp_mvk__specs_MVK_Phase1_ArchSetup_instReprInterruptState_repr(x_3, x_7);
x_22 = lean_alloc_ctor(4, 2, 0);
lean_ctor_set(x_22, 0, x_20);
lean_ctor_set(x_22, 1, x_21);
x_23 = lean_alloc_ctor(6, 1, 1);
lean_ctor_set(x_23, 0, x_22);
lean_ctor_set_uint8(x_23, sizeof(void*)*1, x_10);
x_24 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_24, 0, x_19);
lean_ctor_set(x_24, 1, x_23);
x_25 = lean_obj_once(&lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__15, &lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__15_once, _init_lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__15);
x_26 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__16));
x_27 = lean_alloc_ctor(5, 2, 0);
lean_ctor_set(x_27, 0, x_26);
lean_ctor_set(x_27, 1, x_24);
x_28 = ((lean_object*)(lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___closed__17));
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
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg(x_1);
lean_dec_ref(x_1);
return x_2;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___redArg(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr___boxed(lean_object* x_1, lean_object* x_2) {
_start:
{
lean_object* x_3; 
x_3 = lp_mvk__specs_MVK_Phase1_ArchSetup_instReprArchState_repr(x_1, x_2);
lean_dec(x_2);
lean_dec_ref(x_1);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_arch__setup__exit() {
_start:
{
lean_object* x_2; lean_object* x_3; 
x_2 = lean_box(0);
x_3 = lean_alloc_ctor(0, 1, 0);
lean_ctor_set(x_3, 0, x_2);
return x_3;
}
}
LEAN_EXPORT lean_object* lp_mvk__specs_MVK_Phase1_ArchSetup_arch__setup__exit___boxed(lean_object* x_1) {
_start:
{
lean_object* x_2; 
x_2 = lp_mvk__specs_MVK_Phase1_ArchSetup_arch__setup__exit();
return x_2;
}
}
lean_object* initialize_Init(uint8_t builtin);
lean_object* initialize_mvk__specs_MVK_Phase2_Common(uint8_t builtin);
static bool _G_initialized = false;
LEAN_EXPORT lean_object* initialize_mvk__specs_MVK_Phase1_ArchSetup(uint8_t builtin) {
lean_object * res;
if (_G_initialized) return lean_io_result_mk_ok(lean_box(0));
_G_initialized = true;
res = initialize_Init(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
res = initialize_mvk__specs_MVK_Phase2_Common(builtin);
if (lean_io_result_is_error(res)) return res;
lean_dec_ref(res);
lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS = _init_lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS();
lean_mark_persistent(lp_mvk__specs_MVK_Phase1_ArchSetup_SUCCESS);
return lean_io_result_mk_ok(lean_box(0));
}
#ifdef __cplusplus
}
#endif
