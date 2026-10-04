## FMLALLBB, FMLALLBT, FMLALLTB, FMLALLTT (vector)

8-bit floating-point multiply-add to single-precision (vector)

This instruction widens the first (bottom bottom), second (bottom top), third (top bottom), or fourth (top top) 8-bit element of each 32-bit container in the first and second source vectors to single-precision format and multiplies the corresponding elements. The intermediate products are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup>, before being destructively added without intermediate rounding to the single-precision elements of the destination vector that overlap with the corresponding 8-bit floating-point elements in the source vectors.

The 8-bit floating-point encoding format for the elements of the first source vector is selected by FPMR.F8S1. The 8-bit floating-point encoding format for the elements of the second source vector is selected by FPMR.F8S2.

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
(FEAT_FP8FMA) </span>

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
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">x</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">U</td>
<td colspan="5"></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="5"></td>
<td></td>
<td colspan="4" class="droppedname">opcode</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the FMLALLBB variant

<span id="FMLALLBB_asimdsame2_G"></span> Applies when <span class="bitdiff"> (Q == 0 && size == 00)</span>

FMLALLBB [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").4S, [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").16B, [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").16B

</div>

<div class="encoding">

#### Encoding for the FMLALLBT variant

<span id="FMLALLBT_asimdsame2_G"></span> Applies when <span class="bitdiff"> (Q == 0 && size == 01)</span>

FMLALLBT [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").4S, [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").16B, [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").16B

</div>

<div class="encoding">

#### Encoding for the FMLALLTB variant

<span id="FMLALLTB_asimdsame2_G"></span> Applies when <span class="bitdiff"> (Q == 1 && size == 00)</span>

FMLALLTB [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").4S, [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").16B, [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").16B

</div>

<div class="encoding">

#### Encoding for the FMLALLTT variant

<span id="FMLALLTT_asimdsame2_G"></span> Applies when <span class="bitdiff"> (Q == 1 && size == 01)</span>

FMLALLTT [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").4S, [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").16B, [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").16B

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_FP8FMA) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let elements : integer = 128 DIV 32; let sel : integer = UInt(Q::size\[0\]);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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
<td>&lt;Vn&gt;</td>
<td><span id="Vn__2"></span>
<p>Is the name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vm&gt;</td>
<td><span id="Vm"></span>
<p>Is the name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(128) = V{}(n); let operand2 : bits(128) = V{}(m); let operand3 : bits(128) = V{}(d); var result : bits(128); for e = 0 to elements-1 do let element1 : bits(8) = operand1\[(4 \* e + sel)\*:8\]; let element2 : bits(8) = operand2\[(4 \* e + sel)\*:8\]; let element3 : bits(32) = operand3\[e\*:32\]; result\[e\*:32\] = [FP8MulAddFP](shared_pseudocode.html#func_FP8MulAddFP_6){32}(element3, element1, element2, FPCR(), FPMR()); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>
