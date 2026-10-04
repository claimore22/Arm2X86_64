## RETAA, RETAB

Return from subroutine, with pointer authentication

This instruction authenticates the address that is held in LR, using SP as the modifier and the specified key, and branches to the authenticated address.

Key A is used for <span class="asm-code">RETAA</span>. Key B is used for <span class="asm-code">RETAB</span>.

If the authentication passes, the PE continues execution at the target of the branch. For information on behavior if the authentication fails, see <span class="armarm-xref" data-linkend="ARMARM_MDSec.Faulting_on_pointer_authentication">Faulting on pointer authentication</span>.

The authenticated address is not written back to LR.

This instruction provides a hint that this is a subroutine return.

If <span class="armarm-xref" data-linkend="ARMARM_FEAT_PAuth_LR">FEAT_PAuth_LR</span> is implemented and PSTATE.PACM is 1, then <span class="asm-code">RETAA</span> and <span class="asm-code">RETAB</span> include a second modifier that is in X16.

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
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
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
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
</tr>
<tr class="secondrow">
<td colspan="7"></td>
<td class="droppedname">Z</td>
<td></td>
<td colspan="2" class="droppedname">op</td>
<td colspan="5" class="droppedname">op2</td>
<td colspan="4"></td>
<td class="droppedname">A</td>
<td></td>
<td colspan="5" class="droppedname">Rn</td>
<td colspan="5" class="droppedname">Rm</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the RETAA variant

<span id="RETAA_64E_branch_reg"></span> Applies when <span class="bitdiff"> (M == 0)</span>

RETAA

</div>

<div class="encoding">

#### Encoding for the RETAB variant

<span id="RETAB_64E_branch_reg"></span> Applies when <span class="bitdiff"> (M == 1)</span>

RETAB

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_PAuth) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let use_key_a : boolean = (M == '0'); let auth_then_branch : boolean = TRUE;

<div class="encoding-notes">

</div>

<div class="ps">

### Operation

var inst_type : [GCSInstruction](shared_pseudocode.html#type_GCSInstruction); var target : bits(64) = X{}(30); let modifier : bits(64) = SP{}(); var modifier2 : bits(64); var use_modifier2 : boolean = FALSE; if IsFeatureImplemented(FEAT_PAuth_LR) && [PSTATE](shared_pseudocode.html#global_PSTATE).PACM == '1' then modifier2 = [X](shared_pseudocode.html#accessor_X_2){64}(16); use_modifier2 = TRUE; end; if use_key_a then if use_modifier2 && IsFeatureImplemented(FEAT_PAuth_LR) then target = [AuthIA2](shared_pseudocode.html#func_AuthIA2_4)(target, modifier, modifier2, auth_then_branch); else target = [AuthIA](shared_pseudocode.html#func_AuthIA_3)(target, modifier, auth_then_branch); end; else if use_modifier2 && IsFeatureImplemented(FEAT_PAuth_LR) then target = [AuthIB2](shared_pseudocode.html#func_AuthIB2_4)(target, modifier, modifier2, auth_then_branch); else target = [AuthIB](shared_pseudocode.html#func_AuthIB_3)(target, modifier, auth_then_branch); end; end; if IsFeatureImplemented(FEAT_GCS) && [GCSPCREnabled](shared_pseudocode.html#func_GCSPCREnabled_1)([PSTATE](shared_pseudocode.html#global_PSTATE).EL) then inst_type = if use_key_a then [GCSInstType_PRETAA](shared_pseudocode.html#enum_GCSInstType_PRETAA) else [GCSInstType_PRETAB](shared_pseudocode.html#enum_GCSInstType_PRETAB); target = [LoadCheckGCSRecord](shared_pseudocode.html#func_LoadCheckGCSRecord_2)(target, inst_type); [SetCurrentGCSPointer](shared_pseudocode.html#func_SetCurrentGCSPointer_1)([GetCurrentGCSPointer](shared_pseudocode.html#func_GetCurrentGCSPointer_0)() + 8); end; // Value in BTypeNext will be used to set PSTATE.BTYPE [BTypeNext](shared_pseudocode.html#global_BTypeNext) = '00'; let branch_conditional : boolean = FALSE; [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}(target, [BranchType_RET](shared_pseudocode.html#enum_BranchType_RET), branch_conditional);

</div>
