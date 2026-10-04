## ST3 (single structure)

Store single 3-element structure from one lane of three registers

This instruction stores a 3-element structure to memory from corresponding elements of three SIMD&FP registers.

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
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">x</td>
<td>x</td>
<td class="r">1</td>
<td class="lr">S</td>
<td colspan="2" class="lr">size</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">L</td>
<td class="droppedname">R</td>
<td colspan="4"></td>
<td class="droppedname">o2</td>
<td colspan="3" class="droppedname">opcode</td>
<td></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 8-bit variant

<span id="ST3_asisdlso_B3_3b"></span> Applies when <span class="bitdiff"> (opcode == 001)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").B, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").B, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").B }\[[\<index\>](#index "For the "8-bit", "8-bit, immediate offset", and "8-bit, register offset" variants: is the element index, encoded in "Q:S:size".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the 16-bit variant

<span id="ST3_asisdlso_H3_3h"></span> Applies when <span class="bitdiff"> (opcode == 011 && size == x0)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").H, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").H, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").H }\[[\<index\>](#index__2 "For the "16-bit", "16-bit, immediate offset", and "16-bit, register offset" variants: is the element index, encoded in "Q:S:size<1>".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="ST3_asisdlso_S3_3s"></span> Applies when <span class="bitdiff"> (opcode == 101 && size == 00)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").S, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").S, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").S }\[[\<index\>](#index__3 "For the "32-bit", "32-bit, immediate offset", and "32-bit, register offset" variants: is the element index, encoded in "Q:S".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="ST3_asisdlso_D3_3d"></span> Applies when <span class="bitdiff"> (opcode == 101 && S == 0 && size == 01)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").D, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").D, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").D }\[[\<index\>](#index__4 "For the "64-bit", "64-bit, immediate offset", and "64-bit, register offset" variants: is the element index, encoded in "Q".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; var t : integer = UInt(Rt); let n : integer = UInt(Rn); let m : integer = ARBITRARY : integer; let wback : boolean = FALSE; let nontemporal : boolean = FALSE; let tagchecked : boolean = wback \|\| n != 31;

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
<td>1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">x</td>
<td>x</td>
<td class="r">1</td>
<td class="lr">S</td>
<td colspan="2" class="lr">size</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">L</td>
<td class="droppedname">R</td>
<td colspan="5"></td>
<td colspan="3" class="droppedname">opcode</td>
<td></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 8-bit, immediate offset variant

<span id="ST3_asisdlsop_B3_i3b"></span> Applies when <span class="bitdiff"> (Rm == 11111 && opcode == 001)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").B, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").B, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").B }\[[\<index\>](#index "For the "8-bit", "8-bit, immediate offset", and "8-bit, register offset" variants: is the element index, encoded in "Q:S:size".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#3

</div>

<div class="encoding">

#### Encoding for the 8-bit, register offset variant

<span id="ST3_asisdlsop_BX3_r3b"></span> Applies when <span class="bitdiff"> (Rm != 11111 && opcode == 001)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").B, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").B, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").B }\[[\<index\>](#index "For the "8-bit", "8-bit, immediate offset", and "8-bit, register offset" variants: is the element index, encoded in "Q:S:size".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<Xm\>](#Xm__2 "Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the 16-bit, immediate offset variant

<span id="ST3_asisdlsop_H3_i3h"></span> Applies when <span class="bitdiff"> (Rm == 11111 && opcode == 011 && size == x0)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").H, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").H, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").H }\[[\<index\>](#index__2 "For the "16-bit", "16-bit, immediate offset", and "16-bit, register offset" variants: is the element index, encoded in "Q:S:size<1>".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#6

</div>

<div class="encoding">

#### Encoding for the 16-bit, register offset variant

<span id="ST3_asisdlsop_HX3_r3h"></span> Applies when <span class="bitdiff"> (Rm != 11111 && opcode == 011 && size == x0)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").H, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").H, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").H }\[[\<index\>](#index__2 "For the "16-bit", "16-bit, immediate offset", and "16-bit, register offset" variants: is the element index, encoded in "Q:S:size<1>".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<Xm\>](#Xm__2 "Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the 32-bit, immediate offset variant

<span id="ST3_asisdlsop_S3_i3s"></span> Applies when <span class="bitdiff"> (Rm == 11111 && opcode == 101 && size == 00)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").S, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").S, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").S }\[[\<index\>](#index__3 "For the "32-bit", "32-bit, immediate offset", and "32-bit, register offset" variants: is the element index, encoded in "Q:S".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#12

</div>

<div class="encoding">

#### Encoding for the 32-bit, register offset variant

<span id="ST3_asisdlsop_SX3_r3s"></span> Applies when <span class="bitdiff"> (Rm != 11111 && opcode == 101 && size == 00)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").S, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").S, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").S }\[[\<index\>](#index__3 "For the "32-bit", "32-bit, immediate offset", and "32-bit, register offset" variants: is the element index, encoded in "Q:S".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<Xm\>](#Xm__2 "Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit, immediate offset variant

<span id="ST3_asisdlsop_D3_i3d"></span> Applies when <span class="bitdiff"> (Rm == 11111 && opcode == 101 && S == 0 && size == 01)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").D, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").D, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").D }\[[\<index\>](#index__4 "For the "64-bit", "64-bit, immediate offset", and "64-bit, register offset" variants: is the element index, encoded in "Q".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#24

</div>

<div class="encoding">

#### Encoding for the 64-bit, register offset variant

<span id="ST3_asisdlsop_DX3_r3d"></span> Applies when <span class="bitdiff"> (Rm != 11111 && opcode == 101 && S == 0 && size == 01)</span>

ST3 { [\<Vt\>](#Vt "Is the name of the first or only SIMD&FP register to be transferred, encoded in the "Rt" field.").D, [\<Vt2\>](#Vt2 "Is the name of the second SIMD&FP register to be transferred, encoded as "Rt" plus 1 modulo 32.").D, [\<Vt3\>](#Vt3 "Is the name of the third SIMD&FP register to be transferred, encoded as "Rt" plus 2 modulo 32.").D }\[[\<index\>](#index__4 "For the "64-bit", "64-bit, immediate offset", and "64-bit, register offset" variants: is the element index, encoded in "Q".")\], \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], [\<Xm\>](#Xm__2 "Is the 64-bit name of the general-purpose post-index register, excluding XZR, encoded in the "Rm" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; var t : integer = UInt(Rt); let n : integer = UInt(Rn); let m : integer = UInt(Rm); let wback : boolean = TRUE; let nontemporal : boolean = FALSE; let tagchecked : boolean = wback \|\| n != 31;

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
<td>&lt;index&gt;</td>
<td><span id="index"></span>
<p>For the "8-bit", "8-bit, immediate offset", and "8-bit, register offset" variants: is the element index, encoded in "Q:S:size".</p></td>
</tr>
<tr>
<td></td>
<td><span id="index__2"></span>
<p>For the "16-bit", "16-bit, immediate offset", and "16-bit, register offset" variants: is the element index, encoded in "Q:S:size&lt;1&gt;".</p></td>
</tr>
<tr>
<td></td>
<td><span id="index__3"></span>
<p>For the "32-bit", "32-bit, immediate offset", and "32-bit, register offset" variants: is the element index, encoded in "Q:S".</p></td>
</tr>
<tr>
<td></td>
<td><span id="index__4"></span>
<p>For the "64-bit", "64-bit, immediate offset", and "64-bit, register offset" variants: is the element index, encoded in "Q".</p></td>
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

var scale : bits(2) = opcode\[2:1\]; let selem : integer = UInt(opcode\[0\]::R) + 1; var replicate : boolean = FALSE; var index : integer; case scale of when '11' =\> // load and replicate if L == '0' \|\| S == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; scale = size; replicate = TRUE; when '00' =\> index = UInt(Q::S::size); // B\[0-15\] when '01' =\> if size\[0\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; index = UInt(Q::S::size\[1\]); // H\[0-7\] when '10' =\> if size\[1\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if size\[0\] == '0' then index = UInt(Q::S); // S\[0-3\] else if S == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; index = UInt(Q); // D\[0-1\] scale = '11'; end; end; let datasize : integer{} = 64 \<\< UInt(Q); let esize : integer{} = 8 \<\< UInt(scale);

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var address : bits(64); var eaddr : bits(64); var rval : bits(128); var element : bits(esize); var offs : bits(64) = Zeros{64}; let ebytes : integer{} = esize DIV 8; let privileged : boolean = [PSTATE](shared_pseudocode.html#global_PSTATE).EL != [EL0](shared_pseudocode.html#global_EL0); let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescASIMD](shared_pseudocode.html#func_CreateAccDescASIMD_4)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), nontemporal, tagchecked, privileged); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; if replicate then // load and replicate to all elements for s = 0 to selem-1 do eaddr = [AddressIncrement](shared_pseudocode.html#func_AddressIncrement_3)(address, offs, accdesc); element = [Mem](shared_pseudocode.html#accessor_Mem_3){esize}(eaddr, accdesc); // replicate to fill 128- or 64-bit register [V](shared_pseudocode.html#accessor_V_2){datasize}(t) = Replicate{datasize}(element); offs = offs + ebytes; t = (t + 1) MOD 32; end; else // load/store one element per register for s = 0 to selem-1 do rval = [V](shared_pseudocode.html#accessor_V_2){128}(t); eaddr = [AddressIncrement](shared_pseudocode.html#func_AddressIncrement_3)(address, offs, accdesc); // extract from one lane of 128-bit register [Mem](shared_pseudocode.html#accessor_Mem_3){esize}(eaddr, accdesc) = rval\[index\*:esize\]; offs = offs + ebytes; t = ( t + 1 ) MOD 32; end; end; if wback then if m != 31 then offs = [X](shared_pseudocode.html#accessor_X_2){64}(m); end; address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offs, accdesc); if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
