## SSHR

Signed shift right (immediate)

This instruction reads each vector element in the source SIMD&FP register, right shifts each result by an immediate value, places the final result into a vector, and writes the vector to the destination SIMD&FP register. All the values in this instruction are signed integer values. The results are truncated. For rounded results, see <span class="armarm-xref" data-linkend="ARMARM_A64.instructions.SRSHR_advsimd">SRSHR</span>.

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
<td class="lr">0</td>
<td class="lr">1</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="l">1</td>
<td>x</td>
<td>x</td>
<td class="r">x</td>
<td colspan="3" class="lr">immb</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
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
<td colspan="2"></td>
<td class="droppedname">o1</td>
<td class="droppedname">o0</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SSHR_asisdshf_R"></span>

SSHR D[\<d\>](#d "Is the number of the SIMD&FP destination register, encoded in the "Rd" field."), D[\<n\>](#n__2 "Is the number of the first SIMD&FP source register, encoded in the "Rn" field."), \#[\<shift\>](#r_128UIntimmhimmb "For the "Scalar" variant: is the right shift amount, in the range 1 to 64, encoded as 128 - UInt("immh:immb").")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh\[3\] != '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 \<\< 3; let datasize : integer{} = esize; let elements : integer = 1; let shift : integer = (esize \* 2) - UInt(immh::immb); let unsigned : boolean = FALSE; let round : boolean = FALSE;

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td class="r">0</td>
<td colspan="4" class="lr">!= 0000</td>
<td colspan="3" class="lr">immb</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
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
<td colspan="2"></td>
<td class="droppedname">o1</td>
<td class="droppedname">o0</td>
<td></td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="SSHR_asimdshf_R"></span>

SSHR [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__15 "Is an arrangement specifier, "), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__15 "Is an arrangement specifier, "), \#[\<shift\>](#immh_shift__4 "For the "Vector" variant: is the right shift amount, in the range 1 to the element width in bits, ")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if immh\[3\]::Q == '10' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 \<\< HighestSetBitNZ(immh); let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize; let shift : integer = (esize \* 2) - UInt(immh::immb); let unsigned : boolean = FALSE; let round : boolean = FALSE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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
<td>&lt;n&gt;</td>
<td><span id="n__2"></span>
<p>Is the number of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;shift&gt;</td>
<td><span id="r_128UIntimmhimmb"></span>
<p>For the "Scalar" variant: is the right shift amount, in the range 1 to 64, encoded as 128 - UInt("immh:immb").</p></td>
</tr>
<tr>
<td></td>
<td><span id="immh_shift__4"></span>
<p>For the "Vector" variant: is the right shift amount, in the range 1 to the element width in bits, encoded in “(immh :: immb)”:</p>
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
<td class="symbol">128 - UInt(immh :: immb)</td>
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
<td>&lt;T&gt;</td>
<td><span id="T_option__15"></span>
<p>Is an arrangement specifier, encoded in “(immh :: Q)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">immh</th>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
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
<td class="bitfield">0</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">1xxx</td>
<td class="bitfield">1</td>
<td class="symbol">2D</td>
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

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(n); var result : bits(datasize); var element : integer; for e = 0 to elements-1 do let opelt : bits(esize) = operand\[e\*:esize\]; if unsigned then element = [RShr](shared_pseudocode.html#func_RShr_3)(UInt(opelt), shift, round); else element = [RShr](shared_pseudocode.html#func_RShr_3)(SInt(opelt), shift, round); end; result\[e\*:esize\] = element\[esize-1:0\]; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
