## AESD (indexed)

Multi-vector AES single round decryption

This instruction reads a 16-byte state array from each 128-bit segment of the two or four first source vectors, together with a round key from the indexed 128-bit segment of the corresponding 512-bit portion of the second source vector. Each state array undergoes a single round of the AddRoundKey(), InvShiftRows(), and InvSubBytes() transformations in accordance with the AES standard. Each updated state array is destructively placed in the corresponding segment of the two or four first source vectors.

When the vector length is less than 512 bits, the most significant bits of the index are ignored to select the indexed 128-bit segment of the second source vector. This instruction is unpredicated.

This instruction is legal when executed in Streaming SVE mode if both FEAT_SSVE_AES and FEAT_SVE_AES2 are implemented.

It has encodings from 2 classes: [Two registers](#iclass_two_registers) and [Four registers](#iclass_four_registers)

### <span id="iclass_two_registers"></span>Two registers<span style="font-size:smaller;">\
(FEAT_SVE_AES2) </span>

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
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i2</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="4" class="lr">Zdn</td>
<td class="lr">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td class="droppedname">op</td>
<td colspan="3"></td>
<td colspan="2"></td>
<td class="droppedname">o2</td>
<td colspan="5"></td>
<td colspan="4"></td>
<td class="droppedname">o3</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="aesd_mz_zzi_2x1"></span>

AESD { [\<Zdn1\>](#Zdn1 "For the "Two registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2.").B-[\<Zdn2\>](#Zdn2 "Is the name of the second scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2 plus 1.").B }, { [\<Zdn1\>](#Zdn1 "For the "Two registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2.").B-[\<Zdn2\>](#Zdn2 "Is the name of the second scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2 plus 1.").B }, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").Q\[[\<index\>](#index_aes "Is the round key index, in the range 0 to 3, encoded in the "i2" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE_AES2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let m : integer = UInt(Zm); let dn : integer = UInt(Zdn::'0'); var index : integer = UInt(i2); let nreg : integer{} = 2;

### <span id="iclass_four_registers"></span>Four registers<span style="font-size:smaller;">\
(FEAT_SVE_AES2) </span>

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
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">i2</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Zm</td>
<td colspan="3" class="lr">Zdn</td>
<td class="l">0</td>
<td class="r">0</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2" class="droppedname">size</td>
<td></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td class="droppedname">op</td>
<td colspan="3"></td>
<td colspan="2"></td>
<td class="droppedname">o2</td>
<td colspan="5"></td>
<td colspan="3"></td>
<td colspan="2" class="droppedname">opc3</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="aesd_mz_zzi_4x1"></span>

AESD { [\<Zdn1\>](#Zdn1__2 "For the "Four registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4.").B-[\<Zdn4\>](#Zdn4 "Is the name of the fourth scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4 plus 3.").B }, { [\<Zdn1\>](#Zdn1__2 "For the "Four registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4.").B-[\<Zdn4\>](#Zdn4 "Is the name of the fourth scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4 plus 3.").B }, [\<Zm\>](#Zm "Is the name of the second source scalable vector register, encoded in the "Zm" field.").Q\[[\<index\>](#index_aes "Is the round key index, in the range 0 to 3, encoded in the "i2" field.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE_AES2) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let m : integer = UInt(Zm); let dn : integer = UInt(Zdn::'00'); var index : integer = UInt(i2); let nreg : integer{} = 4;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zdn1&gt;</td>
<td><span id="Zdn1"></span>
<p>For the "Two registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2.</p></td>
</tr>
<tr>
<td></td>
<td><span id="Zdn1__2"></span>
<p>For the "Four registers" variant: is the name of the first scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zdn2&gt;</td>
<td><span id="Zdn2"></span>
<p>Is the name of the second scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 2 plus 1.</p></td>
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
<td>&lt;index&gt;</td>
<td><span id="index_aes"></span>
<p>Is the round key index, in the range 0 to 3, encoded in the "i2" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zdn4&gt;</td>
<td><span id="Zdn4"></span>
<p>Is the name of the fourth scalable vector register of the destination and first source multi-vector group, encoded as "Zdn" times 4 plus 3.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if IsFeatureImplemented(FEAT_SSVE_AES) then [CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); else [CheckNonStreamingSVEEnabled](shared_pseudocode.html#func_CheckNonStreamingSVEEnabled_0)(); end; let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); if VL == 128 then index = 0; end; if VL == 256 then index = index MOD 2; end; let segments : integer = VL DIV 128; let operand2 : bits(VL) = Z{}(m); var results : array \[\[4\]\] of bits(VL); for r = 0 to nreg-1 do let operand1 : bits(VL) = Z{}(dn + r); for s = 0 to segments-1 do let keyindex : integer = (s - (s MOD 4)) + index; let res : bits(128) = operand1\[s\*:128\] XOR operand2\[keyindex\*:128\]; results\[\[r\]\]\[s\*:128\] = [AESInvSubBytes](shared_pseudocode.html#func_AESInvSubBytes_1)([AESInvShiftRows](shared_pseudocode.html#func_AESInvShiftRows_1)(res)); end; end; for r = 0 to nreg-1 do [Z](shared_pseudocode.html#accessor_Z_2){VL}(dn + r) = results\[\[r\]\]; end;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
