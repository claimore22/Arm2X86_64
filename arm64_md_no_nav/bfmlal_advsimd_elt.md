## BFMLALB, BFMLALT (by element)

BFloat16 multiply-add to single-precision (by element)

This instruction widens the even-numbered (bottom) or odd-numbered (top) 16-bit elements in the first source vector, and the indexed element in the second source vector from Bfloat16 to single-precision format. The instruction then multiplies and adds these values without intermediate rounding to single-precision elements of the destination vector that overlap with the corresponding BFloat16 elements in the first source vector.

ID_AA64ISAR1_EL1.BF16 indicates whether this instruction is supported.

### <span id="iclass_vector"></span>Vector<span style="font-size:smaller;">\
(FEAT_BF16) </span>

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
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">L</td>
<td class="lr">M</td>
<td colspan="4" class="lr">Rm</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">H</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="4" class="droppedname">opcode</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="BFMLAL_asimdelem_F"></span>

BFMLAL[\<bt\>](#bt_option "Is the bottom or top element specifier, ") [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").4S, [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").8H, [\<Vm\>](#Vm__2 "For the "Scalar, half-precision" and "Vector, half-precision" variants: is the name of the second SIMD&FP source register, in the range V0 to V15, encoded in the "Rm" field.").H\[[\<index\>](#index__8 "For the "Scalar, half-precision" and "Vector, half-precision" variants: is the element index, in the range 0 to 7, encoded in the "H:L:M" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_BF16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer = UInt(Rn); let m : integer = UInt('0'::Rm); let d : integer = UInt(Rd); let index : integer = UInt(H::L::M); let elements : integer = 128 DIV 32; let sel : integer = UInt(Q);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;bt&gt;</td>
<td><span id="bt_option"></span>
<p>Is the bottom or top element specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;bt&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">T</td>
</tr>
</tbody>
</table></td>
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
<td><span id="Vm__2"></span>
<p>Is the name of the second SIMD&amp;FP source register, in the range V0 to V15, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="index__8"></span>
<p>Is the element index, in the range 0 to 7, encoded in the "H:L:M" fields.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); var result : bits(128); let operand1 : bits(128) = V{}(n); let operand2 : bits(128) = V{}(m); let operand3 : bits(128) = V{}(d); let element2 : bits(16) = operand2\[index\*:16\]; for e = 0 to elements-1 do let element1 : bits(16) = operand1\[(2 \* e + sel)\*:16\]; let addend : bits(32) = operand3\[e\*:32\]; result\[e\*:32\] = [BFMulAddH](shared_pseudocode.html#func_BFMulAddH_4)(addend, element1, element2, FPCR()); end; [V](shared_pseudocode.html#accessor_V_2){128}(d) = result;

</div>
