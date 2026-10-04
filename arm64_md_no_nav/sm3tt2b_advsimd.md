## SM3TT2B

SM3TT2B

This instruction takes three 128-bit vectors from three source SIMD&FP registers, and a 2-bit immediate index value, and returns a 128-bit result in the destination SIMD&FP register. It performs a 32-bit majority function between the three 32-bit fields held in the upper three elements of the first source vector, and adds the resulting 32-bit value and the following three other 32-bit values:

- The bottom 32-bit element of the first source vector, Vd, that was used for the 32-bit majority function.
- The 32-bit element held in the top 32 bits of the second source vector, Vn.
- A 32-bit element indexed out of the third source vector, Vm.

A three-way exclusive-OR is performed of the result of this addition, the result of the addition rotated left by 9, and the result of the addition rotated left by 17. The result of this exclusive-OR is returned as the top element of the returned result. The other elements of this result are taken from elements of the first source vector, with the element returned in bits\<63:32\> being rotated left by 19.

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
(FEAT_SM3) </span>

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
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="2" class="lr">imm2</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="2" class="droppedname">opcode</td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SM3TT2B_VVV_crypto3_imm2"></span>

SM3TT2B [\<Vd\>](#Vd__4 "Is the name of the SIMD&FP source and destination register, encoded in the "Rd" field.").4S, [\<Vn\>](#Vn__6 "Is the name of the second SIMD&FP source register, encoded in the "Rn" field.").4S, [\<Vm\>](#Vm__7 "Is the name of the third SIMD&FP source register, encoded in the "Rm" field.").S\[[\<imm2\>](#index__16 "Is a 32-bit element indexed out of <Vm>, encoded in "imm2".")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SM3) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let i : integer = UInt(imm2);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Vd&gt;</td>
<td><span id="Vd__4"></span>
<p>Is the name of the SIMD&amp;FP source and destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vn&gt;</td>
<td><span id="Vn__6"></span>
<p>Is the name of the second SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vm&gt;</td>
<td><span id="Vm__7"></span>
<p>Is the name of the third SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm2&gt;</td>
<td><span id="index__16"></span>
<p>Is a 32-bit element indexed out of &lt;Vm&gt;, encoded in "imm2".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let Vm : bits(128) = V{}(m); let Vn : bits(128) = V{}(n); let Vd : bits(128) = V{}(d); var Wj : bits(32); var result : bits(128); var TT2 : bits(32); Wj = Vm\[i\*:32\]; TT2 = (Vd\[127:96\] AND Vd\[95:64\]) OR (NOT(Vd\[127:96\]) AND Vd\[63:32\]); TT2 = (TT2 + Vd\[31:0\] + Vn\[127:96\] + Wj)\[31:0\]; result\[31:0\] = Vd\[63:32\]; result\[63:32\] = ROL(Vd\[95:64\], 19); result\[95:64\] = Vd\[127:96\]; result\[127:96\] = TT2 XOR ROL(TT2, 9) XOR ROL(TT2, 17); [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
