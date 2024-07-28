
# Tantivy - unexpected merge policy demo

This repository contains demo of unexpected bahaviour of Tantivy's merge policy described in the issue: https://github.com/quickwit-oss/tantivy/issues/2454

## Experimental results (against proposed fix)

The following results were run on a `release` profile build with M1 Max / 64GB to index `1000` randomly generated documents:

|Run|Commit|Merge policy|Wait for merge threads|Time|Segment counts|`compute_merge_candidates`|
|-|-|-|-|-|-|-|
|A|Single|MergeWhenever|No|`257ms`|`.fast: 4x`<br>`.fieldnorm: 4x`<br>`.idx: 4x`<br>`.pos: 4x`<br>`.store: 4x`<br>`.term: 4x`|**Calls: 8x**<br>`0 args: 7x`<br>`2 args: 1x`|
|B|Single|MergeWhenever|Yes|`454ms`|`.fast: 1x`<br>`.fieldnorm: 1x`<br>`.idx: 1x`<br>`.pos: 1x`<br>`.store: 1x`<br>`.term: 1x`|**Calls: 12x**<br>`0 args: 10x`<br>`2 args: 2x`|
|C|Single|TargetDocs|No|`246ms`|`.fast: 4x`<br>`.fieldnorm: 4x`<br>`.idx: 4x`<br>`.pos: 4x`<br>`.store: 4x`<br>`.term: 4x`|**Calls: 8x**<br>`0 args: 7x`<br>`1 args: 1x`|
|D|Single|TargetDocs|Yes|`472ms`|`.fast: 1x`<br>`.fieldnorm: 1x`<br>`.idx: 1x`<br>`.pos: 1x`<br>`.store: 1x`<br>`.term: 1x`|**Calls: 12x**<br>`0 args: 10x`<br>`2 args: 2x`|
|E|After every change|MergeWhenever|No|`198s`|`.fast: 5x`<br>`.fieldnorm: 5x`<br>`.idx: 5x`<br>`.pos: 5x`<br>`.store: 5x`<br>`.term: 5x`|**Calls: 5992x**<br>`0 args: 4994x`<br>`2 args: 998x`|
|F|After every change|MergeWhenever|Yes|`196s`|`.fast: 1x`<br>`.fieldnorm: 1x`<br>`.idx: 1x`<br>`.pos: 1x`<br>`.store: 1x`<br>`.term: 1x`|**Calls: 5998x**<br>`0 args: 4999x`<br>`2 args: 999x`|
|G|After every change|TargetDocs|No|`194s`|`.fast: 5x`<br>`.fieldnorm: 5x`<br>`.idx: 5x`<br>`.pos: 5x`<br>`.store: 5x`<br>`.term: 5x`|**Calls: 5992x**<br>`0 args: 4994x`<br>`2 args: 998x`|
|H|After every change|TargetDocs|Yes|`185s`|`.fast: 1x`<br>`.fieldnorm: 1x`<br>`.idx: 1x`<br>`.pos: 1x`<br>`.store: 1x`<br>`.term: 1x`|**Calls: 5998x**<br>`0 args: 4999x`<br>`2 args: 999x`|

All the runs finished successfully, but more importantly - finished. No potentially infinite loop was detected and (unlike with the unfixed version) every runs called the merge policy with 2 candidated at least once.

## Experimental results

The following results were run on a `release` profile build with M1 Max / 64GB to index `1000` randomly generated documents:

|Run|Commit|Merge policy|Wait for merge threads|Time|Segment counts|`compute_merge_candidates`|
|-|-|-|-|-|-|-|
|A|Single|MergeWhenever|No|`245ms`|`.fast: 4x`<br>`.fieldnorm: 4x`<br>`.idx: 4x`<br>`.pos: 4x`<br>`.store: 4x`<br>`.term: 4x`|**Calls: 8x**<br>`0 args: 4x`<br>`1 arg: 3x`<br>`2 args: 1x`|
|B|Single|MergeWhenever|Yes|`488ms`|`.fast: 1x`<br>`.fieldnorm: 1x`<br>`.idx: 1x`<br>`.pos: 1x`<br>`.store: 1x`<br>`.term: 1x`|**Calls: 12x**<br>`0 args: 6x`<br>`1 arg: 4x`<br>`2 args: 2x`|
|C|Single|TargetDocs|No|`377ms`|`.fast: 6x`<br>`.fieldnorm: 5x`<br>`.idx: 5x`<br>`.pos: 5x`<br>`.store: 6x`<br>`.term: 5x`|**Calls: 10x**<br>`0 args: 6x`<br>`1 arg: 4x`|
|D|Single|TargetDocs|Yes|**Inf. loop**|`???`|**Calls: >63466x**<br>`0 args: >31734x`<br>`1 arg: >31732x`|
|E|After every change|MergeWhenever|No|`198s`|`.fast: 5x`<br>`.fieldnorm: 5x`<br>`.idx: 5x`<br>`.pos: 5x`<br>`.store: 5x`<br>`.term: 5x`|**Calls: 5992x**<br>`0 args: 2282x`<br>`1 arg: 2712x`<br>`2 args: 998x`|
|F|After every change|MergeWhenever|Yes|`211s`|`.fast: 1x`<br>`.fieldnorm: 1x`<br>`.idx: 1x`<br>`.pos: 1x`<br>`.store: 1x`<br>`.term: 1x`|**Calls: 5998x**<br>`0 args: 2273x`<br>`1 arg: 2726x`<br>`2 args: 999x`|
|G|After every change|TargetDocs|No|`575s`|`.fast: 1004x`<br>`.fieldnorm: 1003x`<br>`.idx: 1003x`<br>`.pos: 1002x`<br>`.store: 1004x`<br>`.term: 1003x`|**Calls: 14548x**<br>`0 args: 8274x`<br>`1 arg: 6274x`|
|H|After every change|TargetDocs|Yes|**Inf. loop**|`???`|**Calls: >62218x**<br>`0 args: >32109x`<br>`1 arg: >30109x`|

## Observations

- Runs `D` and `H` didn't actually finish, after 45-50min I have manually terminated them
- Both runs `D` and `H` share 2 settings - both **use the `TargetDocs` merge policy** and both of them **wait for merging threads**
- When the `TargetDocs` is used, then the `compute_merge_candidates` is never invoked with more then 1 single merge candidate - regardless of other settings (# of commits or waiting for merging threads)
- The `TargetDocs` merge policy is slightly computationally/memory heavier then the very simple `MergeWhenever` merge policy

## Conclusion

See [@PSeitz](https://github.com/PSeitz)'s comment (https://github.com/quickwit-oss/tantivy/issues/2454#issuecomment-2246852126) and PR (https://github.com/quickwit-oss/tantivy/pull/2457) for the actual explanation and proposed fix.

~**Race condition.**~

~Well...~
- ~When the merge policy is "heavier" above _some_ threshold, then a race condition takes place with some internal Tantivy prodecure~
- ~This race condition somehow causes `compute_merge_candidates` never to be passed more then 1 merge candidate~
- ~**Waiting for merging threads in combination with this race condition causes the program to be stuck in a _(possibly)_ infinite loop**~
