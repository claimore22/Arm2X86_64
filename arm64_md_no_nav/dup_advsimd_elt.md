## DUP (element)

Duplicate vector element to vector or scalar

This instruction duplicates the vector element at the specified element index in the source SIMD&FP register into a scalar or each element in a vector, and writes the result to the destination SIMD&FP register.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

This instruction is used by the alias [MOV (scalar)](mov_dup_advsimd_elt.html "Move vector element to scalar").

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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td colspan="2"></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td></td>
<td colspan="4" class="droppedname">imm4</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="DUP_asisdone_only"></span>

DUP [\<V\>](#V_option__3 "Is the destination width specifier, ")[\<d\>](#d "Is the number of the SIMD&FP destination register, encoded in the "Rd" field."), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<T\>](#T_option__12 "For the "Scalar" variant: is the element width specifier, ")\[[\<index\>](#imm5_index__7 "Is the element index ")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5\[3:0\]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let index : integer = UInt(imm5\[4:size+1\]); let idxdsize : integer{} = 64 \<\< UInt(imm5\[4\]); let esize : integer{} = 8 \<\< size; let datasize : integer{} = esize; let elements : integer = 1;

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
<td class="l">0</td>
<td class="r">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">op</td>
<td></td>
<td colspan="3"></td>
<td colspan="2"></td>
<td colspan="2"></td>
<td colspan="5"></td>
<td></td>
<td colspan="4" class="droppedname">imm4</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="DUP_asimdins_DV_v"></span>

DUP [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__5 "For the "Vector" variant: is an arrangement specifier, "), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<Ts\>](#Ts_option "Is an element size specifier, ")\[[\<index\>](#imm5_index__7 "Is the element index ")\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x0000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'x1000' && Q == '0' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5\[3:0\]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let index : integer = UInt(imm5\[4:size+1\]); let idxdsize : integer{} = 64 \<\< UInt(imm5\[4\]); let esize : integer{} = 8 \<\< size; let datasize : integer{} = 64 \<\< UInt(Q); let elements : integer = datasize DIV esize;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;V&gt;</td>
<td><span id="V_option__3"></span>
<p>Is the destination width specifier, encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;V&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
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
<td>&lt;Vn&gt;</td>
<td><span id="Vn"></span>
<p>Is the name of the SIMD&amp;FP source register, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T_option__12"></span>
<p>For the "Scalar" variant: is the element width specifier, encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
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
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="T_option__5"></span>
<p>For the "Vector" variant: is an arrangement specifier, encoded in “(imm5 :: Q)”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
<td class="bitfield">x</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="bitfield">0</td>
<td class="symbol">8B</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="bitfield">1</td>
<td class="symbol">16B</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="bitfield">0</td>
<td class="symbol">4H</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="bitfield">1</td>
<td class="symbol">8H</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="bitfield">0</td>
<td class="symbol">2S</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="bitfield">1</td>
<td class="symbol">4S</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="bitfield">0</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
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
<td>&lt;index&gt;</td>
<td><span id="imm5_index__7"></span>
<p>Is the element index encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;index&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
<td class="symbol">RESERVED</td>
</tr>
<tr>
<td class="bitfield">xxxx1</td>
<td class="symbol">UInt(imm5[4:1])</td>
</tr>
<tr>
<td class="bitfield">xxx10</td>
<td class="symbol">UInt(imm5[4:2])</td>
</tr>
<tr>
<td class="bitfield">xx100</td>
<td class="symbol">UInt(imm5[4:3])</td>
</tr>
<tr>
<td class="bitfield">x1000</td>
<td class="symbol">UInt(imm5[4])</td>
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
<td>&lt;Ts&gt;</td>
<td><span id="Ts_option"></span>
<p>Is an element size specifier, encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;Ts&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">x0000</td>
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

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(idxdsize) = V{}(n); var result : bits(datasize); var element : bits(esize); element = operand\[index\*:esize\]; for e = 0 to elements-1 do result\[e\*:esize\] = element; end; [V](shared_pseudocode.html#accessor_V_2){datasize}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
