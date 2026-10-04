## FMOV (register)

Floating-point move register without conversion

This instruction copies the floating-point value in the SIMD&FP source register to the SIMD&FP destination register.

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
<td class="r">0</td>
<td colspan="2" class="lr">ftype</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
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
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="2" class="droppedname">opc</td>
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

<span id="FMOV_H_floatdp1"></span> Applies when <span class="bitdiff"> (ftype == 11)</span>

FMOV [\<Hd\>](#Hd "Is the 16-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Hn\>](#Hn__2 "Is the 16-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the Single-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMOV_S_floatdp1"></span> Applies when <span class="bitdiff"> (ftype == 00)</span>

FMOV [\<Sd\>](#Sd "Is the 32-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Sn\>](#Sn "Is the 32-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the Double-precision variant <span style="font-size:smaller;">\
(FEAT_FP) </span>

<span id="FMOV_D_floatdp1"></span> Applies when <span class="bitdiff"> (ftype == 01)</span>

FMOV [\<Dd\>](#Dd "Is the 64-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Dn\>](#Dn "Is the 64-bit name of the SIMD&FP source register, encoded in the "Rn" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if ftype == '11' && !IsFeatureImplemented(FEAT_FP16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 \<\< UInt(ftype XOR '10');

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
<td><span id="Hn__2"></span>
<p>Is the 16-bit name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
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

[AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); let operand : bits(esize) = V{}(n); var result : bits(128) = Zeros{}; result\[0+:esize\] = operand; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>
