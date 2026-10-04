## LD1 (multiple structures)

Load multiple single-element structures to one, two, three, or four registers

This instruction loads multiple single-element structures from memory and writes the result to one, two, three, or four SIMD&FP registers.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

It has encodings from 2 classes: [No offset](#iclass_no_offset) and [Post-index](#iclass_post_index)

### <span id="iclass_no_offset"></span>No offset<span style="font-size:smaller;">\
(FEAT_AdvSIMD) </span>

<div class="regdiagram-32">

<table class="regdiagram">
<thead>
<tr>
<th>31</th>
<th>30</th>
<th>29</th>
<th>28</th>
<th>27</th>
<th>26</th>
<th>25</th>
<th>24</th>
<th>23</th>
<th>22</th>
<th>21</th>
<th>20</th>
<th>19</th>
<th>18</th>
<th>17</th>
<th>16</th>
<th>15</th>
<th>14</th>
<th>13</th>
<th>12</th>
<th>11</th>
<th>10</th>
<th>9</th>
<th>8</th>
<th>7</th>
<th>6</th>
<th>5</th>
<th>4</th>
<th>3</th>
<th>2</th>
<th>1</th>
<th>0</th>
</tr>
</thead>
<tbody>
<tr class="firstrow">
<td class="lr">0</td>
<td class="lr">Q</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">x</td>
<td>x</td>
<td>1</td>
<td class="r">x</td>
<td colspan="2" class="lr">size</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">L</td>
<td colspan="6"></td>
<td colspan="4" class="droppedname">opcode</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the One register variant

<span id="LD1_asisdlse_R1_1v"></span> Applies when <span class="bitdiff"> (opcode == 0111)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Two registers variant

<span id="LD1_asisdlse_R2_2v"></span> Applies when <span class="bitdiff"> (opcode == 1010)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Three registers variant

<span id="LD1_asisdlse_R3_3v"></span> Applies when <span class="bitdiff"> (opcode == 0110)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the Four registers variant

<span id="LD1_asisdlse_R4_4v"></span> Applies when <span class="bitdiff"> (opcode == 0010)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt4\>](#Vt4 "Is the name of the fourth SIMD&FP register to be transferred, encoded as "Rt" plus 3 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer = UInt(Rt); let n : integer = UInt(Rn); let m : integer = ARBITRARY : integer; let wback : boolean = FALSE; let nontemporal : boolean = FALSE; let tagchecked : boolean = wback \|\| n != 31;

### <span id="iclass_post_index"></span>Post-index<span style="font-size:smaller;">\
(FEAT_AdvSIMD) </span>

<div class="regdiagram-32">

<table class="regdiagram">
<thead>
<tr>
<th>31</th>
<th>30</th>
<th>29</th>
<th>28</th>
<th>27</th>
<th>26</th>
<th>25</th>
<th>24</th>
<th>23</th>
<th>22</th>
<th>21</th>
<th>20</th>
<th>19</th>
<th>18</th>
<th>17</th>
<th>16</th>
<th>15</th>
<th>14</th>
<th>13</th>
<th>12</th>
<th>11</th>
<th>10</th>
<th>9</th>
<th>8</th>
<th>7</th>
<th>6</th>
<th>5</th>
<th>4</th>
<th>3</th>
<th>2</th>
<th>1</th>
<th>0</th>
</tr>
</thead>
<tbody>
<tr class="firstrow">
<td class="lr">0</td>
<td class="lr">Q</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">x</td>
<td>x</td>
<td>1</td>
<td class="r">x</td>
<td colspan="2" class="lr">size</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">L</td>
<td></td>
<td colspan="5"></td>
<td colspan="4" class="droppedname">opcode</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the One register, immediate offset variant

<span id="LD1_asisdlsep_I1_i1"></span> Applies when <span class="bitdiff"> (Rm == 11111 && opcode == 0111)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<imm\>](#imm_option__5 "For the "One register, immediate offset" variant: is the post-index immediate offset, ")

</div>

<div class="encoding">

#### Encoding for the One register, register offset variant

<span id="LD1_asisdlsep_R1_r1"></span> Applies when <span class="bitdiff"> (Rm != 11111 && opcode == 0111)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<Xm\>](#Xm__2 "Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the Two registers, immediate offset variant

<span id="LD1_asisdlsep_I2_i2"></span> Applies when <span class="bitdiff"> (Rm == 11111 && opcode == 1010)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<imm\>](#imm_option__6 "For the "Two registers, immediate offset" variant: is the post-index immediate offset, ")

</div>

<div class="encoding">

#### Encoding for the Two registers, register offset variant

<span id="LD1_asisdlsep_R2_r2"></span> Applies when <span class="bitdiff"> (Rm != 11111 && opcode == 1010)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<Xm\>](#Xm__2 "Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the Three registers, immediate offset variant

<span id="LD1_asisdlsep_I3_i3"></span> Applies when <span class="bitdiff"> (Rm == 11111 && opcode == 0110)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<imm\>](#imm_option__3 "For the "Three registers, immediate offset" variant: is the post-index immediate offset, ")

</div>

<div class="encoding">

#### Encoding for the Three registers, register offset variant

<span id="LD1_asisdlsep_R3_r3"></span> Applies when <span class="bitdiff"> (Rm != 11111 && opcode == 0110)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<Xm\>](#Xm__2 "Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the Four registers, immediate offset variant

<span id="LD1_asisdlsep_I4_i4"></span> Applies when <span class="bitdiff"> (Rm == 11111 && opcode == 0010)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt4\>](#Vt4 "Is the name of the fourth SIMD&FP register to be transferred, encoded as "Rt" plus 3 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<imm\>](#imm_option "For the "Four registers, immediate offset" variant: is the post-index immediate offset, ")

</div>

<div class="encoding">

#### Encoding for the Four registers, register offset variant

<span id="LD1_asisdlsep_R4_r4"></span> Applies when <span class="bitdiff"> (Rm != 11111 && opcode == 0010)</span>

LD1 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, "), [\<Vt4\>](#Vt4 "Is the name of the fourth SIMD&FP register to be transferred, encoded as "Rt" plus 3 modulo 32.").[\<T\>](#T_option__2 "Is an arrangement specifier, ") }, \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<Xm\>](#Xm__2 "Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer = UInt(Rt); let n : integer = UInt(Rn); let m : integer = UInt(Rm); let wback : boolean = TRUE; let nontemporal : boolean = FALSE; let tagchecked : boolean = wback \|\| n != 31;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Vt&gt;</td>
<td><span id="Vt"></span>
<p>Is the name of the first or only SIMD&amp;FP register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T_option__2"></span>
<p>Is an arrangement specifier, encoded in “(size :: Q)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="bitfield">0</td>
<td class="symbol">8B</td>
</tr>
<tr>
<td class="bitfield">00</td>
<td class="bitfield">1</td>
<td class="symbol">16B</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="bitfield">0</td>
<td class="symbol">4H</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="bitfield">1</td>
<td class="symbol">8H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="bitfield">0</td>
<td class="symbol">2S</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="bitfield">1</td>
<td class="symbol">4S</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="bitfield">0</td>
<td class="symbol">1D</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="bitfield">1</td>
<td class="symbol">2D</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option"></span>
<p>Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vt2&gt;</td>
<td><span id="Vt2"></span>
<p>Is the name of the second SIMD&amp;FP register to be transferred, encoded as "Rt" plus 1 modulo 32.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vt3&gt;</td>
<td><span id="Vt3"></span>
<p>Is the name of the third SIMD&amp;FP register to be transferred, encoded as "Rt" plus 2 modulo 32.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vt4&gt;</td>
<td><span id="Vt4"></span>
<p>Is the name of the fourth SIMD&amp;FP register to be transferred, encoded as "Rt" plus 3 modulo 32.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm_option__5"></span>
<p>For the "One register, immediate offset" variant: is the post-index immediate offset, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;imm&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#8</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#16</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="imm_option__6"></span>
<p>For the "Two registers, immediate offset" variant: is the post-index immediate offset, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;imm&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#16</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#32</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="imm_option__3"></span>
<p>For the "Three registers, immediate offset" variant: is the post-index immediate offset, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;imm&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#24</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#48</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="imm_option"></span>
<p>For the "Four registers, immediate offset" variant: is the post-index immediate offset, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;imm&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">#32</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">#64</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="Xm__2"></span>
<p>Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Shared Decode

let datasize : integer{} = 64 \<\< UInt(Q); let esize : integer{} = 8 \<\< UInt(size); let elements : integer = datasize DIV esize; var rpt : integer; // number of iterations let selem : integer = 1; // structure elements case opcode of when '0010' =\> rpt = 4; // LD/ST1 (4 registers) when '0110' =\> rpt = 3; // LD/ST1 (3 registers) when '1010' =\> rpt = 2; // LD/ST1 (2 registers) when '0111' =\> rpt = 1; // LD/ST1 (1 register) otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end;

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var address : bits(64); var eaddr : bits(64); var rval : bits(datasize); var tt : integer; var offs : bits(64) = Zeros{64}; let ebytes : integer{} = esize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_4)([MemOp_LOAD](shared_pseudocode.html#enum_MemOp_LOAD), nontemporal, tagchecked, privileged); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; for r = 0 to rpt-1 do for e = 0 to elements-1 do tt = (t + r) MOD 32; for s = 0 to selem-1 do rval = [V](shared_pseudocode.html#accessor_V_2){datasize}(tt); eaddr = [AddressIncrement](shared_pseudocode.html#func_AddressIncrement_3)(address, offs, accdesc); rval\[e\*:esize\] = [Mem](shared_pseudocode.html#accessor_Mem_3){esize}(eaddr, accdesc); [V](shared_pseudocode.html#accessor_V_2){datasize}(tt) = rval; offs = offs + ebytes; tt = (tt + 1) MOD 32; end; end; end; if wback then if m != 31 then offs = [X](shared_pseudocode.html#accessor_X_2){64}(m); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offs, accdesc); if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
