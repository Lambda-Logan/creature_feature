# Benchmarks — old and new

The project has benchmarks in its [src/bench.rs](../src/bench.rs) file. These benchamrks have been copied and refactored into new, Criterion-ized style ([benches/bench_criterionized_runner.rs](../benches/bench_criterionized_runner.rs)). This uses Criterion's comparing (comparison group) facility (`benchmark_group`), augmented with the input parametricity (`bench_with_input`). This enables the generation of a well-demostrable comparison diagram of compared colored function graphs.

## Approaches for groupings of benchmarks

As a first apprach, we can put all benchmarks into one single big comparison group. Here below are shown the examples of the generated bechmark plottings, and these diagrams demonstrate what the problems with this first approach is. After several experimentations, here we can see the result diagrams of the various big experimentational runnings:

![All compared in one single group, example 1](sample-diagrams/comparison-2023-02-03--17h51m29s.png
![All compared in one single group, example 2](sample-diagrams/comparison-2023-02-04--18h20m22s.png
![All compared in one single group, example 3](sample-diagrams/comparison-2023-02-04--20h36m33s.png
![All compared in one single group, example 4](sample-diagrams/comparison-2023-02-06--10h20m33s.png

As the above diagram examples represent: if we put and compared all benchmarks in one single group, then the large ones will dominate the diagram, and all the remaining small ones will look as indistinguishable visually.

## Grouping by type of the big input

In order to solve the above-mentioned visualizability problem, we can split the former common single big comparison group into two groups. After some considerations, it seemed that the leading principle of the splitting should be the type of the big input:

* big input of type `&str`: grouping those benches together where the big input data is a string slice
* big input of type `&Vec<char>`: grouping those benches together where the big input data is ref to a Vec of chars

### Grouping those benches together where the big input data is a string slice

![Grouping those together where the big input data is a string slice](sample-diagrams/comparison-2-strslice-input-2023-02-06--14h52m28s.png

### Grouping those together benches where the big input data is ref to a Vec of chars

![Grouping those together where the big input data is ref to a Vec of chars](sample-diagrams/comparison-2-charvec-input-2023-02-06--14h53m44s.png

## Cross-section

As we can see, this refined approch solves the problem of the first approach: now each diagrams show its contained bechmarks in such a way that we do not get lost and the bigger ones do not make the smaller ones blurring together visually in an indistinguishable manner.

For completeness of the explanations, we can make also a third auxiliary group: this is not a separate base group showing any new bench items, but a redundant selection of the former two groups, in such a way that the original problem can be seen well:

![Grouping mixed, crosscutting selection of samples together](sample-diagrams/comparison-2-mixed-crosscutting-2023-02-06--14h55m04s.png

thus, this is a grouping to show a mixed, crosscutting selection of samples together. Although this group does not repeat all benches lumped together, but still it is a selection that shows too large samples together with too small ones, intentionally in order to show the visualizability problem why we have to avoid lumping together the small and the large bench items and maintain a carefully planned grouping policy.

Now, after having seen the above generated diagrams, the details of the benchmarks can be seen in the module itself that solves the task accordingly: the [benches/bench_criterionized_runner.rs](../benches/bench_criterionized_runner.rs) module. As mentioned, the former, earlier benchmark module is [src/bench.rs](../src/bench.rs). In short: [benches/bench_criterionized_runner.rs](../benches/bench_criterionized_runner.rs) is a refactored, criterionized version of [src/bench.rs](../src/bench.rs).

## To do

As for future improvements:

* The benchmarks in the [benches/bench_criterionized_runner.rs](../benches/bench_criterionized_runner.rs) file can be made more DRY, lessening redundancy, by more sophisticated usage of macros.
* Unlike the old bechmarks in the [src/bench.rs](../src/bench.rs) file, the new benchmarks of the [benches/bench_criterionized_runner.rs](../benches/bench_criterionized_runner.rs) file do not make use of the length results (thus, this functionality of the old benches gets lost in the new ones). The use and integration of this length info into a statistics of Criterion can be a goal of a further development.
* During the criterionization, some provacy settings have been changed of the API, it can be revised:
    * `mod tokengroup` to `pub mod tokengroup` in [src/lib.rs](../src/lib.rs)
    * `pub(crate) fn chars_of` to `pub fn chars_of` in [src/tokengroup](../src/tokengroup)
