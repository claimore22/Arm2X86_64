## STGP

Store Allocation Tag and pair of registers

This instruction stores an Allocation Tag and two 64-bit doublewords to memory, from two registers. The address used for the store is calculated from the base register and an immediate signed offset scaled by the Tag Granule. The Allocation Tag is calculated from the Logical Address Tag in the base register.

This instruction generates an Unchecked access.

It has encodings from 3 classes: [Post-index](#iclass_post_index) , [Pre-index](#iclass_pre_index) and [Signed offset](#iclass_signed_offset)

### <span id="iclass_post_index"></span>Post-index<span style="font-size:smaller;">\
(FEAT_MTE) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="7" class="lr">simm7</td>
<td colspan="5" class="lr">Rt2</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td colspan="2"></td>
<td class="droppedname">L</td>
<td colspan="7"></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="STGP_64_ldstpair_post"></span>

STGP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.")\], \#[\<imm\>](#imm__13 "For the "Post-index" and "Pre-index" variants: is the signed immediate offset, a multiple of 16 in the range -1024 to 1008, encoded in the "simm7" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let offset : bits(64) = LSL(SignExtend{64}(simm7), [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE)); let writeback : boolean = TRUE; let postindex : boolean = TRUE;

### <span id="iclass_pre_index"></span>Pre-index<span style="font-size:smaller;">\
(FEAT_MTE) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">1</td>
<td class="lr">0</td>
<td colspan="7" class="lr">simm7</td>
<td colspan="5" class="lr">Rt2</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td colspan="2"></td>
<td class="droppedname">L</td>
<td colspan="7"></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="STGP_64_ldstpair_pre"></span>

STGP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."), \#[\<imm\>](#imm__13 "For the "Post-index" and "Pre-index" variants: is the signed immediate offset, a multiple of 16 in the range -1024 to 1008, encoded in the "simm7" field.")\]!

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let offset : bits(64) = LSL(SignExtend{64}(simm7), [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE)); let writeback : boolean = TRUE; let postindex : boolean = FALSE;

### <span id="iclass_signed_offset"></span>Signed offset<span style="font-size:smaller;">\
(FEAT_MTE) </span>

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
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="l">1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="7" class="lr">simm7</td>
<td colspan="5" class="lr">Rt2</td>
<td colspan="5" class="lr">Rn</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="2" class="droppedname">opc</td>
<td colspan="2"></td>
<td></td>
<td class="droppedname">VR</td>
<td></td>
<td colspan="2"></td>
<td class="droppedname">L</td>
<td colspan="7"></td>
<td colspan="5"></td>
<td colspan="5"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="STGP_64_ldstpair_off"></span>

STGP [\<Xt1\>](#Xt1OrXZR "Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field."), [\<Xt2\>](#Xt2OrXZR "Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field."), \[[\<Xn\|SP\>](#XnSP_option "Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field."){, \#[\<imm\>](#imm__8 "For the "Signed offset" variant: is the optional signed immediate offset, a multiple of 16 in the range -1024 to 1008, defaulting to 0 and encoded in the "simm7" field.")}\]

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_MTE) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let t : integer{} = UInt(Rt); let t2 : integer{} = UInt(Rt2); let n : integer{} = UInt(Rn); let offset : bits(64) = LSL(SignExtend{64}(simm7), [LOG2_TAG_GRANULE](shared_pseudocode.html#global_LOG2_TAG_GRANULE)); let writeback : boolean = FALSE; let postindex : boolean = FALSE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Xt1&gt;</td>
<td><span id="Xt1OrXZR"></span>
<p>Is the 64-bit name of the first general-purpose register to be transferred, encoded in the "Rt" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xt2&gt;</td>
<td><span id="Xt2OrXZR"></span>
<p>Is the 64-bit name of the second general-purpose register to be transferred, encoded in the "Rt2" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Xn|SP&gt;</td>
<td><span id="XnSP_option"></span>
<p>Is the 64-bit name of the general-purpose base register or stack pointer, encoded in the "Rn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__13"></span>
<p>For the "Post-index" and "Pre-index" variants: is the signed immediate offset, a multiple of 16 in the range -1024 to 1008, encoded in the "simm7" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__8"></span>
<p>For the "Signed offset" variant: is the optional signed immediate offset, a multiple of 16 in the range -1024 to 1008, defaulting to 0 and encoded in the "simm7" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

var address : bits(64); if n == 31 then [CheckSPAlignment](shared_pseudocode.html#func_CheckSPAlignment_0)(); address = [SP](shared_pseudocode.html#accessor_SP_1){64}(); else address = [X](shared_pseudocode.html#accessor_X_2){64}(n); end; let stzgm : boolean = FALSE; let ispair : boolean = TRUE; let accdesc : [AccessDescriptor](shared_pseudocode.html#type_AccessDescriptor) = [CreateAccDescLDGSTG](shared_pseudocode.html#func_CreateAccDescLDGSTG_5)([MemOp_STORE](shared_pseudocode.html#enum_MemOp_STORE), stzgm, ispair, t, t2); if !postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if !IsAlignedSize(address, [TAG_GRANULE](shared_pseudocode.html#global_TAG_GRANULE)) then let fault : [FaultRecord](shared_pseudocode.html#type_FaultRecord) = [AlignmentFault](shared_pseudocode.html#func_AlignmentFault_2)(accdesc, address); [AArch64_Abort](shared_pseudocode.html#func_AArch64_Abort_1)(fault); end; let data : bits(128) = (if [BigEndian](shared_pseudocode.html#func_BigEndian_1)(accdesc.acctype) then [X](shared_pseudocode.html#accessor_X_2){64}(t) ::[X](shared_pseudocode.html#accessor_X_2){64}(t2) else [X](shared_pseudocode.html#accessor_X_2){64}(t2)::[X](shared_pseudocode.html#accessor_X_2){64}(t)); [Mem](shared_pseudocode.html#accessor_Mem_3){128}(address, accdesc) = data; [AArch64_MemTag](shared_pseudocode.html#accessor_AArch64_MemTag_2)(address, accdesc) = [AArch64_AllocationTagFromAddress](shared_pseudocode.html#func_AArch64_AllocationTagFromAddress_1)(address); if writeback then if postindex then address = [AddressAdd](shared_pseudocode.html#func_AddressAdd_3)(address, offset, accdesc); end; if n == 31 then [SP](shared_pseudocode.html#accessor_SP_1){64}() = address; else [X](shared_pseudocode.html#accessor_X_2){64}(n) = address; end; end;

</div>
