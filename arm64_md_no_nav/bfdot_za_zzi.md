## BFDOT (multiple and indexed vector)

Multi-vector BFloat16 dot product by indexed element to single-precision

This instruction calculates the dot product of a pair of BF16 values held in the corresponding 32-bit elements of the two or four first source vectors and the indexed 32-bit element of the second source vector. The single-precision dot product results are destructively added to the corresponding single-precision elements of the ZA single-vector groups.

The BF16 pairs within the second source vector are specified using an immediate index that selects the same BF16 pair position within each 128-bit vector segment. The element index range is from 0 to 3.

The single-vector group within each half of or each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo half or quarter the number of ZA array vectors.

The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA single-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code.

This instruction follows SME2 ZA-targeting BFloat16 numerical behaviors.

This instruction is unpredicated.

It has encodings from 2 classes: [Two ZA single-vectors](#iclass_two_za_single_vectors) and [Four ZA single-vectors](#iclass_four_za_single_vectors)

### <span id="iclass_two_za_single_vectors"></span>Two ZA single-vectors<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i2</td>
<td colspan="4" class="lr">Zn</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
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
<td class="droppedname">op</td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="3" class="droppedname">opc2</td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bfdot_za_zzi_2xi"></span>

BFDOT ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs\>](#offs "Is the vector select offset, in the range 0 to 7, encoded in the "off3" field."){, VGx2}\], { [\<Zn1\>](#Zn1__2 "For the "Two ZA single-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.").H-[\<Zn2\>](#Zn2__2 "Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" times 2 plus 1.").H }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").H\[[\<index\>](#index__17 "Is the immediate index of a pair of 16-bit elements within each 128-bit vector segment, in the range 0 to 3, encoded in the "i2" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'0'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off3); let index : integer = UInt(i2); let nreg : integer{} = 2;

### <span id="iclass_four_za_single_vectors"></span>Four ZA single-vectors<span style="font-size:smaller;">\
(FEAT_SME2) </span>

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
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">1</td>
<td colspan="2" class="lr">Rv</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i2</td>
<td colspan="3" class="lr">Zn</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
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
<td class="droppedname">op</td>
<td colspan="2"></td>
<td colspan="3"></td>
<td></td>
<td colspan="3" class="droppedname">opc2</td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bfdot_za_zzi_4xi"></span>

BFDOT ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs\>](#offs "Is the vector select offset, in the range 0 to 7, encoded in the "off3" field."){, VGx4}\], { [\<Zn1\>](#Zn1__3 "For the "Four ZA single-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.").H-[\<Zn4\>](#Zn4__2 "Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" times 4 plus 3.").H }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").H\[[\<index\>](#index__17 "Is the immediate index of a pair of 16-bit elements within each 128-bit vector segment, in the range 0 to 3, encoded in the "i2" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'00'); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off3); let index : integer = UInt(i2); let nreg : integer{} = 4;

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
<p>For the "Two ZA single-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zn1__3"></span>
<p>For the "Four ZA single-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.</p></td>
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
<td><span id="index__17"></span>
<p>Is the immediate index of a pair of 16-bit elements within each 128-bit vector segment, in the range 0 to 3, encoded in the "i2" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn4&gt;</td>
<td><span id="Zn4__2"></span>
<p>Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 32; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let eltspersegment : integer = 128 DIV 32; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(n+r); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = ZAvector{}(vec); for e = 0 to elements-1 do let elt1_a : bits(16) = operand1\[(2 \* e + 0)\*:16\]; let elt1_b : bits(16) = operand1\[(2 \* e + 1)\*:16\]; let segmentbase : integer = e - (e MOD eltspersegment); let s : integer = segmentbase + index; let elt2_a : bits(16) = operand2\[(2 \* s + 0)\*:16\]; let elt2_b : bits(16) = operand2\[(2 \* s + 1)\*:16\]; var sum : bits(32) = operand3\[e\*:32\]; sum = [BFDotAdd](shared_pseudocode.html#func_BFDotAdd_6)(sum, elt1_a, elt1_b, elt2_a, elt2_b, FPCR()); result\[e\*:32\] = sum; end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec) = result; vec = vec + vstride; end;

</div>
