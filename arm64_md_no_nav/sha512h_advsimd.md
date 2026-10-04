## SHA512H

SHA-512 hash update part 1

This instruction takes the values from the three 128-bit source SIMD&FP registers and produces a 128-bit output value that combines the sigma1 and chi functions of two iterations of the SHA-512 calculation. It returns this value to the destination SIMD&FP register.

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
(FEAT_SHA512) </span>

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
<td class="r">1</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="4"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td></td>
<td class="droppedname">O</td>
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

<span id="SHA512H_QQV_cryptosha512_3"></span>

SHA512H [\<Qd\>](#Qd__2 "Is the 128-bit name of the SIMD&FP source and destination register, encoded in the "Rd" field."), [\<Qn\>](#Qn "Is the 128-bit name of the second SIMD&FP source register, encoded in the "Rn" field."), [\<Vm\>](#Vm__7 "Is the name of the third SIMD&FP source register, encoded in the "Rm" field.").2D

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SHA512) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Qd&gt;</td>
<td><span id="Qd__2"></span>
<p>Is the 128-bit name of the SIMD&amp;FP source and destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Qn&gt;</td>
<td><span id="Qn"></span>
<p>Is the 128-bit name of the second SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var Vtmp : bits(128); var MSigma1 : bits(64); var tmp : bits(64); let x : bits(128) = V{}(n); let y : bits(128) = V{}(m); let w : bits(128) = V{}(d); MSigma1 = ROR(y\[127:64\], 14) XOR ROR(y\[127:64\], 18) XOR ROR(y\[127:64\], 41); Vtmp\[127:64\] = (y\[127:64\] AND x\[63:0\]) XOR (NOT(y\[127:64\]) AND x\[127:64\]); Vtmp\[127:64\] = (Vtmp\[127:64\] + MSigma1 + w\[127:64\]); tmp = Vtmp\[127:64\] + y\[63:0\]; MSigma1 = ROR(tmp, 14) XOR ROR(tmp, 18) XOR ROR(tmp, 41); Vtmp\[63:0\] = (tmp AND y\[127:64\]) XOR (NOT(tmp) AND x\[63:0\]); Vtmp\[63:0\] = (Vtmp\[63:0\] + MSigma1 + w\[63:0\]); [V](shared_pseudocode.html#accessor_V_2){128}(d) = Vtmp;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
