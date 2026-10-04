## FMADD

Floating-point fused multiply-add (scalar)

This instruction multiplies the values of the first two SIMD&FP source registers, adds the product to the value of the third SIMD&FP source register, and writes the result to the SIMD&FP destination register.

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td colspan="2" class="lr">ftype</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Ra</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">M</td>
<td></td>
<td class="droppedname">S</td>
<td colspan="5"></td>
<td colspan="2"></td>
<td class="droppedname">o1</td>
<td colspan="5"></td>
<td class="droppedname">o0</td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Half-precision variant <span style="font-size:smaller;">\
(FEAT_FP16) </span>

<span id="FMADD_H_floatdp3"></span> Applies when <span class="bitdiff"> (ftype == 11)</span>

FMADD [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Hn\>](#Hn__5 "Is the 16-bit name of the first SIMD&FP source register holding the multiplicand, encoded in the "Rn" field."), [\<Hm\>](#Hm__2 "Is the 16-bit name of the second SIMD&FP source register holding the multiplier, encoded in the "Rm" field."), [\<Ha\>](#Ha "Is the 16-bit name of the third SIMD&FP source register holding the addend, encoded in the "Ra" field.")

</div>

<div class="encoding">

#### Encoding for the Single-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMADD_S_floatdp3"></span> Applies when <span class="bitdiff"> (ftype == 00)</span>

FMADD [\<Sd\>](#Sd "Is the 32-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Sn\>](#Sn__6 "Is the 32-bit name of the first SIMD&FP source register holding the multiplicand, encoded in the "Rn" field."), [\<Sm\>](#Sm__2 "Is the 32-bit name of the second SIMD&FP source register holding the multiplier, encoded in the "Rm" field."), [\<Sa\>](#Sa "Is the 32-bit name of the third SIMD&FP source register holding the addend, encoded in the "Ra" field.")

</div>

<div class="encoding">

#### Encoding for the Double-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMADD_D_floatdp3"></span> Applies when <span class="bitdiff"> (ftype == 01)</span>

FMADD [\<Dd\>](#Dd "Is the 64-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Dn\>](#Dn__5 "Is the 64-bit name of the first SIMD&FP source register holding the multiplicand, encoded in the "Rn" field."), [\<Dm\>](#Dm__2 "Is the 64-bit name of the second SIMD&FP source register holding the multiplier, encoded in the "Rm" field."), [\<Da\>](#Da "Is the 64-bit name of the third SIMD&FP source register holding the addend, encoded in the "Ra" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let a : integer{} = UInt(Ra); let esize : integer{} = 8 \<\< UInt(ftype XOR '10');

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
<td>&lt;Hn&gt;</td>
<td><span id="Hn__5"></span>
<p>Is the 16-bit name of the first SIMD&amp;FP source register holding the multiplicand, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Hm&gt;</td>
<td><span id="Hm__2"></span>
<p>Is the 16-bit name of the second SIMD&amp;FP source register holding the multiplier, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Ha&gt;</td>
<td><span id="Ha"></span>
<p>Is the 16-bit name of the third SIMD&amp;FP source register holding the addend, encoded in the "Ra" field.</p></td>
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
<td><span id="Sn__6"></span>
<p>Is the 32-bit name of the first SIMD&amp;FP source register holding the multiplicand, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sm&gt;</td>
<td><span id="Sm__2"></span>
<p>Is the 32-bit name of the second SIMD&amp;FP source register holding the multiplier, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Sa&gt;</td>
<td><span id="Sa"></span>
<p>Is the 32-bit name of the third SIMD&amp;FP source register holding the addend, encoded in the "Ra" field.</p></td>
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
<td><span id="Dn__5"></span>
<p>Is the 64-bit name of the first SIMD&amp;FP source register holding the multiplicand, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dm&gt;</td>
<td><span id="Dm__2"></span>
<p>Is the 64-bit name of the second SIMD&amp;FP source register holding the multiplier, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Da&gt;</td>
<td><span id="Da"></span>
<p>Is the 64-bit name of the third SIMD&amp;FP source register holding the addend, encoded in the "Ra" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let addend : bits(esize) = [V](shared_pseudocode.html#accessor_V_2){esize}(a); let operand1 : bits(esize) = [V](shared_pseudocode.html#accessor_V_2){esize}(n); let operand2 : bits(esize) = V{}(m); var result : bits(128) = if [IsMerging](shared_pseudocode.html#func_IsMerging_1)(FPCR()) then [V](shared_pseudocode.html#accessor_V_2){128}(a) else Zeros{128}; result\[0+:esize\] = [FPMulAdd](shared_pseudocode.html#func_FPMulAdd_5){esize}(addend, operand1, operand2, FPCR()); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>
