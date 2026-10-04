## FMLAL, FMLAL2 (by element)

Floating-point fused multiply-add long to accumulator (by element)

This instruction multiplies the half-precision vector elements in the first source SIMD&FP register by the specified half-precision value in the second source SIMD&FP register, and accumulates the intermediate product without rounding to the corresponding single-precision vector element of the destination SIMD&FP register.

This instruction can generate a floating-point exception. Depending on the settings in FPCR, the exception results in either a flag being set in FPSR or a synchronous exception being generated. For more information, see <span class="armarm-xref" data-linkend="ARMARM_BEIJDDAG">Floating-point exceptions and exception traps</span>.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

In Armv8.2 and Armv8.3, this is an <span class="arm-defined-word">OPTIONAL</span> instruction. From Armv8.4, it is mandatory for all implementations to support it.

<div class="note">

------------------------------------------------------------------------

#### Note

ID_AA64ISAR0_EL1.FHM indicates whether this instruction is supported.

------------------------------------------------------------------------

</div>

It has encodings from 2 classes: [FMLAL](#iclass_fmlal) and [FMLAL2](#iclass_fmlal2)

### <span id="iclass_fmlal"></span>FMLAL<span style="font-size:smaller;">\
(FEAT_FHM) </span>

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
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">L</td>
<td class="lr">M</td>
<td colspan="4" class="lr">Rm</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">H</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td></td>
<td class="droppedname">sz</td>
<td></td>
<td></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="2"></td>
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

<span id="FMLAL_asimdelem_LH"></span>

FMLAL [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<Ta\>](#Ta_option__6 "Is an arrangement specifier, "), [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").[\<Tb\>](#Tb_option__6 "Is an arrangement specifier, "), [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").H\[[\<index\>](#index__13 "Is the element index, encoded in the "H:L:M" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_FHM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sz == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let m : integer = UInt('0'::Rm); // Vm can only be in bottom 16 registers. let index : integer = UInt(H::L::M); let esize : integer{} = 32; let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize; let part : integer = 0;

### <span id="iclass_fmlal2"></span>FMLAL2<span style="font-size:smaller;">\
(FEAT_FHM) </span>

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
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">L</td>
<td class="lr">M</td>
<td colspan="4" class="lr">Rm</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">H</td>
<td class="lr">0</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">U</td>
<td></td>
<td colspan="3"></td>
<td></td>
<td></td>
<td class="droppedname">sz</td>
<td></td>
<td></td>
<td colspan="4"></td>
<td></td>
<td class="droppedname">S</td>
<td colspan="2"></td>
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

<span id="FMLAL2_asimdelem_LH"></span>

FMLAL2 [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<Ta\>](#Ta_option__6 "Is an arrangement specifier, "), [\<Vn\>](#Vn__2 "Is the name of the first SIMD&FP source register, encoded in the "Rn" field.").[\<Tb\>](#Tb_option__6 "Is an arrangement specifier, "), [\<Vm\>](#Vm "Is the name of the second SIMD&FP source register, encoded in the "Rm" field.").H\[[\<index\>](#index__13 "Is the element index, encoded in the "H:L:M" fields.")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_FHM) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if sz == '1' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let m : integer = UInt('0'::Rm); // Vm can only be in bottom 16 registers. let index : integer = UInt(H::L::M); let esize : integer{} = 32; let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize; let part : integer = 1;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

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
<td><span id="Ta_option__6"></span>
<p>Is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;Ta&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">2S</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">4S</td>
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
<td><span id="Vn__2"></span>
<p>Is the name of the first SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Tb&gt;</td>
<td><span id="Tb_option__6"></span>
<p>Is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;Tb&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">2H</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">4H</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Vm&gt;</td>
<td><span id="Vm"></span>
<p>Is the name of the second SIMD&amp;FP source register, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="index__13"></span>
<p>Is the element index, encoded in the "H:L:M" fields.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand1 : bits(datasize DIV 2) = Vpart{}(n, part); let operand2 : bits(128) = V{}(m); let operand3 : bits(datasize) = V{}(d); var result : bits(datasize); var element1 : bits(esize DIV 2); let element2 : bits(esize DIV 2) = operand2\[index\*:(esize DIV 2)\]; for e = 0 to elements-1 do element1 = operand1\[e\*:(esize DIV 2)\]; result\[e\*:esize\] = [FPMulAddH](shared_pseudocode.html#func_FPMulAddH_4)(operand3\[e\*:esize\], element1, element2, FPCR()); end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;

</div>
