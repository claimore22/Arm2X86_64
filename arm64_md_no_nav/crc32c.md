## CRC32CB, CRC32CH, CRC32CW, CRC32CX

CRC32C checksum

This instruction performs a cyclic redundancy check (CRC) calculation on a value held in a general-purpose register. It takes an input CRC value in the first source operand, performs a CRC on the input value in the second source operand, and returns the output CRC value. The second source operand can be 8, 16, 32, or 64 bits. To align with common usage, the bit order of the values is reversed as part of the operation, and the polynomial 0x1EDC6F41 is used for the CRC calculation.

In an Armv8.0 implementation, this is an <span class="arm-defined-word">OPTIONAL</span> instruction. From Armv8.1, it is mandatory for all implementations to implement this instruction.

<div class="note">

------------------------------------------------------------------------

#### Note

ID_AA64ISAR0_EL1.CRC32 indicates whether this instruction is supported.

------------------------------------------------------------------------

</div>

### <span id="iclass_crc"></span>CRC<span style="font-size:smaller;">\
(FEAT_CRC32) </span>

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
<td class="lr">sf</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td colspan="5" class="lr">Rm</td>
<td class="l">0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">sz</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="8"></td>
<td colspan="5"></td>
<td colspan="3"></td>
<td class="droppedname">C</td>
<td colspan="2"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the CRC32CB variant

<span id="CRC32CB_32C_dp_2src"></span> Applies when <span class="bitdiff"> (sf == 0 && sz == 00)</span>

CRC32CB [\<Wd\>](#WdOrWZR__2 "Is the 32-bit name of the general-purpose accumulator output register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR__4 "Is the 32-bit name of the general-purpose accumulator input register, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__5 "Is the 32-bit name of the general-purpose data source register, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the CRC32CH variant

<span id="CRC32CH_32C_dp_2src"></span> Applies when <span class="bitdiff"> (sf == 0 && sz == 01)</span>

CRC32CH [\<Wd\>](#WdOrWZR__2 "Is the 32-bit name of the general-purpose accumulator output register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR__4 "Is the 32-bit name of the general-purpose accumulator input register, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__5 "Is the 32-bit name of the general-purpose data source register, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the CRC32CW variant

<span id="CRC32CW_32C_dp_2src"></span> Applies when <span class="bitdiff"> (sf == 0 && sz == 10)</span>

CRC32CW [\<Wd\>](#WdOrWZR__2 "Is the 32-bit name of the general-purpose accumulator output register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR__4 "Is the 32-bit name of the general-purpose accumulator input register, encoded in the "Rn" field."), [\<Wm\>](#WmOrWZR__5 "Is the 32-bit name of the general-purpose data source register, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the CRC32CX variant

<span id="CRC32CX_64C_dp_2src"></span> Applies when <span class="bitdiff"> (sf == 1 && sz == 11)</span>

CRC32CX [\<Wd\>](#WdOrWZR__2 "Is the 32-bit name of the general-purpose accumulator output register, encoded in the "Rd" field."), [\<Wn\>](#WnOrWZR__4 "Is the 32-bit name of the general-purpose accumulator input register, encoded in the "Rn" field."), [\<Xm\>](#XmOrXZR__8 "Is the 64-bit name of the general-purpose data source register, encoded in the "Rm" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_CRC32) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); if sf == '1' && sz != '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sf == '0' && sz == '11' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = 8 \<\< UInt(sz);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR__2"></span>
<p>Is the 32-bit name of the general-purpose accumulator output register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wn&gt;</td>
<td><span id="WnOrWZR__4"></span>
<p>Is the 32-bit name of the general-purpose accumulator input register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Wm&gt;</td>
<td><span id="WmOrWZR__5"></span>
<p>Is the 32-bit name of the general-purpose data source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm&gt;</td>
<td><span id="XmOrXZR__8"></span>
<p>Is the 64-bit name of the general-purpose data source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let acc : bits(32) = X{}(n); // accumulator let val : bits(size) = X{}(m); // input value let poly : bits(32) = 0x1EDC6F41\[31:0\]; let tempacc : bits(32+size) = [BitReverse](shared_pseudocode.html#func_BitReverse_2){32}(acc)::Zeros{size}; let tempval : bits(size+32) = [BitReverse](shared_pseudocode.html#func_BitReverse_2){size}(val)::Zeros{32}; // Poly32Mod2 on a bitstring does a polynomial Modulus over {0,1} operation [X](shared_pseudocode.html#accessor_X_2){32}(d) = [BitReverse](shared_pseudocode.html#func_BitReverse_2){32}([Poly32Mod2](shared_pseudocode.html#func_Poly32Mod2_3){32+size}(tempacc XOR tempval, poly));

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
