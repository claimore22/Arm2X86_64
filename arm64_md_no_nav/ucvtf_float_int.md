## UCVTF (scalar, integer)

Unsigned integer convert to floating-point (scalar)

This instruction converts the unsigned integer value in the general-purpose source register to a floating-point value using the rounding mode that is specified by the FPCR, and writes the result to the SIMD&FP destination register.

This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see <span class="armarm-xref" data-linkend="ARMARM_BEIJDDAG">Floating-point exceptions and exception traps</span>.

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
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
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

#### Encoding for the 32-bit to half-precision variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="UCVTF_H32_float2int"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 11)</span>

UCVTF [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 32-bit to single-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="UCVTF_S32_float2int"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 00)</span>

UCVTF [\<Sd\>](#Sd "Is the 32-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 32-bit to double-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="UCVTF_D32_float2int"></span> Applies when <span class="bitdiff"> (sf == 0 && ftype == 01)</span>

UCVTF [\<Dd\>](#Dd "Is the 64-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR "Is the 32-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit to half-precision variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="UCVTF_H64_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 11)</span>

UCVTF [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit to single-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="UCVTF_S64_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 00)</span>

UCVTF [\<Sd\>](#Sd "Is the 32-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the 64-bit to double-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="UCVTF_D64_float2int"></span> Applies when <span class="bitdiff"> (sf == 1 && ftype == 01)</span>

UCVTF [\<Dd\>](#Dd "Is the 64-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Xn\>](#XnOrXZR__11 "Is the 64-bit name of the general-purpose source register, encoded in the "Rn" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let intsize : integer{} = 32 \<\< UInt(sf); let decode_fltsize : integer{} = 8 \<\< UInt(ftype XOR '10'); let unsigned : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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
<td>&lt;Dd&gt;</td>
<td><span id="Dd"></span>
<p>Is the 64-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let fltsize : integer{} = if [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()) then 128 else decode_fltsize; let intval : bits(intsize) = X{}(n); let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPRoundingMode](shared_pseudocode.html#func_FPRoundingMode_1)(FPCR()); let fracbits : integer = 0; var fltval : bits(fltsize) = if [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()) then [V](shared_pseudocode.html#accessor_V_2){fltsize}(d) else Zeros{fltsize}; fltval\[0+:decode_fltsize\] = [FixedToFP](shared_pseudocode.html#func_FixedToFP_7){decode_fltsize, intsize}(intval, fracbits, unsigned, FPCR(), rounding); [V](shared_pseudocode.html#accessor_V_2){fltsize}(d) = fltval;

</div>
