## CB\<cc\> (immediate)

Compare register with immediate and branch

This instruction compares the value in a register with an immediate, and conditionally branches to a label at a PC-relative offset if the comparison is true. This instruction provides a hint that this is not a subroutine call or return. This instruction does not affect the condition flags.

This instruction is used by the pseudo-instructions [CBGE (immediate)](cbge_imm.html "Compare signed greater than or equal to immediate and branch"), [CBHS (immediate)](cbhs_imm.html "Compare unsigned higher than or same as immediate and branch"), [CBLE (immediate)](cble_imm.html "Compare signed less than or equal to immediate and branch"), and [CBLS (immediate)](cbls_imm.html "Compare unsigned lower than or same as immediate and branch").

### <span id="iclass_branch"></span>Branch<span style="font-size:smaller;">\
(FEAT_CMPBR) </span>

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
<td class="lr">sf</td>
<td class="l">1</td>
<td>1</td>
<td>1</td>
<td>0</td>
<td>1</td>
<td>0</td>
<td class="r">1</td>
<td colspan="3" class="lr">cc</td>
<td colspan="6" class="lr">imm6</td>
<td class="lr">0</td>
<td colspan="9" class="lr">imm9</td>
<td colspan="5" class="lr">Rt</td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding for the 32-bit greater than variant

<span id="CBGT_32_imm"></span> Applies when <span class="bitdiff"> (sf == 0 && cc == 000)</span>

CBGT [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 32-bit less than variant

<span id="CBLT_32_imm"></span> Applies when <span class="bitdiff"> (sf == 0 && cc == 001)</span>

CBLT [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 32-bit higher variant

<span id="CBHI_32_imm"></span> Applies when <span class="bitdiff"> (sf == 0 && cc == 010)</span>

CBHI [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 32-bit lower variant

<span id="CBLO_32_imm"></span> Applies when <span class="bitdiff"> (sf == 0 && cc == 011)</span>

CBLO [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 32-bit equal variant

<span id="CBEQ_32_imm"></span> Applies when <span class="bitdiff"> (sf == 0 && cc == 110)</span>

CBEQ [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 32-bit not equal variant

<span id="CBNE_32_imm"></span> Applies when <span class="bitdiff"> (sf == 0 && cc == 111)</span>

CBNE [\<Wt\>](#WtOrWZR "Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 64-bit greater than variant

<span id="CBGT_64_imm"></span> Applies when <span class="bitdiff"> (sf == 1 && cc == 000)</span>

CBGT [\<Xt\>](#XtOrXZR "Is the 64-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 64-bit less than variant

<span id="CBLT_64_imm"></span> Applies when <span class="bitdiff"> (sf == 1 && cc == 001)</span>

CBLT [\<Xt\>](#XtOrXZR "Is the 64-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 64-bit higher variant

<span id="CBHI_64_imm"></span> Applies when <span class="bitdiff"> (sf == 1 && cc == 010)</span>

CBHI [\<Xt\>](#XtOrXZR "Is the 64-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 64-bit lower variant

<span id="CBLO_64_imm"></span> Applies when <span class="bitdiff"> (sf == 1 && cc == 011)</span>

CBLO [\<Xt\>](#XtOrXZR "Is the 64-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 64-bit equal variant

<span id="CBEQ_64_imm"></span> Applies when <span class="bitdiff"> (sf == 1 && cc == 110)</span>

CBEQ [\<Xt\>](#XtOrXZR "Is the 64-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

<div class="encoding">

#### Encoding for the 64-bit not equal variant

<span id="CBNE_64_imm"></span> Applies when <span class="bitdiff"> (sf == 1 && cc == 111)</span>

CBNE [\<Xt\>](#XtOrXZR "Is the 64-bit name of the general-purpose register to be tested, encoded in the "Rt" field."), \#[\<imm\>](#imm_cbr "Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field."), [\<label\>](#imm9_offset "Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.")

</div>

#### Decode for all variants of this encoding

if !IsFeatureImplemented(FEAT_CMPBR) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let datasize : integer{} = 32 \<\< UInt(sf); let t : integer = UInt(Rt); let offset : bits(64) = SignExtend{}(imm9::'00'); var op : [CmpOp](shared_pseudocode.html#type_CmpOp); var unsigned : boolean; case cc of when '000' =\> op = [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT); unsigned = FALSE; when '001' =\> op = [Cmp_LT](shared_pseudocode.html#enum_Cmp_LT); unsigned = FALSE; when '010' =\> op = [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT); unsigned = TRUE; when '011' =\> op = [Cmp_LT](shared_pseudocode.html#enum_Cmp_LT); unsigned = TRUE; when '110' =\> op = [Cmp_EQ](shared_pseudocode.html#enum_Cmp_EQ); unsigned = TRUE; when '111' =\> op = [Cmp_NE](shared_pseudocode.html#enum_Cmp_NE); unsigned = TRUE; otherwise =\> [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let value2 : integer = UInt(imm6);

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Wt&gt;</td>
<td><span id="WtOrWZR"></span>
<p>Is the 32-bit name of the general-purpose register to be tested, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm_cbr"></span>
<p>Is an unsigned immediate, in the range 0 to 63, encoded in the "imm6" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;label&gt;</td>
<td><span id="imm9_offset"></span>
<p>Is the program label to be conditionally branched to. Its offset from the address of this instruction, in the range -1024 to 1020, is encoded as "imm9" times 4.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt&gt;</td>
<td><span id="XtOrXZR"></span>
<p>Is the 64-bit name of the general-purpose register to be tested, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

let operand1 : bits(datasize) = X{}(t); let branch_conditional : boolean = TRUE; let value1 : integer = if unsigned then UInt(operand1) else SInt(operand1); var cond : boolean; case op of when [Cmp_EQ](shared_pseudocode.html#enum_Cmp_EQ) =\> cond = value1 == value2; when [Cmp_NE](shared_pseudocode.html#enum_Cmp_NE) =\> cond = value1 != value2; when [Cmp_LT](shared_pseudocode.html#enum_Cmp_LT) =\> cond = value1 \< value2; when [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT) =\> cond = value1 \> value2; end; if cond then [BranchTo](shared_pseudocode.html#func_BranchTo_4){64}([PC64](shared_pseudocode.html#func_PC64_0)() + offset, [BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); else [BranchNotTaken](shared_pseudocode.html#func_BranchNotTaken_2)([BranchType_DIR](shared_pseudocode.html#enum_BranchType_DIR), branch_conditional); end;

</div>
