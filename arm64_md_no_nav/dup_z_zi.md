## DUP (indexed)

Broadcast indexed element to vector (unpredicated)

This instruction unconditionally broadcasts the indexed source vector element into each element of the destination vector. This instruction is unpredicated.

The immediate element index is in the range of 0 to 63 (bytes), 31 (halfwords), 15 (words), 7 (doublewords) or 3 (quadwords). Selecting an element beyond the accessible vector length causes the destination vector to be set to zero.

This instruction is used by the alias [MOV (SIMD&FP scalar, unpredicated)](mov_dup_z_zi.html "Move indexed element or SIMD&FP scalar to vector (unpredicated)").

### <span id="iclass_sve"></span>SVE<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">imm2</td>
<td class="lr">1</td>
<td colspan="5" class="lr">tsz</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">Zn</td>
<td colspan="5" class="lr">Zd</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="dup_z_zi_"></span>

DUP [\<Zd\>](#Zd "Is the name of the destination scalable vector register, encoded in the "Zd" field.").[\<T\>](#T__33 "Is the size specifier, "), [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__33 "Is the size specifier, ")\[[\<imm\>](#imm__47 "Is the immediate index, in the range 0 to one less than the number of elements in 512 bits, encoded in "imm2:tsz".")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if tsz == '00000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let lsb : integer{} = LowestSetBit(tsz); let esize : integer{} = 8 \<\< lsb; let imm : bits(7) = imm2::tsz; let index : integer = UInt(imm\[6:(lsb+1)\]); let n : integer = UInt(Zn); let d : integer = UInt(Zd);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Zd&gt;</td>
<td><span id="Zd"></span>
<p>Is the name of the destination scalable vector register, encoded in the "Zd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__33"></span>
<p>Is the size specifier, encoded in “tsz”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">tsz</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="symbol">D</td>
</tr>
<tr>
<td class="bitfield">10000</td>
<td class="symbol">Q</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__47"></span>
<p>Is the immediate index, in the range 0 to one less than the number of elements in 512 bits, encoded in "imm2:tsz".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [MOV (SIMD&FP scalar, unpredicated)](mov_dup_z_zi.html "Move indexed element or SIMD&FP scalar to vector (unpredicated)") | <span class="pseudocode">BitCount(imm2 :: tsz) == 1</span> |
| [MOV (SIMD&FP scalar, unpredicated)](mov_dup_z_zi.html "Move indexed element or SIMD&FP scalar to vector (unpredicated)") | <span class="pseudocode">BitCount(imm2 :: tsz) \> 1</span> |

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let elements : integer = VL DIV esize; let operand1 : bits(VL) = if index \< elements then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(VL); var element : bits(esize); if index \>= elements then element = Zeros{esize}; else element = operand1\[index\*:esize\]; end; result = Replicate{VL}(element); [Z](shared_pseudocode.html#accessor_Z_2){VL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
