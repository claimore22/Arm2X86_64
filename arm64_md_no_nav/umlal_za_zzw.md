## UMLAL (multiple vectors)

Multi-vector unsigned 16-bit integer multiply-add to 32-bit integer

This instruction multiplies each unsigned 16-bit element in the two or four first source vectors by each unsigned 16-bit element in the two or four second source vectors, widens each product to 32 bits, and destructively adds these values to the corresponding 32-bit elements of the ZA double-vector groups.

The double-vector group within each half of or each quarter of the ZA array is selected by the sum of the vector select register and offset range, modulo half or quarter the number of ZA array vectors.

The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA double-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code.

This instruction is unpredicated.

It has encodings from 2 classes: [Two ZA double-vectors](#iclass_two_za_double_vectors) and [Four ZA double-vectors](#iclass_four_za_double_vectors)

### <span id="iclass_two_za_double_vectors"></span>Two ZA double-vectors<span style="font-size:smaller;">\
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
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zn</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">off2</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">U</td>
<td class="droppedname">S</td>
<td></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="umlal_za_zzw_2x2"></span>

UMLAL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the first vector select offset, encoded as "off2" field times 2."):[\<offs2\>](#offs2__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the second vector select offset, encoded as "off2" field times 2 plus 1."){, VGx2}\], { [\<Zn1\>](#Zn1__2 "For the "Two ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.").H-[\<Zn2\>](#Zn2__2 "Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" times 2 plus 1.").H }, { [\<Zm1\>](#Zm1__3 "For the "Two ZA double-vectors" variant: is the name of the first scalable vector register of the second source multi-vector group, encoded as "Zm" times 2.").H-[\<Zm2\>](#Zm2__2 "Is the name of the second scalable vector register of the second source multi-vector group, encoded as "Zm" times 2 plus 1.").H }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'0'); let m : integer = UInt(Zm::'0'); let offset : integer = UInt(off2::'0'); let nreg : integer{} = 2;

### <span id="iclass_four_za_double_vectors"></span>Four ZA double-vectors<span style="font-size:smaller;">\
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
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Zm</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="3" class="lr">Zn</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">off2</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td></td>
<td></td>
<td colspan="3"></td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td class="droppedname">U</td>
<td class="droppedname">S</td>
<td></td>
<td colspan="2"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="umlal_za_zzw_4x4"></span>

UMLAL ZA.S\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs1\>](#offs1__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the first vector select offset, encoded as "off2" field times 2."):[\<offs2\>](#offs2__2 "For the "Four ZA double-vectors" and "Two ZA double-vectors" variants: is the second vector select offset, encoded as "off2" field times 2 plus 1."){, VGx4}\], { [\<Zn1\>](#Zn1__3 "For the "Four ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.").H-[\<Zn4\>](#Zn4__2 "Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" times 4 plus 3.").H }, { [\<Zm1\>](#Zm1__4 "For the "Four ZA double-vectors" variant: is the name of the first scalable vector register of the second source multi-vector group, encoded as "Zm" times 4.").H-[\<Zm4\>](#Zm4__2 "Is the name of the fourth scalable vector register of the second source multi-vector group, encoded as "Zm" times 4 plus 3.").H }

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 32; let v : integer = UInt('010'::Rv); let n : integer = UInt(Zn::'00'); let m : integer = UInt(Zm::'00'); let offset : integer = UInt(off2::'0'); let nreg : integer{} = 4;

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
<td>&lt;offs1&gt;</td>
<td><span id="offs1__2"></span>
<p>Is the first vector select offset, encoded as "off2" field times 2.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;offs2&gt;</td>
<td><span id="offs2__2"></span>
<p>Is the second vector select offset, encoded as "off2" field times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn1&gt;</td>
<td><span id="Zn1__2"></span>
<p>For the "Two ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zn1__3"></span>
<p>For the "Four ZA double-vectors" variant: is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn" times 4.</p></td>
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
<td>&lt;Zm1&gt;</td>
<td><span id="Zm1__3"></span>
<p>For the "Two ZA double-vectors" variant: is the name of the first scalable vector register of the second source multi-vector group, encoded as "Zm" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zm1__4"></span>
<p>For the "Four ZA double-vectors" variant: is the name of the first scalable vector register of the second source multi-vector group, encoded as "Zm" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm2&gt;</td>
<td><span id="Zm2__2"></span>
<p>Is the name of the second scalable vector register of the second source multi-vector group, encoded as "Zm" times 2 plus 1.</p></td>
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

<table>
<tbody>
<tr>
<td>&lt;Zm4&gt;</td>
<td><span id="Zm4__2"></span>
<p>Is the name of the fourth scalable vector register of the second source multi-vector group, encoded as "Zm" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); vec = vec - (vec MOD 2); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(n+r); let operand2 : bits(VL) = Z{}(m+r); for i = 0 to 1 do let operand3 : bits(VL) = ZAvector{}(vec + i); for e = 0 to elements-1 do let element1 : integer = UInt(operand1\[(2 \* e + i)\*:(esize DIV 2)\]); let element2 : integer = UInt(operand2\[(2 \* e + i)\*:(esize DIV 2)\]); let product : bits(esize) = (element1 \* element2)\[esize-1:0\]; result\[e\*:esize\] = operand3\[e\*:esize\] + product; end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec + i) = result; end; vec = vec + vstride; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
