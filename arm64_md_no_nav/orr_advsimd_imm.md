## ORR (vector, immediate)

Bitwise inclusive OR (vector, immediate)

This instruction reads each vector element from the destination SIMD&FP register, performs a bitwise OR between each result and an immediate constant, places the result into a vector, and writes the vector to the destination SIMD&FP register.

Depending on the settings in the CPACR_EL1, CPTR_EL2, and CPTR_EL3 registers, and the current Security state and Exception level, an attempt to execute the instruction might be trapped.

### <span id="iclass_shifted_immediate"></span>Shifted immediate<span style="font-size:smaller;">\
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
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">a</td>
<td class="lr">b</td>
<td class="lr">c</td>
<td class="l">x</td>
<td>x</td>
<td>x</td>
<td class="r">1</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td class="lr">d</td>
<td class="lr">e</td>
<td class="lr">f</td>
<td class="lr">g</td>
<td class="lr">h</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td></td>
<td></td>
<td class="droppedname">op</td>
<td colspan="10"></td>
<td></td>
<td></td>
<td></td>
<td colspan="4" class="droppedname">cmode</td>
<td class="droppedname">o2</td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 16-bit variant

<span id="ORR_asimdimm_L_hl"></span> Applies when <span class="bitdiff"> (cmode == 10x1)</span>

ORR [\<Vd\>](#Vd__2 "Is the name of the SIMD&FP register, encoded in the "Rd" field.").[\<T\>](#T_option__4 "For the "16-bit" variant: is an arrangement specifier, "), \#[\<imm8\>](#a_b_c_d_e_f_g_h "Is an 8-bit immediate encoded in "a:b:c:d:e:f:g:h"."){, LSL \#[\<amount\>](#amount_option__10 "For the "16-bit" variant: is the shift amount ")}

</div>

<div class="encoding">

#### Encoding for the 32-bit variant

<span id="ORR_asimdimm_L_sl"></span> Applies when <span class="bitdiff"> (cmode == 0xx1)</span>

ORR [\<Vd\>](#Vd__2 "Is the name of the SIMD&FP register, encoded in the "Rd" field.").[\<T\>](#T_option__8 "For the "32-bit" variant: is an arrangement specifier, "), \#[\<imm8\>](#a_b_c_d_e_f_g_h "Is an 8-bit immediate encoded in "a:b:c:d:e:f:g:h"."){, LSL \#[\<amount\>](#amount_option__8 "For the "32-bit" variant: is the shift amount ")}

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_AdvSIMD) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let rd : integer = UInt(Rd); let datasize : integer{} = 64 \<\< UInt(Q); let imm64 : bits(64) = [AdvSIMDExpandImm](shared_pseudocode.html#func_AdvSIMDExpandImm_3)(op, cmode, a::b::c::d::e::f::g::h); let imm : bits(datasize) = Replicate{}(imm64);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Vd&gt;</td>
<td><span id="Vd__2"></span>
<p>Is the name of the SIMD&amp;FP register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T_option__4"></span>
<p>For the "16-bit" variant: is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">4H</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">8H</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="T_option__8"></span>
<p>For the "32-bit" variant: is an arrangement specifier, encoded in “Q”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">Q</th>
<th class="symbol">&lt;T&gt;</th>
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
<td>&lt;imm8&gt;</td>
<td><span id="a_b_c_d_e_f_g_h"></span>
<p>Is an 8-bit immediate encoded in "a:b:c:d:e:f:g:h".</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;amount&gt;</td>
<td><span id="amount_option__10"></span>
<p>For the "16-bit" variant: is the shift amount encoded in “cmode[1]”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">cmode[1]</th>
<th class="symbol">&lt;amount&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">0</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">8</td>
</tr>
</tbody>
</table>
defaulting to 0 if LSL is omitted.</td>
</tr>
<tr>
<td></td>
<td><span id="amount_option__8"></span>
<p>For the "32-bit" variant: is the shift amount encoded in “cmode[2:1]”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">cmode[2:1]</th>
<th class="symbol">&lt;amount&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">0</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">8</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">16</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">24</td>
</tr>
</tbody>
</table>
defaulting to 0 if LSL is omitted.</td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); let operand : bits(datasize) = V{}(rd); [V](shared_pseudocode.html#accessor_V_2){datasize}(rd) = operand OR imm;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.
