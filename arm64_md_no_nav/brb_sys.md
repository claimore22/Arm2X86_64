## BRB

Branch record buffer

For more information, see <span class="armarm-xref" data-linkend="ARMARM_BABEJJJE">op0 == 0b01, cache maintenance, TLB maintenance, address translation, prediction restriction, BRBE, Trace Extension, and Guarded Control Stack instructions</span>.

This is an alias of [SYS](sys.html). This means:

- The encodings in this description are named to match the encodings of [SYS](sys.html).
- The description of [SYS](sys.html) gives the operational pseudocode, any <span class="arm-defined-word">constrained unpredictable</span> behavior, and any operational information for this instruction.

### <span id="iclass_system"></span>System<span style="font-size:smaller;">\
(FEAT_BRBE) </span>

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
<td class="r">0</td>
<td class="l">1</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>0</td>
<td class="r">0</td>
<td class="lr">0</td>
<td class="l">0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>1</td>
<td>1</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td colspan="3" class="lr">op2</td>
<td colspan="5" class="lr">Rt</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="3"></td>
<td colspan="4"></td>
<td class="droppedname">L</td>
<td colspan="2"></td>
<td colspan="3" class="droppedname">op1</td>
<td colspan="4" class="droppedname">CRn</td>
<td colspan="4" class="droppedname">CRm</td>
<td colspan="3"></td>
<td colspan="5"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="BRB_SYS_CR_systeminstrs"></span>

BRB [\<brb_op\>](#brb_op_option "Is a BRB operation name, as listed for the BRB system instruction group, ")

is equivalent to

[SYS](sys.html#SYS_CR_systeminstrs) \#1, C7, C2, \#[\<op2\>](sys.html#op2){, [\<Xt\>](sys.html#XtOrXZR__2)}

and is the preferred disassembly when <span class="pseudocode">SysOp('001', '0111', '0010', op2) == Sys_BRB</span>.

</div>

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;brb_op&gt;</td>
<td><span id="brb_op_option"></span>
<p>Is a BRB operation name, as listed for the BRB system instruction group, encoded in “op2”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">op2</th>
<th class="symbol">&lt;brb_op&gt;</th>
<th class="symbol">Architectural Feature</th>
<th class="symbol">Description</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">100</td>
<td class="symbol">IALL</td>
<td class="feature">FEAT_BRBE</td>
<td class="description"><p>For more information, see BRB IALL.</p></td>
</tr>
<tr>
<td class="bitfield">101</td>
<td class="symbol">INJ</td>
<td class="feature">FEAT_BRBE</td>
<td class="description"><p>For more information, see BRB INJ.</p></td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="alias_ps_section">

### Operation

The description of [SYS](sys.html) gives the operational pseudocode for this instruction.

</div>
