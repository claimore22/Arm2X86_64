## BFSUB

Multi-vector BFloat16 subtract from ZA array vectors

This instruction destructively subtracts all elements of the two or four source vectors from the corresponding BFloat16 elements of the ZA single-vector groups.

The single-vector group within each half of or each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo half or quarter the number of ZA array vectors.

The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA single-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code.

This instruction follows SME2 ZA-targeting non-widening BFloat16 numerical behaviors.

This instruction is unpredicated.

ID_AA64SMFR0_EL1.B16B16 indicates whether this instruction is implemented.

It has encodings from 2 classes: [Two ZA single-vectors](#iclass_two_za_single_vectors) and [Four ZA single-vectors](#iclass_four_za_single_vectors)

### <span id="iclass_two_za_single_vectors"></span>Two ZA single-vectors<span style="font-size:smaller;">\
(FEAT_SME_B16B16) </span>

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
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="3" class="lr">off3</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td class="droppedname">sz</td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bfsub_za_zw_2x2_16"></span>

BFSUB ZA.H\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs\>](#offs "Is the vector select offset, in the range 0 to 7, encoded in the "off3" field."){, VGx2}\], { [\<Zm1\>](#Zm1 "For the "Two ZA single-vectors" variant: is the name of the first scalable vector register of the source multi-vector group, encoded as "Zm" times 2.").H-[\<Zm2\>](#Zm2 "Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zm" times 2 plus 1.").H }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_B16B16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let m : integer = UInt(Zm::'0'); let offset : integer = UInt(off3); let nreg : integer{} = 2;

### <span id="iclass_four_za_single_vectors"></span>Four ZA single-vectors<span style="font-size:smaller;">\
(FEAT_SME_B16B16) </span>

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
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td colspan="3" class="lr">Zm</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="3" class="lr">off3</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td class="droppedname">sz</td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="bfsub_za_zw_4x4_16"></span>

BFSUB ZA.H\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs\>](#offs "Is the vector select offset, in the range 0 to 7, encoded in the "off3" field."){, VGx4}\], { [\<Zm1\>](#Zm1__2 "For the "Four ZA single-vectors" variant: is the name of the first scalable vector register of the source multi-vector group, encoded as "Zm" times 4.").H-[\<Zm4\>](#Zm4 "Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zm" times 4 plus 3.").H }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME_B16B16) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let m : integer = UInt(Zm::'00'); let offset : integer = UInt(off3); let nreg : integer{} = 4;

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
<td>&lt;Zm1&gt;</td>
<td><span id="Zm1"></span>
<p>For the "Two ZA single-vectors" variant: is the name of the first scalable vector register of the source multi-vector group, encoded as "Zm" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zm1__2"></span>
<p>For the "Four ZA single-vectors" variant: is the name of the first scalable vector register of the source multi-vector group, encoded as "Zm" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm2&gt;</td>
<td><span id="Zm2"></span>
<p>Is the name of the second scalable vector register of the source multi-vector group, encoded as "Zm" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm4&gt;</td>
<td><span id="Zm4"></span>
<p>Is the name of the fourth scalable vector register of the source multi-vector group, encoded as "Zm" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV 16; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = ZAvector{}(vec); let operand2 : bits(VL) = Z{}(m+r); for e = 0 to elements-1 do let element1 : bits(16) = operand1\[e\*:16\]; let element2 : bits(16) = operand2\[e\*:16\]; result\[e\*:16\] = [BFSub_ZA](shared_pseudocode.html#func_BFSub_ZA_3)(element1, element2, FPCR()); end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec) = result; vec = vec + vstride; end;

</div>
