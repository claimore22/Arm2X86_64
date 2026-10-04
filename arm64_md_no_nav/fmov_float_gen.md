## FMOV (general)

Floating-point move to or from general-purpose register without conversion

This instruction transfers the contents of a SIMD&FP register to a general-purpose register, or the contents of a general-purpose register to a SIMD&FP register.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

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
<td class="lr">sf</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="2" class="lr">ftype</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">x</td>
<td class="l">1</td>
<td>1</td>
<td class="r">x</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="5"></td>
<td colspan="2"></td>
<td></td>
<td colspan="2" class="droppedname">rmode</td>
<td colspan="3" class="droppedname">opcode</td>
<td colspan="6"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Half-precision to 32-bit variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FMOV_32H_float2int"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 11 && rmode == 00 && opcode == 110)</span>

FMOV [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Hn\>](#Hn__2 "Is the 16-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the Half-precision to 64-bit variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FMOV_64H_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 11 && rmode == 00 && opcode == 110)</span>

FMOV [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Hn\>](#Hn__2 "Is the 16-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 32-bit to half-precision variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FMOV_H32_float2int"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 11 && rmode == 00 && opcode == 111)</span>

FMOV [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 32-bit to single-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMOV_S32_float2int"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 00 && rmode == 00 && opcode == 111)</span>

FMOV [\<Sd\>](#Sd "Is the 32-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the Single-precision to 32-bit variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMOV_32S_float2int"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 00 && rmode == 00 && opcode == 110)</span>

FMOV [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Sn\>](#Sn "Is the 32-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit to half-precision variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FMOV_H64_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 11 && rmode == 00 && opcode == 111)</span>

FMOV [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit to double-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMOV_D64_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 01 && rmode == 00 && opcode == 111)</span>

FMOV [\<Dd\>](#Dd "Is the 64-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit to top half of 128-bit variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMOV_V64I_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 10 && rmode == 01 && opcode == 111)</span>

FMOV [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").D\[1\], [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the Double-precision to 64-bit variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMOV_64D_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 01 && rmode == 00 && opcode == 110)</span>

FMOV [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Dn\>](#Dn "Is the 64-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the Top half of 128-bit to 64-bit variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMOV_64VX_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 10 && rmode == 01 && opcode == 110)</span>

FMOV [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").D\[1\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' && opcode\[2:1\]::rmode != '11 01' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let intsize : integer{} = 32 \<\< UInt(sf); let fltsize : integer{} = if ftype == '10' then 64 else (8 \<\< UInt(ftype XOR '10')); let part : integer = UInt(rmode\[0\]); var op : [FPConvOp](shared_pseudocode.html#type_FPConvOp); case opcode\[2:1\]::rmode of when '11 00' =\> // FMOV if fltsize != 16 && fltsize != intsize then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; op = if opcode\[0\] == '1' then [FPConvOp_MOV_ItoF](shared_pseudocode.html#enum_FPConvOp_MOV_ItoF) else [FPConvOp_MOV_FtoI](shared_pseudocode.html#enum_FPConvOp_MOV_FtoI); when '11 01' =\> // FMOV D\[1\] if intsize != 64 \|\| ftype != '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; op = if opcode\[0\] == '1' then [FPConvOp_MOV_ItoF](shared_pseudocode.html#enum_FPConvOp_MOV_ItoF) else [FPConvOp_MOV_FtoI](shared_pseudocode.html#enum_FPConvOp_MOV_FtoI); otherwise =\> unreachable; end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR"></span>
<p>Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Hn&gt;</td>
<td><span id="Hn__2"></span>
<p>Is the 16-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Hd&gt;</td>
<td><span id="Hd"></span>
<p>Is the 16-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR"></span>
<p>Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sd&gt;</td>
<td><span id="Sd"></span>
<p>Is the 32-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sn&gt;</td>
<td><span id="Sn"></span>
<p>Is the 32-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR__11"></span>
<p>Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dd&gt;</td>
<td><span id="Dd"></span>
<p>Is the 64-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vd&gt;</td>
<td><span id="Vd"></span>
<p>Is the name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dn&gt;</td>
<td><span id="Dn"></span>
<p>Is the 64-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vn&gt;</td>
<td><span id="Vn"></span>
<p>Is the name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); var fltval : bits(fltsize); var intval : bits(intsize); case op of when [FPConvOp_MOV_FtoI](shared_pseudocode.html#enum_FPConvOp_MOV_FtoI) =\> fltval = [Vpart](shared_pseudocode.html#accessor_Vpart_3){fltsize}(n, part); [X](shared_pseudocode.html#accessor_X_2){intsize}(d) = ZeroExtend{intsize}(fltval); when [FPConvOp_MOV_ItoF](shared_pseudocode.html#enum_FPConvOp_MOV_ItoF) =\> intval = [X](shared_pseudocode.html#accessor_X_2){intsize}(n); [Vpart](shared_pseudocode.html#accessor_Vpart_3){fltsize}(d, part) = intval\[fltsize-1:0\]; otherwise =\> unreachable; end;

</div>

### Operational information

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the general-purpose register written by this instruction might be significantly delayed.
