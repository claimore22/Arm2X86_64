## USHLL, USHLL2

Unsigned shift left long (immediate)

This instruction reads each vector element in the lower or upper half of the source SIMD&FP register, shifts the unsigned integer value left by the specified number of bits, places the result into a vector, and writes the vector to the destination SIMD&FP register. The destination vector elements are twice as long as the source vector elements.

The <span class="asm-code">USHLL</span> instruction extracts vector elements from the lower half of the source register. The <span class="asm-code">USHLL2</span> instruction extracts vector elements from the upper half of the source register.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

This instruction is used by the alias [UXTL, UXTL2](uxtl_ushll_advsimd.html "Unsigned extend long").

### <span id="iclass_vector"></span>Vector<span style="font-size:smaller;">\
(FEAT_AdvSIMD) </span>

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
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="4" class="lr">!= 0000</td>
<td colspan="3" class="lr">immb</td>
<td class="l">1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="4" class="droppedname">immh</td>
<td colspan="3"></td>
<td colspan="5" class="droppedname">opcode</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="USHLL_asimdshf_L"></span>

USHLL{[2](#s_2_option "Is the second and upper half specifier. If present it causes the operation to be performed on the upper 64 bits of the registers holding the narrower elements, and is ")} [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<Ta\>](#Ta_option_new "Is an arrangement specifier, "), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<Tb\>](#Tb_option_new "Is an arrangement specifier, "), \#[\<shift\>](#immh_shift__new "Is the left shift amount, in the range 0 to the source element width in bits minus 1, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh\[3\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 \<\< HighestSetBitNZ(immh\[2:0\]); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize; let shift : integer = UInt(immh::immb) - esize; let unsigned : boolean = TRUE;

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
<td><span id="Ta_option_new"></span>
<p>Is an arrangement specifier, encoded in “immh”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">immh</th>
<th class="symbol">&lt;Ta&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">8H</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="symbol">4S</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="symbol">2D</td>
</tr>
<tr>
<td class="bitfield">1xxx</td>
<td class="symbol">RESERVED</td>
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

<table>
<tbody>
<tr>
<td>&lt;Tb&gt;</td>
<td><span id="Tb_option_new"></span>
<p>Is an arrangement specifier, encoded in “(immh :: Q)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">immh</th>
<th class="bitfield">Q</th>
<th class="symbol">&lt;Tb&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0001</td>
<td class="bitfield">0</td>
<td class="symbol">8B</td>
</tr>
<tr>
<td class="bitfield">0001</td>
<td class="bitfield">1</td>
<td class="symbol">16B</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="bitfield">0</td>
<td class="symbol">4H</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="bitfield">1</td>
<td class="symbol">8H</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="bitfield">0</td>
<td class="symbol">2S</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="bitfield">1</td>
<td class="symbol">4S</td>
</tr>
<tr>
<td class="bitfield">1xxx</td>
<td class="bitfield">x</td>
<td class="symbol">RESERVED</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="immh_shift__new"></span>
<p>Is the left shift amount, in the range 0 to the source element width in bits minus 1, encoded in “(immh :: immb)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">immh</th>
<th class="symbol">&lt;shift&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">UInt(immh :: immb) - 8</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="symbol">UInt(immh :: immb) - 16</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="symbol">UInt(immh :: immb) - 32</td>
</tr>
<tr>
<td class="bitfield">1xxx</td>
<td class="symbol">RESERVED</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

### Alias Conditions

| Alias | Is preferred when |
|----|----|
| [UXTL, UXTL2](uxtl_ushll_advsimd.html "Unsigned extend long") | <span class="pseudocode">immb == '000' && BitCount(immh) == 1</span> |

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = Vpart{}(n, part); var result : bits(datasize\*2); for e = 0 to elements-1 do let opelt : bits(esize) = operand\[e\*:esize\]; let element : integer = if unsigned then UInt(opelt) else SInt(opelt); result\[e\*:(2\*esize)\] = (element \<\< shift)\[2\*esize-1:0\]; end; [V](shared_pseudocode.html#accessor_V_2){datasize\*2}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
