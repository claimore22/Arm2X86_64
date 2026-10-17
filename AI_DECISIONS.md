# AI_DECISIONS — append-only log of IR / scope decisions

2026-10-04 — ADDG required first IR extension: new `Expr::ChooseTag
{ start_tag: W4, tag_offset: W4, gcr: W64 } -> W4` + `SpecialReg::GcrEl1`.
Reason: Stanford `AArch64_ChooseNonExcludedTagOrZero` (exclusion-aware tag
selection) is not expressible with existing nodes; precedent is AddWithCarry
as a typed primitive. Tag extract (bits 59:56) and reinsertion use existing
Extract/Concat. GCR_EL1 is passed whole so Exclude field selection stays in
the lowering (bit positions not pinned in IR). Provenance now uses real
SHA-256 hashes (Get-FileHash) instead of placeholder strings.

2026-10-04 — ADDPT required second IR extension: new `Expr::PointerAddCheck
{ value: W64, base: W64 } -> W64`. Reason: Stanford `PointerAddCheck`
(top-byte mismatch makes result likely non-canonical) is deliberately
implementation-defined, so composing it from Extract/Select would invent
semantics; opaque typed primitive mirrors the ChooseTag precedent.
Plain `result = base + offset` uses existing BinOp::Add (no carry).

2026-10-04 — APAS (alias of SYS) required third IR extension: `Expr::SysOp
{ op0:W2, op1:W3, crn:W4, crm:W4, op2:W3, address:W64 } -> ()` statement-level
effect via `Stmt::Discard`. Reason: SYS family (APAS, AT, BRB, DC, IC, TLBI,
...) all lower to `AArch64_SysInstr(op0,op1,crn,crm,op2,t)` with the
register `t` as an address operand; one opaque node covers all aliases
without baking in specific cache/TLB semantics. Fixed op0='01' for
SYS-class; decode bindings for t/op1/crn/crm/op2. `FEAT_PAS` gate.

2026-10-04 — ASR (immediate) implemented as SBFM alias. Reason: Stanford
explicitly marks it `alias of SBFM`; SBFM test (`sbfm.rs`) covers the
semantics (bitfield move with sign extension). Alias decode would fix
`immr=0`, `imms=width-1-shift`, `N=0/1` for the specific encoding - no IR
extension needed. Decision only; no new test file.

2026-10-04 — ASR (register) implemented as ASRV alias. Reason: Stanford
explicitly marks it `alias of ASRV`; ASRV test (`asrv.rs`) covers the
variable-shift semantics (`ShiftReg` with `ASR`). Alias decode fixes
`shift_type=ASR` from `op2` - no IR extension needed. Decision only.

2026-10-04 — AT implemented as SYS alias using `SysOp`. Reason: `SysOp`
node added for APAS covers all SYS-class aliases (AT, BRB, DC, IC, TLBI,
...); AT fixes `op0='01', crn=7, crm=8..15, op1, op2` per encoding. No
new IR extension. 4 focused tests pass.

2026-10-04 — BRB implemented as SYS alias using `SysOp`. Reason: same
`SysOp` node; BRB fixes `op0='01', crn=9, op1, crm, op2` per encoding.
Requires `FEAT_BRBE`. No address operand (uses zero placeholder). 5
focused tests pass.

2026-10-04 — AUTDA/AUTDZA implemented using new `AuthDA { address:W64,
modifier:W64, auth_combined:Bool } -> W64` + `AuthIA`/`AuthIB` for
instruction-address variants. Reason: PAC (Pointer Authentication) is
a distinct architectural capability; `AuthDA` models `AuthDA(address,
modifier, auth_combined)` returning authenticated address or faulting.
`FEAT_PAuth` gate. No new test for AUTDZA (alias: modifier=zero).
4 focused tests pass.

2026-10-04 — AUTDB/AUTDZB implemented using new `AuthDB { address:W64,
modifier:W64, auth_combined:Bool } -> W64`. Reason: same PAC family,
key B instead of key A. `FEAT_PAuth` gate. No new test for AUTDZB.
4 focused tests pass.

2026-10-04 — AUTIA family (AUTIA, AUTIAZ, AUTIZA, AUTIA1716, AUTIASP)
implemented using `AuthIA` + `AuthIA2`. `AuthIA` handles single-modifier
cases (AUTIA, AUTIAZ, AUTIZA). `AuthIA2 { address, mod1, mod2, auth_combined }`
handles two-modifier cases (AUTIA1716/AUTIASP with FEAT_PAuth_LR && PACM=1).
`FEAT_PAuth` + optional `FEAT_PAuth_LR` gates. 12 focused tests pass.

2026-10-04 — AUTIA171615, AUTIASPPC, AUTIASPPCR implemented using
`AuthIA2` + `ReadSpecial::Pc64`. All require `FEAT_PAuth_LR`.
AUTIA171615: X17, mod1=X16, mod2=X15. AUTIASPPC: X30, mod1=SP, mod2=PC64-offset.
AUTIASPPCR: X30, mod1=SP, mod2=Xn. 3+3+3 focused tests pass.

2026-10-04 — AUTIB family (AUTIB, AUTIBZ, AUTIZB, AUTIB1716, AUTIBSP)
implemented using `AuthIB` + `AuthIB2`. Mirror of AUTIA with key B.
`AuthIB2 { address, mod1, mod2, auth_combined }` for two-modifier cases
(AUTIB1716/AUTIBSP with FEAT_PAuth_LR && PACM=1). `FEAT_PAuth` + optional
`FEAT_PAuth_LR` gates. 18 focused tests pass.

2026-10-05 — BLRAA, BLRAAZ, BLRAB, BLRABZ implemented using existing
`AuthIA`/`AuthIB` + `WriteReg X30 = PC+4` + `Branch`. Reason: Stanford
`blra.md` is BLR plus `AuthIA/AuthIB(target, modifier, TRUE)`; no IR
extension needed (GCS/BTypeNext omitted like BLR). Variants: key A/B via
AuthIA/AuthIB, modifier Xm vs zero (Const 0). `FEAT_PAuth` gate.
5 focused tests pass (tests/blraa.rs).

2026-10-05 — BRAA, BRAAZ, BRAB, BRABZ implemented using existing
`AuthIA`/`AuthIB` + `Branch`. Reason: Stanford `bra.md` is BR plus
`AuthIA/AuthIB(target, modifier, TRUE)`; no IR extension needed
(BTypeNext omitted like BR). Variants: key A/B, modifier Xm vs zero.
`FEAT_PAuth` gate. 5 focused tests pass (tests/braa.rs).

2026-10-05 — BRK implemented using existing `Stmt::Let(comment:W16)` +
`Stmt::RaiseException(SoftwareException)`. Reason: Stanford `brk.md`
`AArch64_SoftwareBreakpoint(comment)` is EC 0x3C breakpoint with ISS =
imm16; no IR extension needed (immediate preserved via Let, precedent
is BLR/BR omitting BType). No feature gate. 4 focused tests pass
(tests/brk.rs).

2026-10-05 — BTI implemented using existing `Stmt::Let(targets:W2)`. Reason:
Stanford `bti.md` sets `BTypeNext = '00'` and checks compatibility via
`BTypeCompatible_BTI(op2[2:1])`. In the IR, BType/BTypeNext is omitted
(established precedent in BLR, BR, BRK); the indirection targets operand
`op2[2:1]` (absent=00, c=01, j=10, jc=11) is captured in decode and
preserved in semantics via `Stmt::Let`. Requires `FEAT_BTI`. 4 focused
tests pass (tests/bti.rs).

2026-10-05 — CAS, CASA, CASAL, CASL implemented using new `Expr::MemAtomic
{ address, expected, new_value, size, ordering } -> W32/W64` + `WriteReg`.
Reason: Stanford `cas.md` uses `MemAtomic(address, comparevalue, newvalue, accdesc)`
for atomic compare-and-swap; existing IR has no variable reference to tie a
load to a subsequent conditional store, so atomic read-modify-write returning
the original memory value requires an explicit atomic memory node (precedent is
MemRead/MemWrite). Variants: CAS (relaxed), CASA (acquire), CASL (release),
CASAL (acquire-release) in 32-bit and 64-bit. Requires `FEAT_LSE`. 5 focused
tests pass (tests/cas.rs).

2026-10-05 — CASB, CASAB, CASALB, CASLB implemented using existing `Expr::MemAtomic
{ size: Bits8, ... }` + `Expr::ZeroExtend` to W32 + `WriteReg`. Reason: Stanford
`casb.md` specifies `MemAtomic(address, comparevalue, newvalue, accdesc)` with
8-bit operands followed by `ZeroExtend(data, 32)` into Ws; directly reuses the
`MemAtomic` IR node introduced for CAS. Variants: CASB (relaxed), CASAB (acquire),
CASLB (release), CASALB (acquire-release). Requires `FEAT_LSE`. 5 focused tests
pass (tests/casb.rs).

2026-10-05 — CASH, CASAH, CASALH, CASLH implemented using existing `Expr::MemAtomic
{ size: Bits16, ... }` + `Expr::ZeroExtend` to W32 + `WriteReg`. Reason: Stanford
`cash.md` specifies `MemAtomic(address, comparevalue, newvalue, accdesc)` with
16-bit operands followed by `ZeroExtend(data, 32)` into Ws; directly reuses the
`MemAtomic` IR node introduced for CAS. Variants: CASH (relaxed), CASAH (acquire),
CASLH (release), CASALH (acquire-release). Requires `FEAT_LSE`. 5 focused tests
pass (tests/cash.rs).

2026-10-05 — CASP, CASPA, CASPAL, CASPL implemented using existing `Expr::Concat` +
`Expr::MemAtomic { size: Bits64/Bits128, ... }` + `Expr::Extract` + `WriteReg`.
Reason: Stanford `casp.md` specifies pair comparison (`s2::s1`, `t2::t1`) and pair
writeback via `data[datasize-1:0]` and `data[2*datasize-1:datasize]`; reuses `Concat`,
`Extract`, and the `MemAtomic` IR node with 64-bit and 128-bit memory sizes.
Variants: CASP (relaxed), CASPA (acquire), CASPL (release), CASPAL (acquire-release)
in 32-bit (pair 64) and 64-bit (pair 128). Requires `FEAT_LSE`. 5 focused tests
pass (tests/casp.rs).

2026-10-05 — CASPT, CASPAT, CASPALT, CASPLT implemented using existing `Expr::Concat` +
`Expr::MemAtomic { size: Bits128, ... }` + `Expr::Extract` + `WriteReg`.
Reason: Stanford `caspt.md` specifies unprivileged compare-and-swap on a 64-bit pair
(128-bit memory operand); mirrors CASP 64-bit pair semantics with FEAT_LSUI feature gate.
Variants: CASPT (relaxed), CASPAT (acquire), CASPLT (release), CASPALT (acquire-release).
Requires `FEAT_LSUI`. 5 focused tests pass (tests/caspt.rs).

2026-10-05 — CAST, CASAT, CASALT, CASLT implemented using existing `Expr::MemAtomic
{ size: Bits64, ... }` + `WriteReg`. Reason: Stanford `cast.md` specifies unprivileged
compare-and-swap on a 64-bit doubleword; mirrors CAS 64-bit semantics with FEAT_LSUI
feature gate. Variants: CAST (relaxed), CASAT (acquire), CASLT (release), CASALT
(acquire-release). Requires `FEAT_LSUI`. 5 focused tests pass (tests/cast.rs).

2026-10-05 — CB<cc> (immediate) implemented using existing `BinOp` comparisons +
`ReadSpecial::Pc64` + `Stmt::BranchConditional`. Reason: Stanford `cbcc_imm.md`
compares register with immediate (signed/unsigned GT, LT, EQ, NE) and branches to
`PC64 + offset`; all comparison operators exist in `BinOp` (`GtSigned`, `LtSigned`,
`GtUnsigned`, `LtUnsigned`, `Eq`, `Ne`) and `BranchConditional` models conditional
branch without flags effect. Requires `FEAT_CMPBR`. 4 focused tests pass
(tests/cbcc_imm.rs).

2026-10-05 — CB<cc> (register) implemented using existing `BinOp` comparisons +
`ReadSpecial::Pc64` + `Stmt::BranchConditional`. Reason: Stanford `cbcc_regs.md`
compares two registers (signed/unsigned GT, GE, EQ, NE) and branches to
`PC64 + offset`; all comparison operators exist in `BinOp` (`GtSigned`, `GeSigned`,
`GtUnsigned`, `GeUnsigned`, `Eq`, `Ne`) and `BranchConditional` models conditional
branch without flags effect. Requires `FEAT_CMPBR`. 4 focused tests pass
(tests/cbcc_regs.rs).

2026-10-05 — CBB<cc> implemented using existing `BinOp` comparisons on W8 operands +
`ReadSpecial::Pc64` + `Stmt::BranchConditional`. Reason: Stanford `cbbcc_regs.md`
compares byte values in two registers (signed/unsigned GT, GE, EQ, NE) and branches
to `PC64 + offset`; reuses existing `BinOp` (`GtSigned`, `GeSigned`, `GtUnsigned`,
`GeUnsigned`, `Eq`, `Ne`) with `BitWidth::W8` and `BranchConditional`. Aliases:
CBBLE, CBBLO, CBBLS, CBBLT resolve to canonical `CBB<cc>` with inverted register
operands per Stanford spec (`CBBLE Rt, Rm == CBBGE Rm, Rt`, etc.). Requires
`FEAT_CMPBR`. 5 focused tests pass (tests/cbbcc.rs).

2026-10-05 — CBH<cc> implemented using existing `BinOp` comparisons on W16 operands +
`ReadSpecial::Pc64` + `Stmt::BranchConditional`. Reason: Stanford `cbhcc_regs.md`
compares halfword values in two registers (signed/unsigned GT, GE, EQ, NE) and branches
to `PC64 + offset`; reuses existing `BinOp` (`GtSigned`, `GeSigned`, `GtUnsigned`,
`GeUnsigned`, `Eq`, `Ne`) with `BitWidth::W16` and `BranchConditional`. Aliases:
CBHLE, CBHLO, CBHLS, CBHLT resolve to canonical `CBH<cc>` with inverted register
operands per Stanford spec (`CBHLE Rt, Rm == CBHGE Rm, Rt`, etc.). Requires
`FEAT_CMPBR`. 5 focused tests pass (tests/cbhcc.rs).

2026-10-05 — CINC implemented as CSINC alias. Reason: Stanford `cinc_csinc.md`
explicitly marks it `alias of CSINC`; CINC returns Rn+1 if condition true, else Rn.
This maps to CSINC with Rm=Rn and inverted condition: CSINC Rd, Rn, Rn, !cond
= if !cond then Rn else Rn+1 = if cond then Rn+1 else Rn. No IR extension needed.
CSINC test (`csinc.rs`) covers the core semantics; focused test `cinc.rs` validates
the alias mapping (inverted condition, same register for both inputs). 4 focused tests
pass (tests/cinc.rs).

2026-10-05 — CINV implemented as CSINV alias. Reason: Stanford `cinv_csinv.md`
explicitly marks it `alias of CSINV`; CINV returns NOT(Rn) if condition true, else Rn.
This maps to CSINV with Rm=Rn and inverted condition: CSINV Rd, Rn, Rn, !cond
= if !cond then Rn else NOT(Rn) = if cond then NOT(Rn) else Rn. No IR extension needed.
CSINV test (`csinv.rs`) covers the core semantics; focused test `cinv.rs` validates
the alias mapping (inverted condition, same register for both inputs, NOT on true branch).
3 focused tests pass (tests/cinv.rs).

2026-10-05 — CLRBHB implemented as hint instruction. Reason: Stanford `clrbhb.md`
specifies `Hint_CLRBHB()` with FEAT_CLRBHB gate. No IR extension needed - modeled as
`Stmt::Let(hint: W8)` to preserve the hint type in semantics (precedent: BTI uses
`Stmt::Let` for target bits). 3 focused tests pass (tests/clrbhb.rs).

2026-10-05 — CMN (extended register) implemented as ADDS alias. Reason: Stanford
`cmn_adds_addsub_ext.md` explicitly marks it `alias of ADDS (extended register)`;
CMN adds register + extended register, updates flags, discards result. Reuses
ADDS semantics with ExtendReg + AddWithCarry but omits the result register write,
only extracting NZCV flags from tuple element 1. No IR extension needed. ADDS
test (`adds_ext_reg.rs`) covers core semantics; focused test `cmn_ext_reg.rs`
validates alias (flags only, no result write). 3 focused tests pass
(tests/cmn_ext_reg.rs).

2026-10-05 — CMN (immediate) implemented as ADDS alias. Reason: Stanford
`cmn_adds_addsub_imm.md` explicitly marks it `alias of ADDS (immediate)`;
CMN adds register + immediate, updates flags, discards result. Reuses
ADDS semantics with ZeroExtend immediate + AddWithCarry but omits the result register write,
only extracting NZCV flags from tuple element 1. No IR extension needed. ADDS
test (`adds_imm.rs`) covers core semantics; focused test `cmn_imm.rs`
validates alias (flags only, no result write). 3 focused tests pass
(tests/cmn_imm.rs).

2026-10-05 — CMP (extended register) implemented as SUBS alias. Reason: Stanford
`cmp_subs_addsub_ext.md` explicitly marks it `alias of SUBS (extended register)`;
CMP subtracts extended register from register, updates flags, discards result. Reuses
SUBS semantics with ExtendReg + Not + AddWithCarry(carry=1) but omits the result register write,
only extracting NZCV flags from tuple element 1. No IR extension needed. SUBS
test (`subs_ext_reg.rs`) covers core semantics; focused test `cmp_ext_reg.rs`
validates alias (flags only, no result write, uses Not and carry=true). 3 focused tests pass
(tests/cmp_ext_reg.rs).

2026-10-05 — CMP (immediate) implemented as SUBS alias. Reason: Stanford
`cmp_subs_addsub_imm.md` explicitly marks it `alias of SUBS (immediate)`;
CMP subtracts immediate from register, updates flags, discards result. Reuses
SUBS semantics with ZeroExtend + Not + AddWithCarry(carry=1) but omits the result register write,
only extracting NZCV flags from tuple element 1. No IR extension needed. SUBS
test (`subs_imm.rs`) covers core semantics; focused test `cmp_imm.rs`
validates alias (flags only, no result write, uses Not and carry=true). 3 focused tests pass
(tests/cmp_imm.rs).

2026-10-05 — CMP (shifted register) implemented as SUBS alias. Reason: Stanford
`cmp_subs_addsub_shift.md` explicitly marks it `alias of SUBS (shifted register)`;
CMP subtracts shifted register from register, updates flags, discards result. Reuses
SUBS semantics with ShiftReg + Not + AddWithCarry(carry=1) but omits the result register write,
only extracting NZCV flags from tuple element 1. No IR extension needed. SUBS
test (`subs_shifted.rs`) covers core semantics; focused test `cmp_shifted.rs`
validates alias (flags only, no result write, uses Shift, Not and carry=true). 3 focused tests pass
(tests/cmp_shifted.rs).

2026-10-05 — SUBPS implemented as new instruction (canonical for CMPP alias). Reason: Stanford
`subps.md` specifies subtraction of 56-bit addresses (SignExtend[55:0]) with NOT on operand2,
AddWithCarry(carry=1), writing result and flags. Requires FEAT_MTE. Uses Extract(lsb=0, width=56)
+ SignExtend + Not + AddWithCarry. New IR extension not needed. 4 focused tests pass
(tests/subps.rs).

2026-10-05 — CMPP implemented as SUBPS alias. Reason: Stanford `cmpp_subps.md` explicitly marks it
`alias of SUBPS`; CMPP subtracts 56-bit addresses (SignExtend[55:0]) with NOT on operand2,
AddWithCarry(carry=1), updates flags, discards result. Reuses SUBPS semantics but omits the
result register write, only extracting NZCV flags from tuple element 1. Requires FEAT_MTE. No IR
extension needed. SUBPS test (`subps.rs`) covers core semantics; focused test `cmpp.rs` validates
alias (flags only, no result write, uses Extract, SignExtend, Not and carry=true). 4 focused
tests pass (tests/cmpp.rs).

2026-10-05 — CNEG implemented as CSNEG alias. Reason: Stanford `cneg_csneg.md` explicitly marks it
`alias of CSNEG`; CNEG returns negated source if condition true, else source. Maps to CSNEG with
Rm=Rn and inverted condition: CSNEG Rd, Rn, Rn, !cond = if !cond then Rn else NOT(Rn)+1 = if cond
then -Rn else Rn. No IR extension needed. CSNEG test (`csneg.rs`) covers core semantics; focused
test `cneg.rs` validates alias mapping (inverted condition, same register for both inputs, negation
on false branch). 3 focused tests pass (tests/cneg.rs).

2026-10-05 — CNT implemented with new UnaryOp::PopCnt. Reason: Stanford `cnt.md` specifies
BitCount(operand1) which counts set bits in the source register. Population count is not
expressible with existing IR nodes (Not, Neg); adding PopCnt to UnaryOp is a minimal IR extension
following the pattern of typed primitives like AddWithCarry. Requires FEAT_CSSC. No alias.
4 focused tests pass (tests/cnt.rs).

2026-10-05 — COSP implemented as SYS alias using existing SysOp. Reason: Stanford `cosp_sys.md`
explicitly marks it `alias of SYS`; COSP RCTX prevents speculative predictions from other contexts.
Reuses SysOp node (added for APAS) with fixed encoding: op0='01', op1='001', CRn=7, CRm=0, op2='001',
zero address placeholder. Requires FEAT_SPECRES2. No new IR extension. 5 focused tests pass
(tests/cosp.rs).

2026-10-05 — CPP implemented as SYS alias using existing SysOp. Reason: Stanford `cpp_sys.md`
explicitly marks it `alias of SYS`; CPP RCTX prevents cache prefetch predictions from other contexts.
Reuses SysOp node (added for APAS) with fixed encoding: op0='01', op1='001', CRn=7, CRm=1, op2='001',
zero address placeholder. Requires FEAT_SPECRES2. Differs from COSP only in CRm=1 vs CRm=0. No new
IR extension. 5 focused tests pass (tests/cpp.rs).

2026-10-05 — CPYFP, CPYFM, CPYFE implemented as MOPS memory copy family. Reason: Stanford
`cpyfp.md` specifies a three-stage forward memory copy (prologue/main/epilogue) using FEAT_MOPS.
The operation reads Rd (destination), Rs (source), Rn (size), and options from op2 (temporal hints,
privilege). Each stage performs implementation-defined portions of the copy. In the IR, modeled
using MemRead/MemWrite with AccessDescriptor placeholders, register updates for
address/size progression, and NZCV flag updates. Requires FEAT_MOPS. No new IR extension needed
(uses existing MemRead/MemWrite/MemSize/MemOrdering). 5 focused tests pass (tests/cpyfp.rs).

2026-10-05 — CPYFPN, CPYFMN, CPYFEN implemented as MOPS memory copy family (non-temporal). Reason: Stanford
`cpyfpn.md` specifies the same three-stage forward memory copy as CPYFP but with both read and write
non-temporal (options[3]=1, options[2]=1). Reuses the same IR structure with options=0b1100.
Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpyfpn.rs).

2026-10-05 — CPYFPRN, CPYFMRN, CPYFERN implemented as MOPS memory copy family (read non-temporal). Reason: Stanford
`cpyfprn.md` specifies the same three-stage forward memory copy as CPYFP but with read non-temporal only
(options[3]=1, options[2]=0). Reuses the same IR structure with options=0b1000. Requires FEAT_MOPS. No new
IR extension. 6 focused tests pass (tests/cpyfprn.rs).

2026-10-05 — CPYFPRT, CPYFMRT, CPYFERT implemented as MOPS memory copy family (read unprivileged). Reason: Stanford
`cpyfprt.md` specifies the same three-stage forward memory copy as CPYFP but with read unprivileged
(options[1]=1 => rprivileged=0, options[0]=0 => wprivileged=1). Both temporal (options[3]=0, options[2]=0).
Reuses the same IR structure with options=0b0010. Requires FEAT_MOPS. No new IR extension. 6 focused
tests pass (tests/cpyfprt.rs).

2026-10-05 — CPYFPRTN, CPYFMRTN, CPYFERTN implemented as MOPS memory copy family (read unprivileged and non-temporal). 
Reason: Stanford
`cpyfprtn.md` specifies the same three-stage forward memory copy as CPYFP but with read unprivileged and
non-temporal (options[1]=1 => rprivileged=0, options[3]=1 => rnontemporal=1, options[2]=1 => wnontemporal=1,
options[0]=0 => wprivileged=1). Reuses the same IR structure with options=0b1110. Requires FEAT_MOPS. No new
IR extension. 6 focused tests pass (tests/cpyfprtn.rs).

2026-10-05 — CPYFPRTRN, CPYFMRTRN, CPYFERTRN implemented as MOPS memory copy family (read unprivileged and 
non-temporal). Reason: Stanford
`cpyfprtrn.md` specifies the same three-stage forward memory copy as CPYFP but with read unprivileged and
non-temporal (options[1]=1 => rprivileged=0, options[3]=1 => rnontemporal=1, options[2]=0 => wnontemporal=0,
options[0]=0 => wprivileged=1). Reuses the same IR structure with options=0b1010. Requires FEAT_MOPS. No new
IR extension. 6 focused tests pass (tests/cpyfprtrn.rs).

2026-10-05 — CPYFPRTWN, CPYFMRTWN, CPYFERTWN implemented as MOPS memory copy family (read unprivileged, write 
non-temporal). Reason: Stanford
`cpyfprtwn.md` specifies the same three-stage forward memory copy as CPYFP but with read unprivileged
(options[1]=1 => rprivileged=0), write unprivileged (options[0]=1 => wprivileged=0), write non-temporal
(options[2]=1 => wnontemporal=1), read temporal (options[3]=0 => rnontemporal=0). Reuses the same IR
structure with options=0b0111. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass
(tests/cpyfprtwn.rs).

2026-10-05 — CPYFPT, CPYFMT, CPYFET implemented as MOPS memory copy family (both read and write unprivileged). Reason: 
Stanford
`cpyfpt.md` specifies the same three-stage forward memory copy as CPYFP but with both read and write
unprivileged (options[1]=1 => rprivileged=0, options[0]=1 => wprivileged=0). Both temporal
(options[3]=0 => rnontemporal=0, options[2]=0 => wnontemporal=0). Reuses the same IR structure with
options=0b0011. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpyfpt.rs).

2026-10-05 — CPYFPTN, CPYFMTN, CPYFETN implemented as MOPS memory copy family (both read and write unprivileged and 
non-temporal). Reason: Stanford
`cpyfptn.md` specifies the same three-stage forward memory copy as CPYFP but with both read and write
unprivileged (options[1]=1 => rprivileged=0, options[0]=1 => wprivileged=0) and non-temporal
(options[3]=1 => rnontemporal=1, options[2]=1 => wnontemporal=1). Reuses the same IR structure with
options=0b1111. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpyfptn.rs).

2026-10-05 — CPYFPTRN, CPYFMTRN, CPYFETRN implemented as MOPS memory copy family (both read and write unprivileged, 
read non-temporal). Reason: Stanford
`cpyfptrn.md` specifies the same three-stage forward memory copy as CPYFP but with both read and write
unprivileged (options[1]=1 => rprivileged=0, options[0]=1 => wprivileged=0), read non-temporal
(options[3]=1 => rnontemporal=1), write temporal (options[2]=0 => wnontemporal=0). Reuses the same IR
structure with options=0b1011. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass
(tests/cpyfptrn.rs).

2026-10-05 — CPYFPTWN, CPYFMTWN, CPYFETWN implemented as MOPS memory copy family (both read and write unprivileged, 
write non-temporal). Reason: Stanford
`cpyfptwn.md` specifies the same three-stage forward memory copy as CPYFP but with both read and write
unprivileged (options[1]=1 => rprivileged=0, options[0]=1 => wprivileged=0), write non-temporal
(options[2]=1 => wnontemporal=1), read temporal (options[3]=0 => rnontemporal=0). Reuses the same IR
structure with options=0b0111. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass
(tests/cpyfptwn.rs).

2026-10-05 — CPYFPWN, CPYFMWN, CPYFEWN implemented as MOPS memory copy family (both read and write privileged, write non-temporal). Reason: Stanford
`cpyfpwn.md` specifies the same three-stage forward memory copy as CPYFP but with both read and write
privileged (options[1]=0 => rprivileged=1, options[0]=0 => wprivileged=1), write non-temporal
(options[2]=1 => wnontemporal=1), read temporal (options[3]=0 => rnontemporal=0). Reuses the same IR
structure with options=0b0100. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass
(tests/cpyfpwn.rs).

2026-10-05 — CPYFPWT, CPYFMWT, CPYFEWT implemented as MOPS memory copy family (read privileged, write unprivileged). Reason: Stanford
`cpyfpwt.md` specifies the same three-stage forward memory copy as CPYFP but with read privileged
(options[1]=0 => rprivileged=1), write unprivileged (options[0]=1 => wprivileged=0). Both temporal
(options[3]=0 => rnontemporal=0, options[2]=0 => wnontemporal=0). Reuses the same IR structure with
options=0b0001. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpyfpwt.rs).

2026-10-05 — CPYFPWTN, CPYFMWTN, CPYFEWTN implemented as MOPS memory copy family (write unprivileged, reads and writes non-temporal). Reason: Stanford
`cpyfpwtn.md` specifies the same three-stage forward memory copy as CPYFP but with write unprivileged
(options[0]=1 => wprivileged=0), both non-temporal (options[3]=1 => rnontemporal=1, options[2]=1 => wnontemporal=1),
read privileged (options[1]=0 => rprivileged=1). Reuses the same IR structure with
options=0b1101. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpyfpwtn.rs).

tests pass (tests/cpyfpwtn.rs).

2026-10-05 — CPYFPWTRN, CPYFMWTRN, CPYFEWTRN implemented as MOPS memory copy family (write unprivileged, read non-temporal). Reason: Stanford
`cpyfpwtrn.md` specifies the same three-stage forward memory copy as CPYFP but with write unprivileged
(options[0]=1 => wprivileged=0), read non-temporal (options[3]=1 => rnontemporal=1), read privileged
(options[1]=0 => rprivileged=1), write temporal (options[2]=0 => wnontemporal=0). Reuses the same IR
structure with options=0b1001. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpyfpwtrn.rs).

2026-10-05 — CPYFPWTWN, CPYFMWTWN, CPYFEWTWN implemented as MOPS memory copy family (write unprivileged and non-temporal). Reason: Stanford
`cpyfpwtwn.md` specifies the same three-stage forward memory copy as CPYFP but with write unprivileged
(options[0]=1 => wprivileged=0), write non-temporal (options[2]=1 => wnontemporal=1), read privileged
(options[1]=0 => rprivileged=1), read temporal (options[3]=0 => rnontemporal=0). Reuses the same IR
structure with options=0b0111. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpyfpwtwn.rs).

tests pass (tests/cpyfpwtwn.rs).

2026-10-05 — CPYP, CPYM, CPYE implemented as MOPS memory copy family (base version, bidirectional). Reason: Stanford
`cpyp.md` specifies a three-stage memory copy (prologue/main/epilogue) using FEAT_MOPS with options for
privilege and temporal hints. Base version uses both read/write privileged and both temporal
(options=0b0000). Unlike forward-only variants, this version supports both forward and backward copy
directions via implements_option_a flag, and uses unsigned cpysize in prologue vs signed in main/epilogue.
Reuses the same IR structure with MemRead/MemWrite. Requires FEAT_MOPS. No new IR extension. 6 focused
tests pass (tests/cpyp.rs).

2026-10-05 — CPYPN, CPYMN, CPYEN implemented as MOPS memory copy family (non-temporal). Reason: Stanford
`cpypn.md` specifies the same three-stage bidirectional memory copy as CPYP but with both read and write
non-temporal (options[3]=1 => rnontemporal=1, options[2]=1 => wnontemporal=1). Both privileged
(options[1]=0 => rprivileged=1, options[0]=0 => wprivileged=1). Reuses the same IR structure with
options=0b1100. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpypn.rs).

2026-10-05 — CPYPRN, CPYMRN, CPYERN implemented as MOPS memory copy family (read non-temporal). Reason: Stanford
`cpyprn.md` specifies the same three-stage bidirectional memory copy as CPYP but with read non-temporal
(options[3]=1 => rnontemporal=1), write temporal (options[2]=0 => wnontemporal=0). Both privileged
(options[1]=0 => rprivileged=1, options[0]=0 => wprivileged=1). Reuses the same IR structure with
options=0b1000. Requires FEAT_MOPS. No new IR extension. 6 focused tests pass (tests/cpyprn.rs).

(End of file - total 633 lines)
2026-10-05 - CRC32B/CRC32H/CRC32W/CRC32X and CRC32C family required IR extension: new
`Expr::Crc32 { acc: W32, value: data_width, data_width: BitWidth, crc32c: bool } -> W32`
(opaque; Poly32Mod2/BitReverse have no composing IR ops, and the polynomial choice is
implementation detail pinned by crc32c flag: 0x04C11DB7 vs 0x1EDC6F41). Reason: follows
ChkFeat/ChooseTag precedent for opaque architectural functions. Validator enforces acc=W32,
value width == data_width (8/16/32/64), result W32. 7 focused tests each
(tests/crc32.rs, tests/crc32c.rs).

2026-10-05 - CPYPRT/CPYMRT/CPYERT and remaining CPY* MOPS variants (CPYPRTN, CPYPRTRN,
CPYPRTWN, CPYPT, CPYPTN, CPYPTRN, CPYPTWN, CPYPWN, CPYPWT, CPYPWTN, CPYPWTRN, CPYPWTWN)
implemented with the same IR structure as CPYP/CPYPRN, differing only in the 4-bit op2
options ([3]=rnontemporal, [2]=wnontemporal, [1]=runprivileged, [0]=wunprivileged).
Reason: identical pseudocode; only access-descriptor option bits differ. No new IR
extension. 6 focused tests per variant.

2026-10-05 - CSET modeled as alias of CSINC (Select(ConditionHolds, 1, 0)); CSETM modeled
as alias of CSINV (Select(ConditionHolds, all-ones, 0)). No new IR extension.
(tests/cset.rs, tests/csetm.rs - 3 tests each).

2026-10-05 - CSDB, DMB, DSB, DGH, DRPS, DCPS1/2/3, ESB modeled as placeholder
`Stmt::Let` markers: their pseudocode bodies (barriers/hints/debug-state ops) have no
register or flag effects expressible in IR; same precedent as CLRBHB. Feature gates
recorded in required_features (FEAT_DGH, FEAT_RAS). No new IR extension.

2026-10-05 - DC and DVP modeled as SYS aliases reusing `Expr::SysOp` (op0=0b01 for
cache maintenance; DVP RCTX op1/CRn/CRm/op2 = 0b011/0b0111/0b0011/0b101).
No new IR extension (tests/dc.rs, tests/dvp.rs).

2026-10-05 - ERETAA/ERETAB modeled as `Stmt::Branch` to `Expr::AuthIA`/`Expr::AuthIB`
of ELR_ELx (modeled via ReadSpecial Pc, same as ERET) with SP modifier and
auth_then_branch=true. Requires FEAT_PAuth. No new IR extension (tests/eretaa.rs).

2026-10-05 - EON (shifted register): Xd = Xn XOR NOT(shift(Xm)), direct composition of
UnaryOp::Not + BinOp::Xor + Expr::Shift. No new IR extension (tests/eon_shifted.rs).

2026-10-05 - CTZ uses placeholder Binary op for CountLeadingZeroBits(BitReverse(x)),
same precedent as CLZ. Requires FEAT_CSSC (tests/ctz.rs).

2026-10-05 - GCSPOPM and GCSSS2 are SYSL aliases (return a result in Xt), which required
a new IR node: `Expr::SysLOp { op0:W2, op1:W3, crn:W4, crm:W4, op2:W3, address:W64 } -> W64`
mirroring `Expr::SysOp` (AArch64_SysInstrWithResult). Other GCS SYS aliases (GCSPOPCX,
GCSPOPX, GCSPUSHM, GCSPUSHX, GCSSS1) reuse SysOp with Discard. All require FEAT_GCS.
GCSB modeled as placeholder Let (barrier, no state effects). GCSSTR/GCSSTTR modeled as
MemWrite of Xt to X(n) (unprivileged access noted for GCSSTTR). Tests: 3 per instruction.

2026-10-05 - IRG required new IR node: `Expr::ChooseTagOrZero { exclude: W64 } -> W4`
models `AArch64_ChooseTagOrZero` (implementation-chosen non-excluded tag); OR with
GCR_EL1.Exclude stays in the lowering. AddressWithAllocationTag composed from
And/Or/Shift (tag into bits 59:56). Requires FEAT_MTE (tests/irg.rs, 4 tests).

2026-10-05 - GMI modeled as Xd = Xm OR (1 << UInt(address[59:56])) (tag mask insert),
FEAT_MTE. HINT/HLT/HVC/ISB are hint/debug/exception/barrier ops with placeholders
(same precedent as CSDB). IC modeled as SYS alias via `Expr::SysOp`. No further IR
extension.

2026-10-05 - LDADD/LDADDB/LDADDH families required new IR node:
`Expr::MemAtomicRMW { op: AtomicRMWOp, address, value, size, ordering } -> bits(size)`
for non-CAS `MemAtomic` operations (`AtomicRMWOp` = Add/Bic/Eor/Orr/Smax/Smin/
Umax/Umin/Swp, mirrors Stanford `MemAtomicOp_*`). Returns the originally loaded value;
sub-word results are ZeroExtended to 64-bit in the destination. Reused by upcoming
LDCLR/LDEOR/LDSET/LDSMAX/LDSMIN/LDUMAX/LDUMIN/SWP families. LD64B modeled as eight
MemRead+WriteReg pairs at consecutive 8-byte offsets (FEAT_LS64).

2026-10-05 - LDCLR/LDEOR/LDSET/LDSMAX/LDSMIN/LDUMAX/LDUMIN families (word/dword + byte +
halfword each) all reuse `Expr::MemAtomicRMW` with AtomicRMWOp Bic/Eor/Orr/Smax/Smin/
Umax/Umin (FEAT_LSE). LDCLRP uses MemAtomicRMW at 128-bit with Concat/Extract register
pairs (FEAT_LSE128). LDG/LDGM required new node `Expr::MemTagRead { address: W64 } ->
W4` (AArch64_MemTag), with tag insertion via And/Or/Shift (tests/ldg.rs). Exclusive
loads (LDAXR family, LDATXR, LDAXP) modeled as exclusive-monitor placeholder
`Stmt::Let` + MemRead + ZeroExtend; pair forms emit two WriteReg.

2026-10-05 - PAC family required new IR nodes: `Expr::AddPAC { kind: PacKind, address,
modifier }` and `Expr::AddPAC2 { kind, address, modifier1, modifier2 }` model
AddPACI{A,B}/AddPACD{A,B}/AddPACGA and the two-modifier LR forms (QARMA cipher is
opaque in the IR, same precedent as the Auth* nodes). `PacKind` = IA/IB/DA/DB/GA.
PACIA/PACIB families, PACDA/DZA/PACDB/DZB, PACGA (tests/pac*.rs, 3 tests each).
PACM/PSB/PSSBB are placeholder Lets. PRFM/PRFUM modeled as MemRead + Discard.

2026-10-05 - Alias rows modeled with canonical semantics: LSL/LSR imm & reg (UBFM/
LSLV/LSRV), MUL (MADD+XZR), MNEG (MSUB), MVN (ORN), NEG (SUB), NEGS (SUBS),
NGC/NGCS (SBC/SBCS via AddWithCarry with C), MOV x5 (ORR imm/reg, MOVN, MOVZ,
ADD to/from SP), NOP (Let). MADDPT/MSUBPT use Mul+Add/Sub + `Expr::PointerAddCheck`
(FEAT_CPA). MRS/MRRS use ReadSpecial; MSR(reg)/MSRR use WriteSpecial; MSR(imm)
placeholder Let. No further IR extension.

2026-10-06 — INCB, INCD, INCH, INCW (scalar) implemented using new `SVESpecialOp::DecodePredCount`
to model `DecodePredCount(pat, esize)` which computes the number of active predicate elements
for a given pattern and element size. This is an opaque architectural function (like ChkFeat,
ChooseTag) that cannot be composed from existing IR ops. The instruction computes:
`X[dn] = X[dn] + (DecodePredCount(pat, esize) * (imm4 + 1))`. Uses `Expr::SVESpecial` with
operands: pattern (5-bit), esize (8-bit: 8/16/32/64 for B/H/W/D). Returns 64-bit count.
No new IR node types needed - reuses SVESpecial. Requires FEAT_SVE/FEAT_SME. 9 focused tests
pass (tests/incb_scalar_sve.rs).

2026-10-06 — INCD, INCH, INCW (vector) implemented using same `SVESpecialOp::DecodePredCount`
for the predicate count, then broadcasting the scalar increment to all vector elements via
`SVESpecialOp::Dup` (which replicates a scalar across a scalable vector). The instruction computes:
`Z[dn] = Z[dn] + splat(DecodePredCount(pat, esize) * (imm4 + 1))`. Uses `ScalableVectorMap2`
with `Add` for element-wise addition. Esize varies: 64 (INCD), 16 (INCH), 32 (INCW).
Reuses existing SVESpecial and ScalableVectorMap2. Requires FEAT_SVE/FEAT_SME. 6 focused tests
pass (tests/incd_vector_sve.rs).

2026-10-06 — INCP (scalar) and INCP (vector) implemented using new `SVESpecialOp::CountPred`
to model counting true elements in a predicate register (`ActivePredicateElement` loop in
pseudocode). This is an opaque architectural function that cannot be composed from existing
IR ops. The scalar variant computes `X[dn] = X[dn] + CountPred(P[m])`. The vector variant
computes `Z[dn] = Z[dn] + splat(CountPred(P[m]))` with element sizes 16/32/64 (half/word/double).
Uses `SVESpecialOp::Dup` to broadcast the 64-bit count to vector elements, then
`ScalableVectorMap2` with `Add`. Reuses existing SVESpecial and ScalableVectorMap2.
Requires FEAT_SVE/FEAT_SME. 9 focused tests pass (tests/incp_scalar_sve.rs).

2026-10-06 — SQINCP, UQINCP (scalar) implemented using new scalar `BinOp::SQAdd` and `BinOp::UQAdd`
for signed/unsigned saturating addition. Reuses `SVESpecialOp::CountPred` for the predicate
count. The instruction computes saturated addition: `X[dn] = SatQ(X[dn] + CountPred(P[m]))`.
Vector variants use `SVEVectorBinOp::SQAdd`/`UQAdd` with `ScalableVectorMap2` for element-wise
saturating addition. Esize varies: 16 (half), 32 (word), 64 (double). Reuses existing
SVESpecial, Dup, and ScalableVectorMap2. Requires FEAT_SVE/FEAT_SME. 16 focused tests
pass (tests/sqincp_sve.rs).

2026-10-06 — DECB, DECD, DECH, DECW (scalar) implemented as decrement counterparts of INC.
Uses `SVESpecialOp::DecodePredCount` for predicate constraint count, then subtracts
`count * imm` from scalar register. Esize: 8 (B), 64 (D), 16 (H), 32 (W). Reuses
existing BinOp::Sub and SVESpecial. Requires FEAT_SVE/FEAT_SME. 6 focused tests
pass (tests/decb_scalar_sve.rs).

2026-10-06 — DECD, DECH, DECW (vector) implemented similarly to INC vector variants.
Uses `DecodePredCount` and `Dup` to broadcast decrement value, then element-wise
subtraction via `ScalableVectorMap2` with `Sub`. Esize: 64 (D), 16 (H), 32 (W).
Requires FEAT_SVE/FEAT_SME. 8 focused tests pass (tests/decb_scalar_sve.rs).

2026-10-06 — DECP (scalar) and DECP (vector) implemented as decrement counterparts of INCP.
Uses `SVESpecialOp::CountPred` for true predicate element count, then subtracts count.
Scalar uses `BinOp::Sub`; vector uses `ScalableVectorMap2` with `Sub` and `Dup`.
Esize: 16 (H), 32 (W), 64 (D). Requires FEAT_SVE/FEAT_SME. 8 focused tests
pass (tests/decp_scalar_sve.rs).

2026-10-06 — SQDECB/UQDECB/SQDECD/UQDECD/SQDECH/UQDECH/SQDECW/UQDECW (scalar and vector)
implemented as saturating decrement counterparts of SQINCB/UQINCB etc.
Uses new scalar `BinOp::SQSub`/`UQSub` and `SVEVectorBinOp::SQSub`/`UQSub` for
signed/unsigned saturating subtraction. Reuses `DecodePredCount` for constraint count
or `CountPred` for true element count. Esize varies per variant (B/H/W/D).
Requires FEAT_SVE/FEAT_SME. 22 focused tests pass (tests/sqdecb_sve.rs).

2026-10-06 — SQDECP, UQDECP (scalar) implemented using scalar `BinOp::SQSub`/`UQSub`
for signed/unsigned saturating subtraction. Reuses `SVESpecialOp::CountPred` for the
predicate true element count. The instruction computes saturated subtraction:
`X[dn] = SatQ(X[dn] - CountPred(P[m]))`. Vector variants use
`SVEVectorBinOp::SQSub`/`UQSub` with `ScalableVectorMap2` for element-wise
saturating subtraction. Esize varies: 16 (half), 32 (word), 64 (double). Reuses
existing SVESpecial, Dup, and ScalableVectorMap2. Requires FEAT_SVE/FEAT_SME. 16
focused tests pass (tests/sqdecp_sve.rs).

2026-10-06 — ADDHNB, ADDHNT (SVE) implemented using `VectorNarrowing` with `NarrowingOp::AddHN`.
These are unpredicated narrowing operations that add pairs of 64-bit elements and
store the high half (narrowed to 32-bit) in destination elements. ADDHNB places
results in even elements (part=0) and zeros odd elements; ADDHNT places results
in odd elements (part=1) preserving even elements. Uses existing VectorNarrowing
IR node. Requires FEAT_SVE2/FEAT_SME. 6 focused tests pass (tests/addhnb_sve.rs).

2026-10-06 — ADDP (SVE predicated) implemented using `ScalableVectorPairwise` with
`SVEVectorPairwiseOp::AddP`. Adds adjacent element pairs: for even indices adds
operand1[e]+operand1[e+1], for odd indices adds operand2[e-1]+operand2[e].
Results interleaved into destination with merging predication. Requires FEAT_SVE2.
2026-10-06 — ADDPL (scalar) adds predicate register size in bytes (PL/8) multiplied
by signed immediate to general-purpose register or SP. Uses `BinOp::Add` and
`BinOp::Mul` with PL/8 constant. Requires FEAT_SVE/FEAT_SME. 4 focused tests
pass (tests/addp_sve.rs).

2026-10-06 — ADDQV (SVE) implemented using `ScalableVectorReduce` with
`SVEVectorReduceOp::AddQV`. Performs segment-wise reduction: divides VL-bit vector
into 128-bit segments, sums elements at same position across all segments (with
inactive elements treated as zero), and produces 128-bit V register result with
one element per segment position. Uses existing ScalableVectorReduce IR node.
Requires FEAT_SVE2p1/FEAT_SME2p1. 3 focused tests pass (tests/addqv_sve.rs).

2026-10-06 — AESD (indexed, SVE) and AESD (vectors, SVE) implemented using
`SVESpecialOp::AESInvSubBytes` for multi-vector AES single round decryption.
Indexed variant operates on indexed elements; vector variant is unpredicated.
Uses existing SVESpecial IR node. Requires FEAT_SVE2. 2 focused tests pass
(tests/aesd_sve_indexed.rs, tests/aesd_sve_unpredicated.rs).

2026-10-06 — AESEMC (SVE) implemented using `SVESpecialOp::AESMixColumns` for
multi-vector AES single round encryption and mix columns. Uses existing SVESpecial
IR node. Requires FEAT_SVE2. 2 focused tests pass (tests/aesemc_sve.rs).

2026-10-06 — AESIMC (SVE) implemented using `SVESpecialOp::AESInvMixColumns` for
AES inverse mix columns. Uses existing SVESpecial IR node. Requires FEAT_SVE2.
1 focused test passes (tests/aesimc_sve.rs).

2026-10-06 — AESMC (SVE) implemented using `SVESpecialOp::AESMixColumns` for
AES mix columns. Uses existing SVESpecial IR node. Requires FEAT_SVE2.
1 focused test passes (tests/aesmc_sve.rs).

2026-10-06 — CNOT (SVE) implemented using `ScalableVectorMap` with
`SVEVectorUnaryOp::Not` to logically invert boolean values (zero -> 1, non-zero -> 0)
in each active vector element. Supports both merging and zeroing predication modes.
Uses existing ScalableVectorMap IR node. Requires FEAT_SVE2p2/FEAT_SME2p2.
4 focused tests pass (tests/cnot_sve.rs).

2026-10-06 — COMPACT (SVE) implemented using `SVESpecialOp::Compact` to copy
active predicate elements to consecutive positions in the destination vector,
zeroing the remaining elements. Uses existing SVESpecial IR node with predicate
and source vector operands. Requires FEAT_SVE/FEAT_SME2p2. 2 focused tests
pass (tests/compact_sve.rs).

2026-10-06 — CTERMEQ, CTERMNE (SVE) implemented using scalar comparison
(`BinOp::Eq`/`BinOp::Ne`) and `Select` operations to model the termination
condition logic. Sets N flag to 1 and V flag to 0 if comparison succeeds;
otherwise sets N=0 and V=NOT(C). Uses existing BinOp, Select, and flag
operations. Requires FEAT_SVE/FEAT_SME. 4 focused tests pass
(tests/ctermeq_sve.rs).

2026-10-06 — CNTB, CNTD, CNTH, CNTW (scalar) implemented using `SVESpecialOp::DecodePredCount`
to get the predicate constraint element count, multiplied by immediate.
Uses existing `DecodePredCount` and `BinOp::Mul`. Esize varies: 8 (B), 64 (D), 16 (H), 32 (W).
Reuses existing SVESpecial and BinOp. Requires FEAT_SVE/FEAT_SME. 6 focused tests
pass (tests/cntb_scalar_sve.rs).

2026-10-06 — CNTP (predicate) implemented using `SVESpecialOp::CountPred` with
both predicate mask and operand to count true elements matching the mask.
Uses existing SVESpecial IR node. Esize varies: 16 (H), 32 (W), 64 (D).
Requires FEAT_SVE/FEAT_SME. 8 focused tests pass (tests/cntb_scalar_sve.rs).

2026-10-06 — CNTP (predicate as counter) implemented using new `SVESpecialOp::CounterToPredicate`
to expand a 16-bit predicate counter to PL*4 bits via `CounterToPredicate(pred[15:0])`,
then `CountPred` to count active elements up to maxelements = elements * width.
Uses new SVESpecialOp for CounterToPredicate and existing CountPred. Requires FEAT_SVE2p1/FEAT_SME2.
3 focused tests pass (tests/cntp_counter_sve.rs).

2026-10-06 — CPY (immediate, merging/zeroing) implemented by broadcasting the immediate
to a full vector using `SVESpecialOp::Dup`, then using `ScalableVectorMap` with new
`SVEVectorUnaryOp::Id` (identity) to copy the broadcast value to active elements.
Merging and zeroing predication modes handled by `ScalableVectorMap` predication.
Uses existing SVESpecial and new Id unary op. Requires FEAT_SVE. 4 focused tests
pass (tests/cpy_sve.rs).

2026-10-06 — CPY (scalar) and CPY (SIMD&FP scalar) implemented similarly by broadcasting
the source register value to a vector via `SVESpecialOp::Dup`, then using
`ScalableVectorMap` with `Id` to copy with merging predication. Reuses existing
SVESpecial and new Id unary op. Requires FEAT_SVE. 5 focused tests pass
(tests/cpy_sve.rs).

2026-10-06 — DUP (immediate) implemented using `SVESpecialOp::Dup` to replicate
an immediate value across all vector elements. Uses existing SVESpecial with
immediate operand. Requires FEAT_SVE/FEAT_SME. 2 focused tests
pass (tests/dup_sve.rs).

2026-10-06 — DUP (indexed) implemented using `SVESpecialOp::Dup` to extract an
indexed element from a source vector and replicate it across all elements.
Uses existing SVESpecial with source vector operand and immediate index.
Requires FEAT_SVE/FEAT_SME. 2 focused tests pass (tests/dup_sve.rs).

2026-10-06 — DUPM implemented using `SVESpecialOp::Dup` to broadcast a 64-bit
logical bitmask immediate across all vector elements. Uses existing SVESpecial
with 64-bit immediate operand. Requires FEAT_SVE/FEAT_SME. 2 focused tests
pass (tests/dup_sve.rs).

2026-10-06 — DUPQ implemented using `SVESpecialOp::Dup` to broadcast an indexed
element within each 128-bit vector segment. Uses existing SVESpecial with
source vector and immediate index. Requires FEAT_SVE2p1/FEAT_SME2p1. 2 focused
tests pass (tests/dup_sve.rs).

2026-10-06 — DUP (scalar) implemented using `SVESpecialOp::Dup` to broadcast
a general-purpose register value across all vector elements. Uses existing
SVESpecial with source register operand. Requires FEAT_SVE/FEAT_SME. 2 focused
tests pass (tests/dup_sve.rs).

2026-10-06 — F1CVT, F2CVT (SVE predicated) implemented using `SVEVectorUnaryOp::Bfloat16Cvt`
with `ScalableVectorMap` to convert FP8 elements to BF16. F1CVT uses FPMR.LSCALE/F8S1,
F2CVT uses FPMR.LSCALE2/F8S2. The issrc2 parameter is handled at lowering time.
Uses existing `Bfloat16Cvt` unary op and `ScalableVectorMap` with merging predication.
Requires FEAT_SVE2/FEAT_SME2/FEAT_FP8. 4 focused tests pass (tests/f1cvt_sve_predicated.rs,
tests/bf1cvt_sve_predicated.rs).

2026-10-06 — F1CVTLT, F2CVTLT (SVE predicated) implemented using `SVEVectorUnaryOp::Bfloat16Cvtlt`
with `ScalableVectorMap` to convert odd-numbered 8-bit FP8 elements to BF16. F1CVTLT uses
FPMR.LSCALE/F8S1, F2CVTLT uses FPMR.LSCALE2/F8S2. The issrc2 parameter handled at lowering.
Uses existing `Bfloat16Cvtlt` unary op and `ScalableVectorMap` with merging predication.
Requires FEAT_SVE2/FEAT_SME2/FEAT_FP8. 4 focused tests pass (tests/f1cvlt_sve_predicated.rs,
tests/bf1cvlt_sve_predicated.rs).

2026-10-06 — FABD (SVE predicated) implemented using `SVEVectorBinOp::Fabd` with
`ScalableVectorMap2` for destructive absolute difference: result = |Zdn - Zm| for active
elements, Zdn unchanged for inactive. Uses merging predication. Requires FEAT_SVE.
2 focused tests pass (tests/fabd_sve_predicated.rs).

2026-10-06 — FABS (SVE predicated) implemented using `SVEVectorUnaryOp::Fabs` with
`ScalableVectorMap` for floating-point absolute value. Both merging (inactive keep dest)
and zeroing (inactive set to zero) predication variants implemented.
Requires FEAT_SVE2p2/FEAT_SME2p2. 4 focused tests pass (tests/fabs_sve_predicated.rs).

2026-10-06 — FAC<cc> (SVE) implemented by adding `Facge`, `Facgt`, `Faclt`, `Faceq`, `Facle`
variants to `SVEPredicateOp`. Uses `PredicateMap2` with vector operands to compare
absolute values of FP elements and produce predicate results. FACGE: |Z[n]| >= |Z[m]|
FACGT: |Z[n]| > |Z[m]|. Pseudo-instructions FACLT/FACLE swap operands. Requires FEAT_SVE.
4 focused tests pass (tests/fac_sve.rs).

2026-10-06 — FADD (SVE) implemented using `SVEVectorBinOp::Fadd` with `ScalableVectorMap2`
for all three variants: (1) predicated destructive vector-vector add (Zdn = Zdn + Zm for active),
(2) unpredicated vector-vector add (Zd = Zn + Zm), and (3) predicated immediate add using
`SVESpecialOp::Dup` to broadcast FPOne/FPPointFive then `ScalableVectorMap2` with merging
predication. Requires FEAT_SVE. 6 focused tests pass (tests/fadd_sve.rs).

2026-10-06 — FADDA (SVE) implemented by adding `FAddA` to `SVEVectorReduceOp` and extending
`ScalableVectorReduce` with optional `initial_value` field. Uses scalar V register as initial
accumulator, adds all active vector elements in strict order low-to-high. Result to V register.
Requires FEAT_SVE. 2 focused tests pass (tests/fadda_sve.rs).

2026-10-06 — FADDP (SVE predicated) implemented using `SVEVectorPairwiseOp::FAddP` with
`ScalableVectorPairwise`. Adds adjacent element pairs: even e uses Zdn[e]+Zdn[e+1], odd e uses
Zm[e-1]+Zm[e]. Results interleaved into Zdn (destructive). Requires FEAT_SVE2/FEAT_SME.
2 focused tests pass (tests/faddp_sve.rs).

2026-10-06 — FADDQV (SVE) implemented by adding `FAddQVSeg` to `SVESpecialOp`. Performs
segment-wise reduction across 128-bit quadword segments, producing 128-bit result in V register.
Inactive elements treated as +0.0. Extended `SVESpecial` validation to handle predicate and
return type (128-bit). Requires FEAT_SVE2p1/FEAT_SME2p1. 2 focused tests pass (tests/faddqv_sve.rs).

2026-10-06 — FADDV (SVE) uses existing `SVEVectorReduceOp::FAddV` with `ScalableVectorReduce`.
Recursive pairwise reduction to scalar in V register. Inactive elements treated as +0.0.
Updated existing test to include new `initial_value` field. Requires FEAT_SVE. 1 test passes.

2026-10-06 — FAMAX, FAMIN (SVE predicated) implemented by adding `FAMax`, `FAMin` to
`SVEVectorBinOp`. Uses `ScalableVectorMap2` with merging predication for absolute maximum/minimum
of complex numbers: result = max(|Zdn|, |Zm|) or min(|Zdn|, |Zm|) for active elements, Zdn
unchanged for inactive. Requires FEAT_SVE2/FEAT_SME2/FEAT_FAMINMAX. 4 focused tests pass
(tests/famax_famin_sve.rs).

2026-10-06 — FCADD (SVE predicated) implemented using existing `SVEVectorTernaryOp::Fcadd`
with `ScalableVectorMap3`. Complex add with 90° rotation: cross-adds real/imaginary parts
(Zdn.real + Zm.imag, Zdn.imag + Zm.real) for active elements. Destructive on Zdn.
Requires FEAT_SVE/FEAT_SME. 2 focused tests pass (tests/fcadd_sve.rs).

2026-10-06 — FCLAMP (SVE unpredicated) implemented by adding `FClamp` to `SVEVectorTernaryOp`.
Uses `ScalableVectorMap3` with zeroing predication (unpredicated) to clamp Z[d] between
Z[n] (min) and Z[m] (max): result = min(max(Z[d], Z[n]), Z[m]). Destructive on Z[d].
Requires FEAT_SVE2p1/FEAT_SME2. 2 focused tests pass (tests/fclamp_sve.rs).

2026-10-06 — FCM<cc> (SVE) implemented using existing `SVEPredicateOp` variants
(Eq, Ge, Gt, Ne, Lt, Le, Unordered) with `PredicateMap2`. Vector-to-predicate FP comparisons:
FCMEQ (==), FCMGE (>=), FCMGT (>), FCMLE (<=, swaps operands), FCMLT (<, swaps operands),
FCMNE (!=), FCMUO (unordered). Inactive elements set to zero. Requires FEAT_SVE/FEAT_SME.
12 focused tests pass (tests/fcm_sve.rs).

2026-10-06 — FCPY (SVE predicated) implemented using `SVESpecialOp::Dup` to broadcast
FP immediate, then `ScalableVectorMap` with `Id` for predicated copy. Both merging (inactive
keep dest) and zeroing (inactive set to zero) variants. Similar to CPY immediate but for FP.
Requires FEAT_SVE/FEAT_SME. 4 focused tests pass (tests/fcpy_sve.rs).

2026-10-06 — FCVT (SVE predicated) implemented using existing `SVEVectorUnaryOp::Fcvt`
with `ScalableVectorMap` for FP precision conversion (FP32<->FP64). Source elements s_esize
bits, dest elements d_esize bits. Smaller type unpacked in LSBs of larger. Both merging and
zeroing predication. Requires FEAT_SVE2p2/FEAT_SME2p2. 4 focused tests pass
(tests/fcvt_sve_predicated.rs).

2026-10-06 — FCVTLT (SVE predicated) implemented using existing `SVEVectorUnaryOp::Fcvtlt`
with `ScalableVectorMap` for widening convert of odd-numbered elements (top half). Converts
half-precision elements to full precision (e.g., FP32->FP64). Both merging and zeroing
predication. Requires FEAT_SVE2p2/FEAT_SME2p2. 4 focused tests pass
(tests/fcvtlt_sve_predicated.rs).

2026-10-06 — FCVTZS, FCVTZU (SVE predicated) implemented using existing
`SVEVectorUnaryOp::Fcvtzs`/`Fcvtzu` with `ScalableVectorMap` for FP-to-integer conversion
with rounding toward zero (truncation). FCVTZS: signed (sign-extend), FCVTZU: unsigned
(zero-extend). Both merging and zeroing predication. Requires FEAT_SVE2p2/FEAT_SME2p2.
8 focused tests pass (tests/fcvtz_sve_predicated.rs).

2026-10-06 — FDIV, FDIVR (SVE predicated) implemented using existing
`SVEVectorBinOp::FDiv`/`FDivR` with `ScalableVectorMap2`. FDIV: Zdn = Zdn / Zm (destructive),
FDIVR: Zdn = Zm / Zdn (reversed, destructive). Both with merging predication.
Requires FEAT_SVE/FEAT_SME. 4 focused tests pass (tests/fdiv_sve.rs).

2026-10-06 — FDOT (SVE) implemented using existing `VectorDotOp::FP8Dot` with
`VectorDot` expression. Two variants: (1) 2-way FP8 to FP16 - each 16-bit element holds
pair of FP8, dot product accumulated in FP16; (2) 4-way FP8 to FP32 - each 32-bit element
holds four FP8, dot product accumulated in FP32. Both unpredicated, use FPMR for scaling
and format selection. Extended validation to accept esize=W16 for FP8Dot 2-way.
Requires FEAT_FP8DOT2/FEAT_FP8DOT4/FEAT_SVE. 4 focused tests pass (tests/fdot_sve.rs).

2026-10-06 — FMUL (SVE) implemented using existing `SVEVectorBinOp::Fmul` with
`ScalableVectorMap2`. Four variants: (1) predicated destructive vector-vector multiply
(Zdn = Zdn * Zm), (2) unpredicated vector-vector multiply (Zd = Zn * Zm), (3) predicated
immediate multiply using `SVESpecialOp::Dup` to broadcast FPTwo/FPPointFive, (4) indexed
multiply by element within 128-bit segment. All require FEAT_SVE. 8 focused tests pass
(tests/fmul_sve.rs).

2026-10-06 — FMULX (SVE predicated) implemented using existing `SVEVectorBinOp::FMulX`
with `ScalableVectorMap2`. Like FMUL but ∞×0.0 gives 2.0 instead of NaN. Used with FRECPX
for safe vector normalization. Requires FEAT_SVE. 2 focused tests pass (tests/fmulx_sve.rs).

2026-10-06 — FMLA, FMLS (SVE predicated) implemented using existing
`SVEVectorTernaryOp::Fmla`/`Fmls` with `ScalableVectorMap3`. FMLA: Zda = Zda + Zn*Zm,
FMLS: Zda = Zda - Zn*Zm (negated first operand). Destructive on Zda (addend/destination).
Both with merging predication. Requires FEAT_SVE. 4 focused tests pass
(tests/fmla_fmls_sve.rs).

2026-10-06 — FDUP (SVE unpredicated) implemented using `SVESpecialOp::Dup` to broadcast
FP immediate to all vector elements. Uses `SVESpecial` expression directly (unpredicated).
Also known as FMOV (immediate, unpredicated). Requires FEAT_SVE/FEAT_SME. 2 focused tests
pass (tests/fdup_sve_unpredicated.rs).

2026-10-06 — FEXPA (SVE unpredicated) implemented using existing
`SVEVectorUnaryOp::Fexpa` with `ScalableVectorMap` (zeroing predication for unpredicated).
Floating-point exponential accelerator - bit-manipulation operation returning approximate
2^x. Requires FEAT_SVE/FEAT_SSVE_FEXPA. 1 test passes (tests/fexpa_sve_unpredicated.rs).

2026-10-06 — FLOGB (SVE predicated) implemented using existing
`SVEVectorUnaryOp::Flogb` with `ScalableVectorMap`. Returns base-2 logarithm as integer
(unbiased exponent). Special cases: infinite→max_int, zero/NaN→min_int. Both merging and
zeroing predication. Requires FEAT_SVE2p2/FEAT_SME2p2. 4 focused tests pass
(tests/flogb_sve_predicated.rs).

2026-10-06 — FMAX, FMIN (SVE predicated) implemented using existing
`SVEVectorBinOp::FMax`/`FMin` with `ScalableVectorMap2`. Standard FP max/min with
destructive update on Zdn. Merging predication. Requires FEAT_SVE/FEAT_SME. 4 focused
tests pass (tests/fmax_fmin_sve.rs).

2026-10-06 — FMAXNM, FMINNM (SVE predicated) implemented using existing
`SVEVectorBinOp::FMaxNum`/`FMinNum` with `ScalableVectorMap2`. "Number" variants that
return the numeric operand when one is NaN and the other is numeric. Destructive on Zdn.
Merging predication. Requires FEAT_SVE/FEAT_SME. 4 focused tests pass
(tests/fmaxnm_fminnm_sve.rs).

2026-10-06 — FMAXP, FMINP (SVE predicated) implemented using existing
`SVEVectorPairwiseOp::FMaxP`/`FMinP` with `ScalableVectorPairwise`. Adjacent element
pairwise max/min: even e uses Zdn[e],Zdn[e+1]; odd e uses Zm[e-1],Zm[e]. Results
interleaved into Zdn (destructive). Requires FEAT_SVE2/FEAT_SME. 4 focused tests pass
(tests/fmaxp_fminp_sve.rs).

2026-10-06 — FMAXNMP, FMINNMP (SVE predicated) implemented using existing
`SVEVectorPairwiseOp::FMaxNMP`/`FMinNMP` with `ScalableVectorPairwise`. "Number" variants
of pairwise max/min that return numeric operand when other is NaN. Destructive on Zdn.
Requires FEAT_SVE2/FEAT_SME. 4 focused tests pass (tests/fmaxnmp_fminnmp_sve.rs).

2026-10-06 — FMAXV, FMINV, FMAXQV, FMINQV, FMAXNMV, FMINNMV, FMAXNMQV, FMINNMQV (SVE)
implemented by adding `FMaxNMV`, `FMinNMV`, `FMaxNMQV`, `FMinNMQV` to `SVEVectorReduceOp`
and using existing `FMaxV`/`FMinV`/`FMaxQV`/`FMinQV`/`FMaxNV`/`FMinNV`. Scalar reductions
use `ScalableVectorReduce`; quadword segment reductions to 128-bit use `SVESpecial` with
`FAddQVSeg` (reused). Inactive treated as -Inf (FMAX), +Inf (FMIN), default NaN (number
variants). Requires FEAT_SVE/FEAT_SVE2p1. 8 focused tests pass (tests/fmaxv_fminv_sve.rs).

2026-10-06 — FMAD (SVE predicated) implemented using existing `SVEVectorTernaryOp::Fmad`
with `ScalableVectorMap3`. FMAD: Zdn = Za + Zdn * Zm (Zdn is multiplicand AND destination,
Za is addend, Zm is multiplier). Different from FMLA where Zda is addend. Merging predication.
Requires FEAT_SVE/FEAT_SME. 2 focused tests pass (tests/fmad_sve.rs).

2026-10-06 — FMAX/FMIN/FMAXNM/FMINNM (immediate, SVE predicated) implemented using existing
`SVEVectorBinOp::FMax`/`FMin`/`FMaxNum`/`FMinNum` with `ScalableVectorMap2`. Immediate is
0.0 or 1.0, broadcast via `SVESpecialOp::Dup` then merged with source. Destructive on Zdn.
Merging predication. Requires FEAT_SVE/FEAT_SME. 8 focused tests pass
(tests/fmaxmin_imm_sve.rs).

2026-10-06 — FRECPE, FRECPX, FRECPS, FRSQRTE, FRSQRTS (SVE) implemented using existing
`SVEVectorUnaryOp::Frecpe`/`FRecpX`/`Frsqrte` and `SVEVectorBinOp::Frecps`/`Frsqrts`.
FRECPE/FRSQRTE: unpredicated reciprocal/sqrt estimates via `ScalableVectorMap` (zeroing).
FRECPX: predicated reciprocal exponent via `ScalableVectorMap` (merging).
FRECPS/FRSQRTS: unpredicated Newton-Raphson refinement steps via `ScalableVectorMap2` (zeroing).
Requires FEAT_SVE/FEAT_SME. 10 focused tests pass (tests/frecp_frsqrt_sve.rs).

2026-10-06 — FSQRT, FSCALE, FSUB, FNEG (SVE) implemented using existing
`SVEVectorUnaryOp::Fsqrt`/`Fneg` and `SVEVectorBinOp::FScale`/`Fsub`.
FSQRT: square root (predicated, merging). FSCALE: adjust exponent Zdn*2^Zm (predicated, merging).
FSUB: subtract with three variants - predicated vector (destructive), unpredicated vector,
immediate (broadcast via Dup). FNEG: negate (predicated, merging).
Requires FEAT_SVE/FEAT_SME. 12 focused tests pass (tests/fsqrt_fscale_fsub_fneg_sve.rs).

2026-10-06 — FRINT<r> family (SVE predicated) implemented using existing
`SVEVectorUnaryOp` variants (Frintn, Frintp, Frintm, Frintz, Frinta, Frintx, Frinti,
FRint32X, FRint32Z, FRint64X, FRint64Z) with `ScalableVectorMap`. 11 rounding mode
variants (N, A, M, P, Z, I, X, 32X, 32Z, 64X, 64Z), each with merging and zeroing
predication. Requires FEAT_SVE2p2/FEAT_SME2p2. 22 focused tests pass (tests/frint_sve.rs).

2026-10-06 — FSUBR (SVE predicated) implemented by adding `Fsubr` to `SVEVectorBinOp`
with `ScalableVectorMap2`. FSUBR: Zdn = Zm - Zdn (reversed), Zdn = imm - Zdn (immediate).
Both destructive on Zdn with merging predication. Requires FEAT_SVE/FEAT_SME.
4 focused tests pass (tests/fsubr_sve.rs).

2026-10-06 — FTMAD, FTSMUL, FTSSEL (SVE unpredicated) implemented using existing
`SVEVectorTernaryOp::Ftmad` and `SVEVectorBinOp::Ftsmul`/`Ftssel`.
FTMAD: Zdn = Zdn * |Zm| + coefficient(imm, sign(Zm)) - Taylor series coefficient
accumulation for sin/cos. FTSMUL: Zd = Zn^2 with sign from Zm[0]. FTSSEL: selects
x/1.0/-x/-1.0 based on Zm[0:1] for quadrant handling. All unpredicated.
Requires FEAT_SVE. 6 focused tests pass (tests/ftrig_sve.rs).

2026-10-06 — HISTCNT, HISTSEG (SVE) implemented using existing `SVESpecialOp::Histcnt`/`Histseg`.
HISTCNT: predicated count of Zm[i] == Zn[e] for i <= e (active elements only), zero for inactive.
HISTSEG: unpredicated count of matches within 128-bit segments for byte elements.
Both use `SVESpecial` with predicate for HISTCNT. Requires FEAT_SVE2. 4 focused tests pass
(tests/histcnt_histseg_sve.rs).

2026-10-06 — INDEX family (SVE unpredicated) implemented using existing `SVESpecialOp::Index`.
Four variants: (1) immediates [imm1 + e*imm2], (2) immediate+scalar [imm + e*X[m]],
(3) scalar+immediate [X[n] + e*imm], (4) scalars [X[n] + e*X[m]]. All use `SVESpecial`
with Index operation. Requires FEAT_SVE/FEAT_SME. 8 focused tests pass
(tests/index_sve.rs).

2026-10-07 — LASTA, LASTB, LASTP (SVE) implemented using existing `SVESpecialOp::LastA`/`LastB`/`LastP`
with new variants `LastA_Scalar`/`LastB_Scalar` for X register destinations (returning 64-bit
zero-extended scalars). LASTA extracts element after last active, LASTB extracts last active,
LASTP finds index of last true predicate element. Scalar variants write to X register (64-bit
zero-extended), SIMD&FP variants write to V register (esize bits). LASTP writes index to X
register. All predicated. Requires FEAT_SVE/FEAT_SME (LASTA/LASTB) and FEAT_SVE2p2 (LASTP).
10 focused tests pass (tests/last_sve.rs).

2026-10-07 — LD1B (scalar plus scalar, single register) implemented using existing
`SVEGatherScatterOp::ContiguousLoad` with `SVEGatherScatter`. Predicated contiguous load
of unsigned bytes with scalar base (X[n]/SP) and scalar index (X[m]). Index increments after
each element but register not updated. Extended validation to allow 64-bit register index
for contiguous loads. Requires FEAT_SVE. 2 focused tests pass (tests/ld1b_sve_contiguous.rs).

2026-10-07 — LD1B (scalar plus immediate, single register) implemented using existing
`SVEGatherScatterOp::ContiguousLoad` with `SVEGatherScatter`. Predicated contiguous load
of unsigned bytes with scalar base (X[n]/SP) and immediate index (imm4, range -8 to 7).
Extended validation to accept 4-bit immediate index for contiguous loads. Requires FEAT_SVE.
2 focused tests pass (tests/ld1b_sve_contiguous_imm.rs).

2026-10-07 — LD1H, LD1W, LD1D (scalar plus immediate, single register) implemented using existing
`SVEGatherScatterOp::ContiguousLoad` with `SVEGatherScatter`. Predicated contiguous loads
of unsigned halfwords (16-bit), words (32-bit), and doublewords (64-bit) with scalar base
(X[n]/SP) and immediate index (imm4, range -8 to 7). Same pattern as LD1B but with
different msize (16/32/64). Extended validation already accepts 4-bit immediate index.
Requires FEAT_SVE. 2 focused tests each pass (tests/ld1h_sve_contiguous_imm.rs,
tests/ld1w_sve_contiguous_imm.rs, tests/ld1d_sve_contiguous_imm.rs).

2026-10-07 — LD1H, LD1W, LD1D (vector plus immediate) implemented using existing
`SVEGatherScatterOp::GatherLoad` with `SVEGatherScatter`. Predicated gather loads
of unsigned halfwords (16-bit), words (32-bit), and doublewords (64-bit) with vector base
(Zn) and immediate index (imm5, multiple of mbytes, range 0-62/124/248). Extended validation
to accept 5-bit immediate index for gather loads with vector base. Requires FEAT_SVE.
2 focused tests each pass (tests/ld1h_sve_gather_imm.rs, tests/ld1w_sve_gather_imm.rs,
tests/ld1d_sve_gather_imm.rs).

2026-10-07 — LD1RB, LD1RH, LD1RW, LD1RD (SVE predicated) implemented using existing
`SVEGatherScatterOp::ReplicateLoad` with `SVEGatherScatter`. Load and broadcast single
element to all active vector elements. Scalar base (X[n]/SP) with immediate index (imm6,
range 0-63). Predicated with zeroing. Extended validation to accept 6-bit immediate index.
Requires FEAT_SVE/FEAT_SME. 8 focused tests pass (tests/ld1r_sve.rs).

2026-10-07 — LD1RSB, LD1RSH, LD1RSW (SVE predicated) implemented using existing
`SVEGatherScatterOp::ReplicateLoad` with `SVEGatherScatter`. Load and broadcast signed
element (byte/halfword/word) to all active vector elements with sign extension.
Scalar base (X[n]/SP) with immediate index (imm6, range 0-63). Predicated with zeroing.
Requires FEAT_SVE/FEAT_SME. 6 focused tests pass (tests/ld1rsb_rsh_rsw_sve.rs).

2026-10-07 — LD1SB, LD1SH, LD1SW (SVE) implemented using existing
`SVEGatherScatterOp::GatherLoad`/`ContiguousLoad` with `SVEGatherScatter`.
Load signed elements (byte/halfword/word) with sign extension.
Variants: (1) gather with vector index (Zn + Zm), (2) contiguous with scalar index (X[m]),
(3) contiguous with immediate index (imm4, SInt), (4) gather with immediate index (imm5).
All use sign extension (data_unsigned: false). Predicated with zeroing.
Requires FEAT_SVE/FEAT_SME. 10 focused tests pass (tests/ld1sb_sh_sw_sve.rs).

2026-10-07 — LD2B (SVE) implemented using `SVEGatherScatterOp::ContiguousLoad` with
`SVEGatherScatter` for scalar index variant, and `SVESpecialOp::Index` for immediate variant.
Contiguous load of two-byte structures to two consecutive vector registers (nreg=2).
Two variants: (1) contiguous with immediate index (imm4, SInt, range -16 to 14),
(2) contiguous with scalar index (X[m]). Predicated with zeroing.
Requires FEAT_SVE/FEAT_SME. 4 focused tests pass (tests/ld2b_sve.rs).

2026-10-07 — LD2H, LD2W, LD2D, LD2Q (SVE) implemented using existing
`SVEGatherScatterOp::ContiguousLoad` with `SVEGatherScatter` for scalar index variant,
and `SVESpecialOp::Index` for immediate variant.
Contiguous load of two-element structures (halfwords/words/doublewords/quadwords) to
two consecutive vector registers (nreg=2).
Four variants: immediate index (imm4, SInt, range -16 to 14) and scalar index (X[m])
for each element size (halfword/word/doubleword/quadword). Predicated with zeroing.
Extended validation to accept esize/msize=W128 for LD2Q.
Requires FEAT_SVE/FEAT_SME. 16 focused tests pass (tests/ld2_hwdq_sve.rs).

2026-10-07 — LD3B, LD3H, LD3W, LD3D, LD3Q, LD4B, LD4H, LD4W, LD4D, LD4Q (SVE) implemented using existing
`SVEGatherScatterOp::ContiguousLoad` with `SVEGatherScatter` for scalar index variant,
and `SVESpecialOp::Index` for immediate variant.
Contiguous load of three-element (LD3) or four-element (LD4) structures to three/four
consecutive vector registers (nreg=3/4).
For each of 5 element sizes (byte/halfword/word/doubleword/quadword), two variants:
immediate index (imm4, SInt, range -24 to 21 for LD3, -32 to 28 for LD4)
and scalar index (X[m]). Predicated with zeroing.
Extended validation to accept esize/msize=W128 for LD3Q/LD4Q.
Requires FEAT_SVE/FEAT_SME. 40 focused tests pass (tests/ld3_ld4_sve.rs).

2026-10-07 — LDFF1B, LDFF1H, LDFF1W, LDFF1D, LDFF1SB, LDFF1SH, LDFF1SW (SVE) implemented using existing
`SVEGatherScatterOp::FirstFaultLoad` with `SVEGatherScatter`. Gather loads with
first-faulting behavior: only first active element that faults actually faults,
subsequent elements set to zero. Vector base (Zn) + vector index (Zm). Predicated with
zeroing. Requires FEAT_SVE. 14 focused tests pass (tests/ldff1_gather_sve.rs).

2026-10-07 — LDNF1B, LDNF1H, LDNF1W, LDNF1D, LDNF1SB, LDNF1SH, LDNF1SW (SVE) implemented using existing
`SVEGatherScatterOp::NonFaultLoad` with `SVEGatherScatter`. Contiguous loads with
non-faulting behavior: MemNF[] returns (data, fault) instead of faulting. Inactive
elements set to zero. Immediate index (imm4, range -8 to 7). Predicated with zeroing.
Requires FEAT_SVE. 14 focused tests pass (tests/ldnf1_sve.rs).
