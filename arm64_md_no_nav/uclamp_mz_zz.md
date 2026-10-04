## UCLAMP

Multi-vector unsigned clamp to minimum/maximum

This instruction clamps each unsigned element in the two or four destination vectors to between the unsigned minimum value in the corresponding element of the first source vector and the unsigned maximum value in the corresponding element of the second source vector and destructively places the clamped results in the corresponding elements of the two or four destination vectors.

This instruction is unpredicated.

It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

### <span id="iclass_two_registers"></span>Two registers<span style="font-size:smaller;">\
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
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="4" class="lr">Zd</td>
<td class="lr">1</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="4"></td>
<td class="droppedname">U</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uclamp_mz_zz_2"></span>

UCLAMP { [\<Zd1\>](#Zd1 "For the "Two registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 2.").[\<T\>](#T__36 "Is the size specifier, ")-[\<Zd2\>](#Zd2 "For the "Consecutive" variant: is the name of the second scalable vector register of the destination multi-vector group, encoded as "Zd" times 2 plus 1.").[\<T\>](#T__36 "Is the size specifier, ") }, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__36 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__36 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd::'0'); let nreg : integer{} = 2;

### <span id="iclass_four_registers"></span>Four registers<span style="font-size:smaller;">\
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
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td class="l">1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>1</td>
<td class="r">1</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="3" class="lr">Zd</td>
<td class="lr">0</td>
<td class="lr">1</td>
</tr>
<tr class="secondrow">
<td></td>
<td colspan="2"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td></td>
<td class="droppedname">U</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="uclamp_mz_zz_4"></span>

UCLAMP { [\<Zd1\>](#Zd1__2 "For the "Four registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.").[\<T\>](#T__36 "Is the size specifier, ")-[\<Zd4\>](#Zd4 "For the "Consecutive" variant: is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.").[\<T\>](#T__36 "Is the size specifier, ") }, [\<Zn\>](#Zn__2 "Is the name of the first source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__36 "Is the size specifier, "), [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").[\<T\>](#T__36 "Is the size specifier, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SME2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let n : integer = UInt(Zn); let m : integer = UInt(Zm); let d : integer = UInt(Zd::'00'); let nreg : integer{} = 4;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd1&gt;</td>
<td><span id="Zd1"></span>
<p>For the "Two registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zd1__2"></span>
<p>For the "Four registers" variant: is the name of the first scalable vector register of the destination multi-vector group, encoded as "Zd" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__36"></span>
<p>Is the size specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
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
<td>&lt;Zd2&gt;</td>
<td><span id="Zd2"></span>
<p>Is the name of the second scalable vector register of the destination multi-vector group, encoded as "Zd" times 2 plus 1.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn__2"></span>
<p>Is the name of the first source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zm&gt;</td>
<td><span id="Zm"></span>
<p>Is the name of the second source scalable vector register, encoded in the "Zm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zd4&gt;</td>
<td><span id="Zd4"></span>
<p>Is the name of the fourth scalable vector register of the destination multi-vector group, encoded as "Zd" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckStreamingSVEEnabled](shared_pseudocode.html#func_CheckStreamingSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; var results : array \[\[4\]\] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(n); let operand2 : bits(VL) = Z{}(m); let operand3 : bits(VL) = Z{}(d+r); for e = 0 to elements-1 do let element1 : integer = UInt(operand1\[e\*:esize\]); let element2 : integer = UInt(operand2\[e\*:esize\]); let element3 : integer = UInt(operand3\[e\*:esize\]); let res : integer = Min(Max(element1, element3), element2); results\[\[r\]\]\[e\*:esize\] = res\[esize-1:0\]; end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(d+r) = results\[\[r\]\]; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
