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
