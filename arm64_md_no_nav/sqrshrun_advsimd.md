## SQRSHRUN, SQRSHRUN2

Signed saturating rounded shift right unsigned narrow (immediate)

This instruction reads each signed integer value in the vector of the source SIMD&FP register, right shifts each value by an immediate value, saturates the result to an unsigned integer value that is half the original width, places the final result into a vector, and writes the vector to the destination SIMD&FP register. The results are rounded. For truncated results, see <span class="armarm-xref" data-linkend="ARMARM_A64.instructions.SQSHRUN_advsimd">SQSHRUN</span>.

The <span class="asm-code">SQRSHRUN</span> instruction writes the vector to the lower half of the destination register and clears the upper half. The <span class="asm-code">SQRSHRUN2</span> instruction writes the vector to the upper half of the destination register without affecting the other bits of the register.

If saturation occurs, the cumulative saturation bit FPSR.QC is set.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

It has encodings from 2 classes: [Scalar](#iclass_scalar) and [Vector](#iclass_vector)

### <span id="iclass_scalar"></span>Scalar<span style="font-size:smaller;">\
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
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="4" class="lr">!= 0000</td>
<td colspan="3" class="lr">immb</td>
<td class="l">1</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="4" class="droppedname">immh</td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SQRSHRUN_asisdshf_N"></span>

SQRSHRUN [\<Vb\>](#Vb_option "Is the destination width specifier, ")[\<d\>](#d "Is the number of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Va\>](#Va_option "Is the source width specifier, ")[\<n\>](#n__3 "Is the number of the SIMD&FP source register, encoded in the "Rn" field."), \#[\<shift\>](#immh_shift__2 "For the "Scalar" variant: is the right shift amount, in the range 1 to the destination operand width in bits, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh\[3\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 \<\< HighestSetBitNZ(immh\[2:0\]); let datasize : integer{} = esize; let elements : integer = 1; let part : integer = 0; let shift : integer = (2 \* esize) - UInt(immh::immb); let round : boolean = TRUE;

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
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
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
<td colspan="4"></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SQRSHRUN_asimdshf_N"></span>

SQRSHRUN{[2](#s_2_option "Is the second and upper half specifier. If present it causes the operation to be performed on the upper 64 bits of the registers holding the narrower elements, and is ")} [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<Tb\>](#Tb_option "Is an arrangement specifier, "), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<Ta\>](#Ta_option "Is an arrangement specifier, "), \#[\<shift\>](#immh_shift__6 "For the "Vector" variant: is the right shift amount, in the range 1 to the destination element width in bits, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh\[3\] == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 \<\< HighestSetBitNZ(immh\[2:0\]); let datasize : integer{} = 64; let part : integer = UInt(Q); let elements : integer = datasize DIV esize; let shift : integer = (2 \* esize) - UInt(immh::immb); let round : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Vb&gt;</td>
<td><span id="Vb_option"></span>
<p>Is the destination width specifier, encoded in “immh”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">immh</th>
<th class="symbol">&lt;Vb&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="symbol">S</td>
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
<td>&lt;d&gt;</td>
<td><span id="d"></span>
<p>Is the number of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Va&gt;</td>
<td><span id="Va_option"></span>
<p>Is the source width specifier, encoded in “immh”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">immh</th>
<th class="symbol">&lt;Va&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0001</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="symbol">D</td>
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
<td>&lt;n&gt;</td>
<td><span id="n__3"></span>
<p>Is the number of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="immh_shift__2"></span>
<p>For the "Scalar" variant: is the right shift amount, in the range 1 to the destination operand width in bits, encoded in “(immh :: immb)”:</p>
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
<td class="symbol">16 - UInt(immh :: immb)</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="symbol">32 - UInt(immh :: immb)</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="symbol">64 - UInt(immh :: immb)</td>
</tr>
<tr>
<td class="bitfield">1xxx</td>
<td class="symbol">RESERVED</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="immh_shift__6"></span>
<p>For the "Vector" variant: is the right shift amount, in the range 1 to the destination element width in bits, encoded in “(immh :: immb)”:</p>
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
<td class="symbol">16 - UInt(immh :: immb)</td>
</tr>
<tr>
<td class="bitfield">001x</td>
<td class="symbol">32 - UInt(immh :: immb)</td>
</tr>
<tr>
<td class="bitfield">01xx</td>
<td class="symbol">64 - UInt(immh :: immb)</td>
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
<td>&lt;Tb&gt;</td>
<td><span id="Tb_option"></span>
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
<td>&lt;Vn&gt;</td>
<td><span id="Vn"></span>
<p>Is the name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Ta&gt;</td>
<td><span id="Ta_option"></span>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize\*2) = V{}(n); var result : bits(datasize); var element : integer; var sat : boolean; for e = 0 to elements-1 do element = [RShr](shared_pseudocode.html#func_RShr_3)(SInt(operand\[e\*:(2\*esize)\]), shift, round); (result\[e\*:esize\], sat) = [UnsignedSatQ](shared_pseudocode.html#func_UnsignedSatQ_2){esize}(element); if sat then FPSR().QC = '1'; end; end; [Vpart](shared_pseudocode.html#accessor_Vpart_3){datasize}(d, part) = result;

</div>
