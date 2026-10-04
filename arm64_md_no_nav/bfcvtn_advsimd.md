## BFCVTN, BFCVTN2

Single-precision convert to BFloat16 (vector)

This instruction reads each single-precision element in the SIMD&FP source vector, converts each value to BFloat16 format, and writes the results in the lower or upper half of the SIMD&FP destination vector. The result elements are half the width of the source elements.

The <span class="asm-code">BFCVTN</span> instruction writes the half-width results to the lower half of the 128-bit SIMD&FP destination vector and clears the upper half to zero. The <span class="asm-code">BFCVTN2</span> instruction writes the results to the upper half of the 128-bit SIMD&FP destination vector without affecting the lower half.

### <span id="iclass_vector_single_precision_to_bfloat16"></span>Vector single-precision to BFloat16<span style="font-size:smaller;">\
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
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
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
<td colspan="5"></td>
<td colspan="5" class="droppedname">opcode</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="BFCVTN_asimdmisc_4S"></span>

BFCVTN{[2](#s_2_option "Is the second and upper half specifier. If present it causes the operation to be performed on the upper 64 bits of the registers holding the narrower elements, and is ")} [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<Ta\>](#Ta_option__7 "Is an arrangement specifier, "), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").4S

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_BF16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let part : integer = UInt(Q); let elements : integer = 64 DIV 16;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>2</td>
<td><span id="s_2_option"></span>
<p>Is the second and upper half specifier. If present it causes the operation to be performed on the upper 64 bits of the registers holding the narrower elements, and is encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">2</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">[absent]</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">[present]</td>
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
<td>&lt;Ta&gt;</td>
<td><span id="Ta_option__7"></span>
<p>Is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;Ta&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">4H</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">8H</td>
</tr>
</tbody>
</table></td>
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

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(128) = V{}(n); var result : bits(64); for e = 0 to elements-1 do result\[e\*:16\] = [FPConvertBF](shared_pseudocode.html#func_FPConvertBF_2)(operand\[e\*:32\], FPCR()); end; [Vpart](shared_pseudocode.html#accessor_Vpart_3){64}(d, part) = result;

</div>
