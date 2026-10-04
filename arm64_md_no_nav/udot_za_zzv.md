## UDOT (4-way, multiple and single vector)

Multi-vector unsigned integer dot product by vector

This instruction calculates the dot product of four unsigned 8-bit or 16-bit integer values held in each 32-bit or 64-bit element of the two or four first source vectors and four unsigned 8-bit or 16-bit integer values in the corresponding 32-bit or 64-bit element of the second source vector. The widened dot product result is destructively added to the corresponding 32-bit or 64-bit element of the ZA single-vector groups.

The single-vector group within each half of or each quarter of the ZA array is selected by the sum of the vector select register and offset, modulo half or quarter the number of ZA array vectors.

The vector group symbol, VGx2 or VGx4, indicates that the ZA operand consists of two or four ZA single-vector groups respectively. The vector group symbol is preferred for disassembly, but optional in assembler source code.

This instruction is unpredicated.

ID_AA64SMFR0_EL1.I16I64 indicates whether the 16-bit integer variant is implemented.

It has encodings from 2 classes: [Two ZA single-vectors](#iclass_two_za_single_vectors) and [Four ZA single-vectors](#iclass_four_za_single_vectors)

### <span id="iclass_two_za_single_vectors"></span>Two ZA single-vectors<span style="font-size:smaller;">\
(FEAT_SME2 && (sz == '0' \|\| FEAT_SME_I16I64)) </span>

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
<td class="r">0</td>
<td class="lr">sz</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="3" class="lr">off3</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="udot_za_zzv_2x1"></span>

UDOT ZA.[\<T\>](#T__83 "Is the size specifier, ")\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs\>](#offs "Is the vector select offset, in the range 0 to 7, encoded in the "off3" field."){, VGx2}\], { [\<Zn1\>](#Zn1 "Is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn".").[\<Tb\>](#Tb__13 "Is the size specifier, ")-[\<Zn2\>](#Zn2 "Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" plus 1 modulo 32.").[\<Tb\>](#Tb__13 "Is the size specifier, ") }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").[\<Tb\>](#Tb__13 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sz == '1' && !IsFeatureImplemented(FEAT_SME_I16I64) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let esize : integer{} = 32 \<\< UInt(sz); let n : integer = UInt(Zn); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off3); let nreg : integer{} = 2;

### <span id="iclass_four_za_single_vectors"></span>Four ZA single-vectors<span style="font-size:smaller;">\
(FEAT_SME2 && (sz == '0' \|\| FEAT_SME_I16I64)) </span>

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
<td class="r">0</td>
<td class="lr">sz</td>
<td class="l">1</td>
<td class="r">1</td>
<td colspan="4" class="lr">Zm</td>
<td class="lr">0</td>
<td colspan="2" class="lr">Rv</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td colspan="3" class="lr">off3</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td colspan="2"></td>
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="udot_za_zzv_4x1"></span>

UDOT ZA.[\<T\>](#T__83 "Is the size specifier, ")\[[\<Wv\>](#Wv "Is the 32-bit name of the vector select register W8-W11, encoded in the "Rv" field."), [\<offs\>](#offs "Is the vector select offset, in the range 0 to 7, encoded in the "off3" field."){, VGx4}\], { [\<Zn1\>](#Zn1 "Is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn".").[\<Tb\>](#Tb__13 "Is the size specifier, ")-[\<Zn4\>](#Zn4 "Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" plus 3 modulo 32.").[\<Tb\>](#Tb__13 "Is the size specifier, ") }, [\<Zm\>](#Zm__2 "For the "Double-precision" variant: is the name of the second source scalable vector register Z0-Z15, encoded in the "Zm" field.").[\<Tb\>](#Tb__13 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sz == '1' && !IsFeatureImplemented(FEAT_SME_I16I64) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let v : integer = UInt('010'::Rv); let esize : integer{} = 32 \<\< UInt(sz); let n : integer = UInt(Zn); let m : integer = UInt('0'::Zm); let offset : integer = UInt(off3); let nreg : integer{} = 4;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__83"></span>
<p>Is the size specifier, encoded in “sz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sz</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">D</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

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
<td><span id="Zn1"></span>
<p>Is the name of the first scalable vector register of the first source multi-vector group, encoded as "Zn".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Tb&gt;</td>
<td><span id="Tb__13"></span>
<p>Is the size specifier, encoded in “sz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">sz</th>
<th class="symbol">&lt;Tb&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">H</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn2&gt;</td>
<td><span id="Zn2"></span>
<p>Is the name of the second scalable vector register of the first source multi-vector group, encoded as "Zn" plus 1 modulo 32.</p></td>
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
<td>&lt;Zn4&gt;</td>
<td><span id="Zn4"></span>
<p>Is the name of the fourth scalable vector register of the first source multi-vector group, encoded as "Zn" plus 3 modulo 32.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEAndZAEnabled](shared_pseudocode.html#func_CheckStreamingSVEAndZAEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let vectors : integer = VL DIV 8; let vstride : integer = vectors DIV nreg; let vbase : bits(32) = X{}(v); var vec : integer = (UInt(vbase) + offset) MOD vstride; var result : bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}((n+r) MOD 32); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = ZAvector{}(vec); for e = 0 to elements-1 do var sum : bits(esize) = operand3\[e\*:esize\]; for i = 0 to 3 do let element1 : integer = UInt(operand1\[(4 \* e + i)\*:(esize DIV 4)\]); let element2 : integer = UInt(operand2\[(4 \* e + i)\*:(esize DIV 4)\]); sum = sum + element1 \* element2; end; result\[e\*:esize\] = sum; end; [ZAvector](shared_pseudocode.html#accessor_ZAvector_2){VL}(vec) = result; vec = vec + vstride; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
