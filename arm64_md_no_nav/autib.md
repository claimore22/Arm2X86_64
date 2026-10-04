## AUTIB, AUTIB1716, AUTIBSP, AUTIBZ, AUTIZB

Authenticate instruction address, using key B

This instruction authenticates an instruction address, using a modifier and key B.

If the authentication passes, the upper bits of the address are restored to enable subsequent use of the address. For information on behavior if the authentication fails, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.Faulting_on_pointer_authentication">Faulting on pointer authentication</span>.

The address is:

- In the general-purpose register that is specified by \<Xd\> for <span class="asm-code">AUTIB</span> and <span class="asm-code">AUTIZB</span>.
- In X17, for <span class="asm-code">AUTIB1716</span>.
- In X30, for <span class="asm-code">AUTIBSP</span> and <span class="asm-code">AUTIBZ</span>.

The modifier is:

- In the general-purpose register or stack pointer that is specified by \<Xn\|SP\> for <span class="asm-code">AUTIB</span>.
- The value zero, for <span class="asm-code">AUTIZB</span> and <span class="asm-code">AUTIBZ</span>.
- In X16, for <span class="asm-code">AUTIB1716</span>.
- In SP, for <span class="asm-code">AUTIBSP</span>.

If <span class="armarm-xref" data-linkend="ARMARM_FEAT_PAuth_LR">FEAT_PAuth_LR</span> is implemented and PSTATE.PACM is 1, then <span class="asm-code">AUTIB1716</span> and <span class="asm-code">AUTIBSP</span> include a second modifier that is:

- In X15, for <span class="asm-code">AUTIB1716</span>.
- In X16, for <span class="asm-code">AUTIBSP</span>.

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
<td class="l">1</td>
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

#### Encoding for the AUTIB variant

<span id="AUTIB_64P_dp_1src"></span> Applies when <span class="bitdiff"> (Z == 0)</span>

AUTIB [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field."), [\<Xn\|SP\>](#XnSP_option__7 "Is the 64-bit name of the general-purpose source register or stack pointer, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the AUTIZB variant

<span id="AUTIZB_64Z_dp_1src"></span> Applies when <span class="bitdiff"> (Z == 1 && Rn == 11111)</span>

AUTIZB [\<Xd\>](#XdOrXZR__6 "Is the 64-bit name of the general-purpose destination register, encoded in the "Rd" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Z == '1' && Rn != '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let autib1716 : boolean = FALSE; let d : integer = UInt(Rd); let n : integer = UInt(Rn); let auth_combined : boolean = FALSE; let source_is_sp : boolean = Z == '0' && n == 31;

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
<td class="l">1</td>
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

#### Encoding for the AUTIB1716 variant

<span id="AUTIB1716_HI_hints"></span> Applies when <span class="bitdiff"> (CRm == 0001 && op2 == 110)</span>

AUTIB1716

</div>

<div class="encoding">

#### Encoding for the AUTIBSP variant

<span id="AUTIBSP_HI_hints"></span> Applies when <span class="bitdiff"> (CRm == 0011 && op2 == 111)</span>

AUTIBSP

</div>

<div class="encoding">

#### Encoding for the AUTIBZ variant

<span id="AUTIBZ_HI_hints"></span> Applies when <span class="bitdiff"> (CRm == 0011 && op2 == 110)</span>

AUTIBZ

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_NOP](shared_pseudocode.html#enum_Decode_NOP)); end; var d : integer; var n : integer; var source_is_sp : boolean = FALSE; var autib1716 : boolean = FALSE; let auth_combined : boolean = FALSE; case CRm::op2 of when '0011 110' =\> // AUTIBZ d = 30; n = 31; when '0011 111' =\> // AUTIBSP d = 30; source_is_sp = TRUE; when '0001 110' =\> // AUTIB1716 d = 17; n = 16; autib1716 = TRUE; end;

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

if source_is_sp then if IsFeatureImplemented(FEAT_PAuth_LR) && [PSTATE](shared_pseudocode.html#global_PSTATE).PACM == '1' then [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthIB2](shared_pseudocode.html#func_AuthIB2_4)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), [X](shared_pseudocode.html#accessor_X_2){64}(16), auth_combined); else [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthIB](shared_pseudocode.html#func_AuthIB_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [SP](shared_pseudocode.html#accessor_SP_1){64}(), auth_combined); end; else if IsFeatureImplemented(FEAT_PAuth_LR) && [PSTATE](shared_pseudocode.html#global_PSTATE).PACM == '1' && autib1716 then [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthIB2](shared_pseudocode.html#func_AuthIB2_4)([X](shared_pseudocode.html#accessor_X_2){64}(d), [X](shared_pseudocode.html#accessor_X_2){64}(n), [X](shared_pseudocode.html#accessor_X_2){64}(15), auth_combined); else [X](shared_pseudocode.html#accessor_X_2){64}(d) = [AuthIB](shared_pseudocode.html#func_AuthIB_3)([X](shared_pseudocode.html#accessor_X_2){64}(d), [X](shared_pseudocode.html#accessor_X_2){64}(n), auth_combined); end; end;

</div>
