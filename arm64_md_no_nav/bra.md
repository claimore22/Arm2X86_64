## BRAA, BRAAZ, BRAB, BRABZ

Branch to register, with pointer authentication

This instruction authenticates the address in the general-purpose register that is specified by \<Xn\>, using a modifier and the specified key, and branches to the authenticated address.

The modifier is:

- In the general-purpose register or stack pointer that is specified by \<Xm\|SP\>, for <span class="asm-code">BRAA</span> and <span class="asm-code">BRAB</span>.
- The value zero, for <span class="asm-code">BRAAZ</span> and <span class="asm-code">BRABZ</span>.

Key A is used for <span class="asm-code">BRAA</span> and <span class="asm-code">BRAAZ</span>. Key B is used for <span class="asm-code">BRAB</span> and <span class="asm-code">BRABZ</span>.

If the authentication passes, the PE continues execution at the target of the branch. For information on behavior if the authentication fails, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.Faulting_on_pointer_authentication">Faulting on pointer authentication</span>.

The authenticated address is not written back to the general-purpose register.

This instruction provides a hint that this is not a subroutine call or return.

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
<td class="l">1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td class="r">1</td>
<td class="lr">Z</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">0</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">M</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rm</td>
</tr>
<tr class="secondrow">
<td colspan="7"></td>
<td></td>
<td></td>
<td colspan="2" class="droppedname">op</td>
<td colspan="5" class="droppedname">op2</td>
<td colspan="4"></td>
<td class="droppedname">A</td>
<td></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the Key A, register modifier variant

<span id="BRAA_64P_branch_reg"></span> Applies when <span class="bitdiff"> (Z == 1 && M == 0)</span>

BRAA [\<Xn\>](#XnOrXZR "Is the 64-bit name of the general-purpose register holding the address to be branched to, encoded in the "Rn" field."), [\<Xm\|SP\>](#XmSP_option "Is the 64-bit name of the general-purpose source register or stack pointer holding the modifier, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the Key A, zero modifier variant

<span id="BRAAZ_64_branch_reg"></span> Applies when <span class="bitdiff"> (Z == 0 && M == 0 && Rm == 11111)</span>

BRAAZ [\<Xn\>](#XnOrXZR "Is the 64-bit name of the general-purpose register holding the address to be branched to, encoded in the "Rn" field.")

</div>

<div class="encoding">

#### Encoding for the Key B, register modifier variant

<span id="BRAB_64P_branch_reg"></span> Applies when <span class="bitdiff"> (Z == 1 && M == 1)</span>

BRAB [\<Xn\>](#XnOrXZR "Is the 64-bit name of the general-purpose register holding the address to be branched to, encoded in the "Rn" field."), [\<Xm\|SP\>](#XmSP_option "Is the 64-bit name of the general-purpose source register or stack pointer holding the modifier, encoded in the "Rm" field.")

</div>

<div class="encoding">

#### Encoding for the Key B, zero modifier variant

<span id="BRABZ_64_branch_reg"></span> Applies when <span class="bitdiff"> (Z == 0 && M == 1 && Rm == 11111)</span>

BRABZ [\<Xn\>](#XnOrXZR "Is the 64-bit name of the general-purpose register holding the address to be branched to, encoded in the "Rn" field.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; if Z == '0' && Rm != '11111' then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let n : integer{} = UInt(Rn); let m : integer{} = UInt(Rm); let use_key_a : boolean = (M == '0'); let source_is_sp : boolean = ((Z == '1') && (m == 31)); let auth_then_branch : boolean = TRUE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xn&gt;</td>
<td><span id="XnOrXZR"></span>
<p>Is the 64-bit name of the general-purpose register holding the address to be branched to, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xm|SP&gt;</td>
<td><span id="XmSP_option"></span>
<p>Is the 64-bit name of the general-purpose source register or stack pointer holding the modifier, encoded in the "Rm" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var target : bits(64) = X{}(n); let modifier : bits(64) = if source_is_sp then [SP](shared_pseudocode.html#accessor_SP_1){64}() else [X](shared_pseudocode.html#accessor_X_2){64}(m); if use_key_a then target = [AuthIA](shared_pseudocode.html#func_AuthIA_3)(target, modifier, auth_then_branch); else target = [AuthIB](shared_pseudocode.html#func_AuthIB_3)(target, modifier, auth_then_branch); end; // Value in BTypeNext will be used to set PSTATE.BTYPE if [InGuardedPage](shared_pseudocode.html#global_InGuardedPage) then if n == 16 \|\| n == 17 then [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '01'; else [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '11'; end; else [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '01'; end; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}(target, [BranchType_INDIR](shared_pseudocode.html#enum_BranchType_INDIR), branch_conditional);

</div>
