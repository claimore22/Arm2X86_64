## MOVI

Move immediate (vector)

This instruction places an immediate constant into every vector element of the destination SIMD&FP register.

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
<td class="lr">op</td>
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
<td colspan="4" class="lr">cmode</td>
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
<td></td>
<td colspan="10"></td>
<td></td>
<td></td>
<td></td>
<td colspan="4"></td>
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

#### Encoding for the 8-bit variant

<span id="MOVI_asimdimm_N_b"></span> Applies when <span class="bitdiff"> (op == 0 && cmode == 1110)</span>

MOVI [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__7 "For the "8-bit" variant: is an arrangement specifier, "), \#[\<imm8\>](#a_b_c_d_e_f_g_h "Is an 8-bit immediate encoded in "a:b:c:d:e:f:g:h"."){, LSL \#0}

</div>

<div class="encoding">

#### Encoding for the 16-bit shifted immediate variant

<span id="MOVI_asimdimm_L_hl"></span> Applies when <span class="bitdiff"> (op == 0 && cmode == 10x0)</span>

MOVI [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__4 "For the "16-bit shifted immediate" variant: is an arrangement specifier, "), \#[\<imm8\>](#a_b_c_d_e_f_g_h "Is an 8-bit immediate encoded in "a:b:c:d:e:f:g:h"."){, LSL \#[\<amount\>](#amount_option__10 "For the "16-bit shifted immediate" variant: is the shift amount ")}

</div>

<div class="encoding">

#### Encoding for the 32-bit shifted immediate variant

<span id="MOVI_asimdimm_L_sl"></span> Applies when <span class="bitdiff"> (op == 0 && cmode == 0xx0)</span>

MOVI [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__8 "For the "32-bit shifted immediate" and "32-bit shifting ones" variants: is an arrangement specifier, "), \#[\<imm8\>](#a_b_c_d_e_f_g_h "Is an 8-bit immediate encoded in "a:b:c:d:e:f:g:h"."){, LSL \#[\<amount\>](#amount_option__8 "For the "32-bit shifted immediate" variant: is the shift amount ")}

</div>

<div class="encoding">

#### Encoding for the 32-bit shifting ones variant

<span id="MOVI_asimdimm_M_sm"></span> Applies when <span class="bitdiff"> (op == 0 && cmode == 110x)</span>

MOVI [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").[\<T\>](#T_option__8 "For the "32-bit shifted immediate" and "32-bit shifting ones" variants: is an arrangement specifier, "), \#[\<imm8\>](#a_b_c_d_e_f_g_h "Is an 8-bit immediate encoded in "a:b:c:d:e:f:g:h"."), MSL \#[\<amount\>](#amount_option__12 "For the "32-bit shifting ones" variant: is the shift amount ")

</div>

<div class="encoding">

#### Encoding for the 64-bit scalar variant

<span id="MOVI_asimdimm_D_ds"></span> Applies when <span class="bitdiff"> (Q == 0 && op == 1 && cmode == 1110)</span>

MOVI [\<Dd\>](#Dd "Is the 64-bit name of the SIMD&FP destination register, encoded in the "Rd" field."), \#[\<imm\>](#a_b_c_d_e_f_g_h__3 "Is a 64-bit immediate 'aaaaaaaabbbbbbbbccccccccddddddddeeeeeeeeffffffffgggggggghhhhhhhh', encoded in "a:b:c:d:e:f:g:h".")

</div>

<div class="encoding">

#### Encoding for the 64-bit vector variant

<span id="MOVI_asimdimm_D2_d"></span> Applies when <span class="bitdiff"> (Q == 1 && op == 1 && cmode == 1110)</span>

MOVI [\<Vd\>](#Vd "Is the name of the SIMD&FP destination register, encoded in the "Rd" field.").2D, \#[\<imm\>](#a_b_c_d_e_f_g_h__3 "Is a 64-bit immediate 'aaaaaaaabbbbbbbbccccccccddddddddeeeeeeeeffffffffgggggggghhhhhhhh', encoded in "a:b:c:d:e:f:g:h".")

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
<td><span id="Vd"></span>
<p>Is the name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T_option__7"></span>
<p>For the "8-bit" variant: is an arrangement specifier, encoded in “Q”:</p>
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
<td class="symbol">8B</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">16B</td>
</tr>
</tbody>
</table></td>
</tr>
<tr>
<td></td>
<td><span id="T_option__4"></span>
<p>For the "16-bit shifted immediate" variant: is an arrangement specifier, encoded in “Q”:</p>
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
<p>For the "32-bit shifted immediate" and "32-bit shifting ones" variants: is an arrangement specifier, encoded in “Q”:</p>
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
<p>For the "16-bit shifted immediate" variant: is the shift amount encoded in “cmode[1]”:</p>
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
<p>For the "32-bit shifted immediate" variant: is the shift amount encoded in “cmode[2:1]”:</p>
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
<tr>
<td></td>
<td><span id="amount_option__12"></span>
<p>For the "32-bit shifting ones" variant: is the shift amount encoded in “cmode[0]”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">cmode[0]</th>
<th class="symbol">&lt;amount&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">0</td>
<td class="symbol">8</td>
</tr>
<tr>
<td class="bitfield">1</td>
<td class="symbol">16</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Dd&gt;</td>
<td><span id="Dd"></span>
<p>Is the 64-bit name of the SIMD&amp;FP destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="a_b_c_d_e_f_g_h__3"></span>
<p>Is a 64-bit immediate 'aaaaaaaabbbbbbbbccccccccddddddddeeeeeeeeffffffffgggggggghhhhhhhh', encoded in "a:b:c:d:e:f:g:h".</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[AArch64_CheckFPAdvSIMDEnabled](shared_pseudocode.html#func_AArch64_CheckFPAdvSIMDEnabled_0)(); [V](shared_pseudocode.html#accessor_V_2){datasize}(rd) = imm;

</div>
