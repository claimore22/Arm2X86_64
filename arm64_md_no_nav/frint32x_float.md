## FRINT32X (scalar)

Floating-point round to 32-bit integer, using current rounding mode (scalar)

This instruction rounds a floating-point value in the SIMD&FP source register to an integral floating-point value that fits into a 32-bit integer size using the rounding mode that is determined by the FPCR, and writes the result to the SIMD&FP destination register.

A zero input returns a zero result with the same sign. When the result value is not numerically equal to the input value, an Inexact exception is raised. When the input is infinite, NaN or out-of-range, the instruction returns the most negative integer representable in the destination size, and an Invalid Operation floating-point exception is raised.

This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see <span class="armarm-xref" data-linkend="ARMARM_BEIJDDAG">Floating-point exceptions and exception traps</span>.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

### <span id="iclass_floating_point"></span>Floating-point<span style="font-size:smaller;">\
(FEAT_FRINTTS) </span>

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">x</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">M</td>
<td></td>
<td class="droppedname">S</td>
<td colspan="5"></td>
<td colspan="2" class="droppedname">ftype</td>
<td colspan="5"></td>
<td colspan="2" class="droppedname">op</td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Single-precision variant

<span id="FRINT32X_S_floatdp1"></span> Applies when <span class="bitdiff"> (ftype == 00)</span>

FRINT32X [\<Sd\>](#Sd "Is the 32-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Sn\>](#Sn "Is the 32-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the Double-precision variant

<span id="FRINT32X_D_floatdp1"></span> Applies when <span class="bitdiff"> (ftype == 01)</span>

FRINT32X [\<Dd\>](#Dd "Is the 64-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Dn\>](#Dn "Is the 64-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FRINTTS) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype IN {'1x'} then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 32 \<\< UInt(ftype\[0\]); let intsize : integer{} = 32;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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
<td>&lt;Dd&gt;</td>
<td><span id="Dd"></span>
<p>Is the 64-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand : bits(esize) = V{}(n); let rounding : [FPRounding](shared_pseudocode.html#type_FPRounding) = [FPRoundingMode](shared_pseudocode.html#func_FPRoundingMode_1)(FPCR()); var result : bits(128) = if [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()) then [V](shared_pseudocode.html#accessor_V_2){128}(d) else Zeros{128}; result\[0+:esize\] = [FPRoundIntN](shared_pseudocode.html#func_FPRoundIntN_5){esize}(operand, FPCR(), rounding, intsize); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>
