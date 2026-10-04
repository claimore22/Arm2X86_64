## SMOV

Signed move vector element to general-purpose register

This instruction reads the signed integer from the source SIMD&FP register, sign-extends it to form a 32-bit or 64-bit value, and writes the result to destination general-purpose register.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

### <span id="iclass_advanced_simd"></span>Advanced SIMD<span style="font-size:smaller;">\
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
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">0</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td class="lr">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">op</td>
<td colspan="8"></td>
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

#### Encoding for the 32-bit variant

<span id="SMOV_asimdins_W_w"></span> Applies when <span class="bitdiff"> (Q == 0)</span>

SMOV [\<Wd\>](#WdOrWZR "Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<Ts\>](#Ts_option__2 "For the "32-bit" variant: is an element size specifier, ")\[[\<index\>](#imm5_index__2 "For the "32-bit" variant: is the element index ")\]

</div>

<div class="encoding">

#### Encoding for the 64-bit variant

<span id="SMOV_asimdins_X_x"></span> Applies when <span class="bitdiff"> (Q == 1)</span>

SMOV [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Vn\>](#Vn "Is the name of the SIMD&FP source register, encoded in the "Rn" field.").[\<Ts\>](#Ts_option__3 "For the "64-bit" variant: is an element size specifier, ")\[[\<index\>](#imm5_index__3 "For the "64-bit" variant: is the element index ")\]

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if imm5 == 'xx000' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let size : integer{} = LowestSetBitNZ(imm5\[2:0\]); let d : integer{} = UInt(Rd); let n : integer{} = UInt(Rn); let esize : integer{} = 8 \<\< size; let datasize : integer{} = 32 \<\< UInt(Q); if datasize \<= esize then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let index : integer = UInt(imm5\[4:size+1\]); let idxdsize : integer{} = 64 \<\< UInt(imm5\[4\]);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wd&gt;</td>
<td><span id="WdOrWZR"></span>
<p>Is the 32-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
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
<td>&lt;Ts&gt;</td>
<td><span id="Ts_option__2"></span>
<p>For the "32-bit" variant: is an element size specifier, encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;Ts&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">xxx00</td>
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
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="Ts_option__3"></span>
<p>For the "64-bit" variant: is an element size specifier, encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;Ts&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">xx000</td>
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
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;index&gt;</td>
<td><span id="imm5_index__2"></span>
<p>For the "32-bit" variant: is the element index encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;index&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">xxx00</td>
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
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="imm5_index__3"></span>
<p>For the "64-bit" variant: is the element index encoded in “imm5”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">imm5</th>
<th class="symbol">&lt;index&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">xx000</td>
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
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if index == 0 then [AArch64_CheckFPEnabled](shared_pseudocode.html#func_AArch64_CheckFPEnabled_0)(); else [AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); end; let operand : bits(idxdsize) = V{}(n); [X](shared_pseudocode.html#accessor_X_2){datasize}(d) = SignExtend{datasize}(operand\[index\*:esize\]);

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
