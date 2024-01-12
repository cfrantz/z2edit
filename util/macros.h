#ifndef PROTONES_UTIL_MACROS_H
#define PROTONES_UTIL_MACROS_H

#define PASTE_(x, y) x ## y
#define PASTE(x, y) PASTE_(x, y)
#define PASTE_LIST(x, y) PASTE(x, y),

#define STRINGIFY(x) #x

#define PP_NARG(...)    PP_NARG_(__VA_ARGS__, PP_RSEQ_N())
#define PP_NARG_(...)   PP_ARG_N(__VA_ARGS__)

#define PP_ARG_N( \
        _1, _2, _3, _4, _5, _6, _7, _8, _9,_10,  \
        _11,_12,_13,_14,_15,_16,_17,_18,_19,_20, \
        _21,_22,_23,_24,_25,_26,_27,_28,_29,_30, \
        _31,_32,_33,_34,_35,_36,_37,_38,_39,_40, \
        _41,_42,_43,_44,_45,_46,_47,_48,_49,_50, \
        _51,_52,_53,_54,_55,_56,_57,_58,_59,_60, \
        _61,_62,_63,N,...) N

#define PP_RSEQ_N() \
        63,62,61,60,                   \
        59,58,57,56,55,54,53,52,51,50, \
        49,48,47,46,45,44,43,42,41,40, \
        39,38,37,36,35,34,33,32,31,30, \
        29,28,27,26,25,24,23,22,21,20, \
        19,18,17,16,15,14,13,12,11,10, \
        9,8,7,6,5,4,3,2,1,0

/**
 * Call macro `X` on each argument in the argument list.
 */
#define APPLYX_(M, ...) M(__VA_ARGS__)
#define APPLYX(X, ...) APPLYX_(PASTE(APPLYX, PP_NARG(__VA_ARGS__)), X, __VA_ARGS__)
#define APPLYX1(X, a0) X(a0)
#define APPLYX2(X, a0, ...) X(a0) APPLYX1(X, __VA_ARGS__)
#define APPLYX3(X, a0, ...) X(a0) APPLYX2(X, __VA_ARGS__)
#define APPLYX4(X, a0, ...) X(a0) APPLYX3(X, __VA_ARGS__)
#define APPLYX5(X, a0, ...) X(a0) APPLYX4(X, __VA_ARGS__)
#define APPLYX6(X, a0, ...) X(a0) APPLYX5(X, __VA_ARGS__)
#define APPLYX7(X, a0, ...) X(a0) APPLYX6(X, __VA_ARGS__)
#define APPLYX8(X, a0, ...) X(a0) APPLYX7(X, __VA_ARGS__)
#define APPLYX9(X, a0, ...) X(a0) APPLYX8(X, __VA_ARGS__)
#define APPLYX10(X, a0, ...) X(a0) APPLYX9(X, __VA_ARGS__)
#define APPLYX11(X, a0, ...) X(a0) APPLYX10(X, __VA_ARGS__)
#define APPLYX12(X, a0, ...) X(a0) APPLYX11(X, __VA_ARGS__)
#define APPLYX13(X, a0, ...) X(a0) APPLYX12(X, __VA_ARGS__)
#define APPLYX14(X, a0, ...) X(a0) APPLYX13(X, __VA_ARGS__)
#define APPLYX15(X, a0, ...) X(a0) APPLYX14(X, __VA_ARGS__)
#define APPLYX16(X, a0, ...) X(a0) APPLYX15(X, __VA_ARGS__)
#define APPLYX17(X, a0, ...) X(a0) APPLYX16(X, __VA_ARGS__)
#define APPLYX18(X, a0, ...) X(a0) APPLYX17(X, __VA_ARGS__)
#define APPLYX19(X, a0, ...) X(a0) APPLYX18(X, __VA_ARGS__)
#define APPLYX20(X, a0, ...) X(a0) APPLYX19(X, __VA_ARGS__)
#define APPLYX21(X, a0, ...) X(a0) APPLYX20(X, __VA_ARGS__)
#define APPLYX22(X, a0, ...) X(a0) APPLYX21(X, __VA_ARGS__)
#define APPLYX23(X, a0, ...) X(a0) APPLYX22(X, __VA_ARGS__)
#define APPLYX24(X, a0, ...) X(a0) APPLYX23(X, __VA_ARGS__)
#define APPLYX25(X, a0, ...) X(a0) APPLYX24(X, __VA_ARGS__)
#define APPLYX26(X, a0, ...) X(a0) APPLYX25(X, __VA_ARGS__)
#define APPLYX27(X, a0, ...) X(a0) APPLYX26(X, __VA_ARGS__)
#define APPLYX28(X, a0, ...) X(a0) APPLYX27(X, __VA_ARGS__)
#define APPLYX29(X, a0, ...) X(a0) APPLYX28(X, __VA_ARGS__)
#define APPLYX30(X, a0, ...) X(a0) APPLYX29(X, __VA_ARGS__)
#define APPLYX31(X, a0, ...) X(a0) APPLYX30(X, __VA_ARGS__)
#define APPLYX32(X, a0, ...) X(a0) APPLYX31(X, __VA_ARGS__)

/**
 * Call macro `W` on (k, arg) for each arg in the argument list.
 */
#define APPLYW_(M, k, ...) M(k, __VA_ARGS__)
#define APPLYW(W, k, ...) APPLYW_(PASTE(APPLYW, PP_NARG(__VA_ARGS__)), W, k, __VA_ARGS__)

#define APPLYW1(W,  k, a0) W(k, a0)
#define APPLYW2(W,  k, a0, ...) W(k, a0) APPLYW1(W, k, __VA_ARGS__)
#define APPLYW3(W,  k, a0, ...) W(k, a0) APPLYW2(W, k, __VA_ARGS__)
#define APPLYW4(W,  k, a0, ...) W(k, a0) APPLYW3(W, k, __VA_ARGS__)
#define APPLYW5(W,  k, a0, ...) W(k, a0) APPLYW4(W, k, __VA_ARGS__)
#define APPLYW6(W,  k, a0, ...) W(k, a0) APPLYW5(W, k, __VA_ARGS__)
#define APPLYW7(W,  k, a0, ...) W(k, a0) APPLYW6(W, k, __VA_ARGS__)
#define APPLYW8(W,  k, a0, ...) W(k, a0) APPLYW7(W, k, __VA_ARGS__)
#define APPLYW9(W,  k, a0, ...) W(k, a0) APPLYW8(W, k, __VA_ARGS__)
#define APPLYW10(W, k,  a0, ...) W(k, a0) APPLYW9(W, k, __VA_ARGS__)
#define APPLYW11(W, k,  a0, ...) W(k, a0) APPLYW10(W, k, __VA_ARGS__)
#define APPLYW12(W, k,  a0, ...) W(k, a0) APPLYW11(W, k, __VA_ARGS__)
#define APPLYW13(W, k,  a0, ...) W(k, a0) APPLYW12(W, k, __VA_ARGS__)
#define APPLYW14(W, k,  a0, ...) W(k, a0) APPLYW13(W, k, __VA_ARGS__)
#define APPLYW15(W, k,  a0, ...) W(k, a0) APPLYW14(W, k, __VA_ARGS__)
#define APPLYW16(W, k,  a0, ...) W(k, a0) APPLYW15(W, k, __VA_ARGS__)
#define APPLYW17(W, k,  a0, ...) W(k, a0) APPLYW16(W, k, __VA_ARGS__)
#define APPLYW18(W, k,  a0, ...) W(k, a0) APPLYW17(W, k, __VA_ARGS__)
#define APPLYW19(W, k,  a0, ...) W(k, a0) APPLYW18(W, k, __VA_ARGS__)
#define APPLYW20(W, k,  a0, ...) W(k, a0) APPLYW19(W, k, __VA_ARGS__)
#define APPLYW21(W, k,  a0, ...) W(k, a0) APPLYW20(W, k, __VA_ARGS__)
#define APPLYW22(W, k,  a0, ...) W(k, a0) APPLYW21(W, k, __VA_ARGS__)
#define APPLYW23(W, k,  a0, ...) W(k, a0) APPLYW22(W, k, __VA_ARGS__)
#define APPLYW24(W, k,  a0, ...) W(k, a0) APPLYW23(W, k, __VA_ARGS__)
#define APPLYW25(W, k,  a0, ...) W(k, a0) APPLYW24(W, k, __VA_ARGS__)
#define APPLYW26(W, k,  a0, ...) W(k, a0) APPLYW25(W, k, __VA_ARGS__)
#define APPLYW27(W, k,  a0, ...) W(k, a0) APPLYW26(W, k, __VA_ARGS__)
#define APPLYW28(W, k,  a0, ...) W(k, a0) APPLYW27(W, k, __VA_ARGS__)
#define APPLYW29(W, k,  a0, ...) W(k, a0) APPLYW28(W, k, __VA_ARGS__)
#define APPLYW30(W, k,  a0, ...) W(k, a0) APPLYW29(W, k, __VA_ARGS__)
#define APPLYW31(W, k,  a0, ...) W(k, a0) APPLYW30(W, k, __VA_ARGS__)
#define APPLYW32(W, k,  a0, ...) W(k, a0) APPLYW31(W, k, __VA_ARGS__)

// The preprocessor techniques below are explained at
// https://github.com/pfultz2/Cloak/wiki/C-Preprocessor-tricks,-tips,-and-idioms

#define IIF(c) PASTE(IIF_, c)
#define IIF_0(t, ...) __VA_ARGS__
#define IIF_1(t, ...) t

#define CHECK_N(x, n, ...) n
#define CHECK(...) CHECK_N(__VA_ARGS__, 0,)
#define PROBE(x) x, 1,

#define NOT(x) CHECK(PASTE(NOT_, x))
#define NOT_0 PROBE(~)

#define IS_PAREN(x) CHECK(IS_PAREN_PROBE x)
#define IS_PAREN_PROBE(...) PROBE(~)

#define CALL(arglist_, macro_) \
    IIF(IS_PAREN(arglist_))(macro_ arglist_, macro_(arglist_))

#define EMPTY()
#define DEFER(id) id EMPTY()
#define OBSTRUCT(...) __VA_ARGS__ DEFER(EMPTY)()
#define EXPAND(...) __VA_ARGS__

#define EVAL(...) EVAL1(EVAL1(EVAL1(__VA_ARGS__)))
#define EVAL1(...) EVAL2(EVAL2(EVAL2(__VA_ARGS__)))
#define EVAL2(...) EVAL3(EVAL3(EVAL3(__VA_ARGS__)))
#define EVAL3(...) EVAL4(EVAL4(EVAL4(__VA_ARGS__)))
#define EVAL4(...) EVAL5(EVAL5(EVAL5(__VA_ARGS__)))
#define EVAL5(...) __VA_ARGS__


#endif // PROTONES_UTIL_MACROS_H
