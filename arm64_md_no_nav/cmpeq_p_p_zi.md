## CMP\<cc\> (immediate)

Compare vector to immediate

This instruction compares active integer elements in the source vector with an immediate, and places the boolean results of the specified comparison in the corresponding elements of the destination predicate. Inactive elements in the destination predicate register are set to zero. This instruction sets the First (N), None (Z), and !Last (C) condition flags based on the predicate result, and sets the V flag to zero.

| \<cc\> | Comparison                   |
|--------|------------------------------|
| EQ     | equal                        |
| GE     | signed greater than or equal |
| GT     | signed greater than          |
| HI     | unsigned higher than         |
| HS     | unsigned higher than or same |
| LE     | signed less than or equal    |
| LO     | unsigned lower than          |
| LS     | unsigned lower than or same  |
| LT     | signed less than             |
| NE     | not equal                    |

It has encodings from 10 classes: [Equal](#iclass_equal) , [Greater than](#iclass_greater_than) , [Greater than or equal](#iclass_greater_than_or_equal) , [Higher](#iclass_higher) , [Higher or same](#iclass_higher_or_same) , [Less than](#iclass_less_than) , [Less than or equal](#iclass_less_than_or_equal) , [Lower](#iclass_lower) , [Lower or same](#iclass_lower_or_same) and [Not equal](#iclass_not_equal)

### <span id="iclass_equal"></span>Equal<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">o2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmpeq_p_p_zi_"></span>

CMPEQ [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__43 "For the "Equal", "Greater than or equal", "Greater than", "Less than or equal", "Less than", and "Not equal" variants: is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_EQ](shared_pseudocode.html#enum_Cmp_EQ); let imm : integer = SInt(imm5); let unsigned : boolean = FALSE;

### <span id="iclass_greater_than"></span>Greater than<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">lt</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmpgt_p_p_zi_"></span>

CMPGT [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__43 "For the "Equal", "Greater than or equal", "Greater than", "Less than or equal", "Less than", and "Not equal" variants: is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT); let imm : integer = SInt(imm5); let unsigned : boolean = FALSE;

### <span id="iclass_greater_than_or_equal"></span>Greater than or equal<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">lt</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmpge_p_p_zi_"></span>

CMPGE [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__43 "For the "Equal", "Greater than or equal", "Greater than", "Less than or equal", "Less than", and "Not equal" variants: is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE); let imm : integer = SInt(imm5); let unsigned : boolean = FALSE;

### <span id="iclass_higher"></span>Higher<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td colspan="7" class="lr">imm7</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">lt</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmphi_p_p_zi_"></span>

CMPHI [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__44 "For the "Higher or same", "Higher", "Lower or same", and "Lower" variants: is the unsigned immediate operand, in the range 0 to 127, encoded in the "imm7" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT); let imm : integer = UInt(imm7); let unsigned : boolean = TRUE;

### <span id="iclass_higher_or_same"></span>Higher or same<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td colspan="7" class="lr">imm7</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">lt</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmphs_p_p_zi_"></span>

CMPHS [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__44 "For the "Higher or same", "Higher", "Lower or same", and "Lower" variants: is the unsigned immediate operand, in the range 0 to 127, encoded in the "imm7" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE); let imm : integer = UInt(imm7); let unsigned : boolean = TRUE;

### <span id="iclass_less_than"></span>Less than<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">lt</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmplt_p_p_zi_"></span>

CMPLT [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__43 "For the "Equal", "Greater than or equal", "Greater than", "Less than or equal", "Less than", and "Not equal" variants: is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_LT](shared_pseudocode.html#enum_Cmp_LT); let imm : integer = SInt(imm5); let unsigned : boolean = FALSE;

### <span id="iclass_less_than_or_equal"></span>Less than or equal<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">lt</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmple_p_p_zi_"></span>

CMPLE [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__43 "For the "Equal", "Greater than or equal", "Greater than", "Less than or equal", "Less than", and "Not equal" variants: is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_LE](shared_pseudocode.html#enum_Cmp_LE); let imm : integer = SInt(imm5); let unsigned : boolean = FALSE;

### <span id="iclass_lower"></span>Lower<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td colspan="7" class="lr">imm7</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">0</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">lt</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmplo_p_p_zi_"></span>

CMPLO [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__44 "For the "Higher or same", "Higher", "Lower or same", and "Lower" variants: is the unsigned immediate operand, in the range 0 to 127, encoded in the "imm7" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_LT](shared_pseudocode.html#enum_Cmp_LT); let imm : integer = UInt(imm7); let unsigned : boolean = TRUE;

### <span id="iclass_lower_or_same"></span>Lower or same<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">0</td>
<td colspan="2" class="lr">size</td>
<td class="lr">1</td>
<td colspan="7" class="lr">imm7</td>
<td class="lr">1</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="7"></td>
<td class="droppedname">lt</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmpls_p_p_zi_"></span>

CMPLS [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__44 "For the "Higher or same", "Higher", "Lower or same", and "Lower" variants: is the unsigned immediate operand, in the range 0 to 127, encoded in the "imm7" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_LE](shared_pseudocode.html#enum_Cmp_LE); let imm : integer = UInt(imm7); let unsigned : boolean = TRUE;

### <span id="iclass_not_equal"></span>Not equal<span style="font-size:smaller;">\
(FEAT_SVE \|\| FEAT_SME) </span>

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
<td>0</td>
<td class="r">1</td>
<td class="l">0</td>
<td>0</td>
<td>1</td>
<td class="r">0</td>
<td class="lr">1</td>
<td colspan="2" class="lr">size</td>
<td class="lr">0</td>
<td colspan="5" class="lr">imm5</td>
<td class="lr">1</td>
<td class="lr">0</td>
<td class="lr">0</td>
<td colspan="3" class="lr">Pg</td>
<td colspan="5" class="lr">Zn</td>
<td class="lr">1</td>
<td colspan="4" class="lr">Pd</td>
</tr>
<tr class="secondrow">
<td colspan="3"></td>
<td colspan="4"></td>
<td></td>
<td colspan="2"></td>
<td></td>
<td colspan="5"></td>
<td class="droppedname">op</td>
<td></td>
<td class="droppedname">o2</td>
<td colspan="3"></td>
<td colspan="5"></td>
<td class="droppedname">ne</td>
<td colspan="4"></td>
</tr>
</tbody>
</table>

</div>

<div class="encoding">

#### Encoding

<span id="cmpne_p_p_zi_"></span>

CMPNE [\<Pd\>](#Pd "Is the name of the destination scalable predicate register, encoded in the "Pd" field.").[\<T\>](#T__23 "Is the size specifier, "), [\<Pg\>](#Pg "Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.")/Z, [\<Zn\>](#Zn "Is the name of the source scalable vector register, encoded in the "Zn" field.").[\<T\>](#T__23 "Is the size specifier, "), \#[\<imm\>](#imm__43 "For the "Equal", "Greater than or equal", "Greater than", "Less than or equal", "Less than", and "Not equal" variants: is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field.")

</div>

#### Decode for this encoding

if !IsFeatureImplemented(FEAT_SVE) && !IsFeatureImplemented(FEAT_SME) then [EndOfDecode](shared_pseudocode.html#func_EndOfDecode_1)([Decode_UNDEF](shared_pseudocode.html#enum_Decode_UNDEF)); end; let esize : integer{} = 8 \<\< UInt(size); let g : integer = UInt(Pg); let n : integer = UInt(Zn); let d : integer = UInt(Pd); let cmp_op : [CmpOp](shared_pseudocode.html#type_CmpOp) = [Cmp_NE](shared_pseudocode.html#enum_Cmp_NE); let imm : integer = SInt(imm5); let unsigned : boolean = FALSE;

<div class="encoding-notes">

</div>

### Assembler Symbols

<div class="explanations">

<table>
<tbody>
<tr>
<td>&lt;Pd&gt;</td>
<td><span id="Pd"></span>
<p>Is the name of the destination scalable predicate register, encoded in the "Pd" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;T&gt;</td>
<td><span id="T__23"></span>
<p>Is the size specifier, encoded in “size”:</p>
<table class="valuetable">
<thead>
<tr>
<th class="bitfield">size</th>
<th class="symbol">&lt;T&gt;</th>
</tr>
</thead>
<tbody>
<tr>
<td class="bitfield">00</td>
<td class="symbol">B</td>
</tr>
<tr>
<td class="bitfield">01</td>
<td class="symbol">H</td>
</tr>
<tr>
<td class="bitfield">10</td>
<td class="symbol">S</td>
</tr>
<tr>
<td class="bitfield">11</td>
<td class="symbol">D</td>
</tr>
</tbody>
</table></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Pg&gt;</td>
<td><span id="Pg"></span>
<p>Is the name of the governing scalable predicate register P0-P7, encoded in the "Pg" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;Zn&gt;</td>
<td><span id="Zn"></span>
<p>Is the name of the source scalable vector register, encoded in the "Zn" field.</p></td>
</tr>
</tbody>
</table>

<table>
<tbody>
<tr>
<td>&lt;imm&gt;</td>
<td><span id="imm__43"></span>
<p>For the "Equal", "Greater than or equal", "Greater than", "Less than or equal", "Less than", and "Not equal" variants: is the signed immediate operand, in the range -16 to 15, encoded in the "imm5" field.</p></td>
</tr>
<tr>
<td></td>
<td><span id="imm__44"></span>
<p>For the "Higher or same", "Higher", "Lower or same", and "Lower" variants: is the unsigned immediate operand, in the range 0 to 127, encoded in the "imm7" field.</p></td>
</tr>
</tbody>
</table>

</div>

<div class="syntax-notes">

</div>

<div class="ps">

### Operation

[CheckSVEEnabled](shared_pseudocode.html#func_CheckSVEEnabled_0)(); let VL : integer{} = [CurrentVL](shared_pseudocode.html#func_CurrentVL_0)(); let PL : integer{} = VL DIV 8; let elements : integer = VL DIV esize; let mask : bits(PL) = P{}(g); let operand1 : bits(VL) = if [AnyActiveElement](shared_pseudocode.html#func_AnyActiveElement_3){PL}(mask, esize) then [Z](shared_pseudocode.html#accessor_Z_2){VL}(n) else Zeros{VL}; var result : bits(PL); let psize : integer{} = esize DIV 8; for e = 0 to elements-1 do let op1elt : bits(esize) = operand1\[e\*:esize\]; let element1 : integer = if unsigned then UInt(op1elt) else SInt(op1elt); if [ActivePredicateElement](shared_pseudocode.html#func_ActivePredicateElement_4){PL}(mask, e, esize) then var cond : boolean; case cmp_op of when [Cmp_EQ](shared_pseudocode.html#enum_Cmp_EQ) =\> cond = element1 == imm; when [Cmp_NE](shared_pseudocode.html#enum_Cmp_NE) =\> cond = element1 != imm; when [Cmp_GE](shared_pseudocode.html#enum_Cmp_GE) =\> cond = element1 \>= imm; when [Cmp_LT](shared_pseudocode.html#enum_Cmp_LT) =\> cond = element1 \< imm; when [Cmp_GT](shared_pseudocode.html#enum_Cmp_GT) =\> cond = element1 \> imm; when [Cmp_LE](shared_pseudocode.html#enum_Cmp_LE) =\> cond = element1 \<= imm; end; let pbit : bit = if cond then '1' else '0'; result\[e\*:psize\] = ZeroExtend{psize}(pbit); else result\[e\*:psize\] = ZeroExtend{psize}('0'); end; end; PSTATE.\[N,Z,C,V\] = [PredTest](shared_pseudocode.html#func_PredTest_4){PL}(mask, result, esize); [P](shared_pseudocode.html#accessor_P_2){PL}(d) = result;

</div>

### Operational information

This instruction is a data-independent-time instruction as described in <span class="armarm-xref" data-linkend="ARMARM_BEICCDDAB3">About PSTATE.DIT</span>.

If FEAT_SME is implemented and the PE is in Streaming SVE mode, then any subsequent instruction which is dependent on the predicate register or NZCV condition flags written by this instruction might be significantly delayed.
