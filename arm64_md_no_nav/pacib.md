## PACIB, PACIB1716, PACIBSP, PACIBZ, PACIZB

Pointer Authentication Code for instruction address, using key B

This instruction calculates and inserts a Pointer Authentication Code for an instruction address, using a modifier and key B.

The address is:

- In the general-purpose register that is specified by \<Xd\> for <span class="asm-code">PACIB</span> and <span class="asm-code">PACIZB</span>.
- In X17, for <span class="asm-code">PACIB1716</span>.
- In X30, for <span class="asm-code">PACIBSP</span> and <span class="asm-code">PACIBZ</span>.

The modifier is:

- In the general-purpose register or stack pointer that is specified by \<Xn\|SP\> for <span class="asm-code">PACIB</span>.
- The value zero, for <span class="asm-code">PACIZB</span> and <span class="asm-code">PACIBZ</span>.
- In X16, for <span class="asm-code">PACIB1716</span>.
- In SP, for <span class="asm-code">PACIBSP</span>.

If <span class="armarm-xref" data-linkend="ARMARM_FEAT_PAuth_LR">FEAT_PAuth_LR</span> is implemented and PSTATE.PACM is 1, then <span class="asm-code">PACIB1716</span> and <span class="asm-code">PACIBSP</span> include a second modifier that is:

- In X15, for <span class="asm-code">PACIB1716</span>.
- The value of PC, for <span class="asm-code">PACIBSP</span>.

A <span class="asm-code">PACIBSP</span> instruction has an implicit <span class="asm-code">BTI</span> instruction. The implicit <span class="asm-code">BTI</span> instruction of a <span class="asm-code">PACIBSP</span> instruction is always compatible with <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span>.BTYPE == 0b01 and <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span>.BTYPE == 0b10. Controls in <span class="armarm-xref" data-linkend="ARMARM_SCTLR_ELx">SCTLR_ELx</span> configure whether the implicit <span class="asm-code">BTI</span> instruction of a <span class="asm-code">PACIBSP</span> instruction is compatible with <span class="armarm-xref" data-linkend="ARMARM_PSTATE">PSTATE</span>.BTYPE == 0b11. For more information, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.PSTATE_BTYPE">PSTATE.BTYPE</span>.

It has encodings from 2 classes: [Integer](#iclass_integer) and [System](#iclass_system)

### <span id="iclass_integer"></span>Integer<span style="font-size:smaller;">\
(FEAT_PAuth) </span>

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
<td class="lr">1</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="lr">Z</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rd</td>
</tr>
<tr class="secondrow">
<td class="droppedname">sf</td>
<td></td>
<td class="droppedname">S</td>
<td colspan="8"></td>
<td colspan="5" class="droppedname">opcode2</td>
<td colspan="2"></td>
<td></td>
<td colspan="3"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the PACIB variant

<span id="PACIB_64P_dp_1src"></span> Applies when <span class="bitdiff"> (Z == 0)</span>

PACIB [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\|SP\>](#XnSP_option__7 "Is the 64-bit name of the general-purpose source register or stack pointer, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the PACIZB variant

<span id="PACIZB_64Z_dp_1src"></span> Applies when <span class="bitdiff"> (Z == 1 && Rn == 11111)</span>

PACIZB [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; var source_is_sp : boolean = FALSE; let pacib1716 : boolean = FALSE; let d : integer = UInt(Rd); let n : integer = UInt(Rn); if Z == '0' then // PACIB if n == 31 then source_is_sp = TRUE; end; else // PACIZB if n != 31 then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; end;

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_PAuth) </span>

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
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="l">0</td>
<td>0</td>
<td>x</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td class="r">x</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="20"></td>
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3" class="droppedname">op2</td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the PACIB1716 variant

<span id="PACIB1716_HI_hints"></span> Applies when <span class="bitdiff"> (CRm == 0001 && op2 == 010)</span>

PACIB1716

</div>

<div class="encoding">

#### Encoding for the PACIBSP variant

<span id="PACIBSP_HI_hints"></span> Applies when <span class="bitdiff"> (CRm == 0011 && op2 == 011)</span>

PACIBSP

</div>

<div class="encoding">

#### Encoding for the PACIBZ variant

<span id="PACIBZ_HI_hints"></span> Applies when <span class="bitdiff"> (CRm == 0011 && op2 == 010)</span>

PACIBZ

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; var d : integer; var n : integer; var source_is_sp : boolean = FALSE; var pacib1716 : boolean = FALSE; case CRm::op2 of when '0011 010' =\> // PACIBZ d = 30; n = 31; when '0011 011' =\> // PACIBSP d = 30; source_is_sp = TRUE; if IsFeatureImplemented(FEAT_BTI) then // Check for branch target compatibility between PSTATE.BTYPE // and implicit branch target of PACIBSP instruction. let pacinst : [PACInstType](shared_pseudocode.html#type_PACInstType) = [PACIxSP](shared_pseudocode.html#enum_PACIxSP); [SetBTypeCompatible](shared_pseudocode.html#func_SetBTypeCompatible_1)([BTypeCompatible_PAC](shared_pseudocode.html#func_BTypeCompatible_PAC_1)(pacinst)); end; when '0001 010' =\> // PACIB1716 d = 17; n = 16; pacib1716 = TRUE; end;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xd&gt;</td>
<td><span id="XdOrXZR__6"></span>
<p>Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option__7"></span>
<p>Is the 64-bit name of the general-purpose source register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

if source_is_sp then if IsFeatureImplemented(FEAT_PAuth_LR) && [PSTATE](shared_pseudocode.html#global_PSTATE).PACM == '1' then [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACIB2](shared_pseudocode.html#func_AddPACIB2_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), [PC64](shared_pseudocode.html#func_PC64_0)()); else [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACIB](shared_pseudocode.html#func_AddPACIB_2)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}()); end; else if IsFeatureImplemented(FEAT_PAuth_LR) && [PSTATE](shared_pseudocode.html#global_PSTATE).PACM == '1' && pacib1716 then [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACIB2](shared_pseudocode.html#func_AddPACIB2_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [X](shared_pseudocode.html#accessor_X_2){64}(n), [X](shared_pseudocode.html#accessor_X_2){64}(15)); else [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AddPACIB](shared_pseudocode.html#func_AddPACIB_2)([X](shared_pseudocode.html#accessor_X_2){64}(d), [X](shared_pseudocode.html#accessor_X_2){64}(n)); end; end;

</div>
