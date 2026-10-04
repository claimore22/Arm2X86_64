## FVDOTB

Multi-vector 8-bit floating-point vertical dot product by indexed element to single-precision (bottom)

This instruction calculates the fused sum-of-products of each vertical pair of 8-bit floating-point values held in the corresponding elements of the two first source vectors with the lower-numbered horizontal pair of 8-bit floating-point values in the indexed 32-bit group of the corresponding 128-bit segment of the second source vector. The single-precision sum-of-products are scaled by 2<sup>-UInt(FPMR.LSCALE)</sup>, before being destructively added without intermediate rounding to the corresponding single-precision elements of the four ZA single-vector groups. The 8-bit floating-point encoding format for the elements of the first source vector and the second source vector is selected by FPMR.F8S1 and FPMR.F8S2 respectively.

The 8-bit floating-point groups within the second source vector are specified using an immediate index that selects the same group position within each 128-bit vector segment.

The single-vector group within each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo quarter the number of ZA array vectors.

The vector group symbol VGx4 indicates that the ZA operand consists of four ZA single-vector groups.

This instruction is unpredicated.

### <span id="iclass_sme2"></span>SME2<span style="font-size:smaller;">\
(FEAT_SME_F8F32) </span>

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
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">i2h</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">i2l</td>
<td colspan="3" class="lr">off3</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">T</td>
<td></td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="fvdotb_za32_z8z8i_2xi"></span>

FVDOTB ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs\>](#offs "Is the vector select offset, in the range 0 to 7, encoded in the "off3" field."), VGx4\], { [\<Zn1\>](#Zn1__2 "For the "Two ZA single-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.").B-[\<Zn2\>](#Zn2__2 "Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" times 2 plus 1.").B }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").B\[[\<index\>](#index__25 "Is the immediate index of a group of four 8-bit values within each 128-bit vector segment, in the range 0 to 3, encoded in the "i2h:i2l" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_F8F32) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'0'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off3); let index : integer = UInt(i2h::i2l);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wv&gt;</td>
<td><span id="Wv"></span>
<p>Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs&gt;</td>
<td><span id="offs"></span>
<p>Is the vector select offset, in the range 0 to 7, encoded in the "off3" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__2"></span>
<p>Is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2__2"></span>
<p>Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm&gt;</td>
<td><span id="Zm__2"></span>
<p>Is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="index__25"></span>
<p>Is the immediate index of a group of four 8-bit values within each 128-bit vector segment, in the range 0 to 3, encoded in the "i2h:i2l" fields.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckFPMREnabled](shared_pseudocode.html#func_CheckFPMREnabled_0)(); [CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV 4; let eltspersegment : integer = 128 DIV 32; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); for r = 0 to 3 do let operand1a : bits(VL) = Z{}(n+0); let operand1b : bits(VL) = Z{}(n+1); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = ZAvector{}(vec); for e = 0 to elements-1 do var op1 : bits(16); op1\[0+:8\] = operand1a\[(4 \* e + r)\*:8\]; op1\[1\*:8\] = operand1b\[(4 \* e + r)\*:8\]; let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = 2\*(segmentbase + index); let op2 : bits(16) = operand2\[s\*:16\]; var sum : bits(32) = operand3\[e\*:(32)\]; sum = [FP8DotAddFP](shared_pseudocode.html#func_FP8DotAddFP_7){32, 16}(sum, op1, op2, FPCR(), FPMR()); result\[e\*:(32)\] = sum; end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec) = result; vec = vec + vstride; end;

</div>
