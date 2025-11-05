# Prettier Conformance Test Results - Development Branch

**Branch:** temp/fix-remaining-conformance-tests

## Overall Summary

- **JavaScript compatibility:** 700/749 (93.46%)
- **TypeScript compatibility:** 547/598 (91.47%)

## JavaScript Test Results

| Test Name                                                        | % Passing |          Code Area           |
| :--------------------------------------------------------------- | :-------: | :--------------------------: |
| js/arrows/comment.js                                             |  88.89%   |            arrows            |
| js/call/boolean/boolean.js                                       |  77.88%   |             call             |
| js/class-comment/misc.js                                         |  72.73%   |        class-comment         |
| js/class-comment/superclass.js                                   |  95.35%   |        class-comment         |
| js/comments/15661.js                                             |  55.17%   |           comments           |
| js/comments/dangling_for.js                                      |  22.22%   |           comments           |
| js/comments/empty-statements.js                                  |  90.91%   |           comments           |
| js/comments/function-declaration.js                              |  92.80%   |           comments           |
| js/comments/if.js                                                |  74.83%   |           comments           |
| js/comments/return-statement.js                                  |  98.28%   |           comments           |
| js/conditional/comments.js                                       |  23.69%   |         conditional          |
| js/conditional/new-ternary-examples.js                           |  20.14%   |         conditional          |
| js/conditional/new-ternary-spec.js                               |  24.35%   |         conditional          |
| js/conditional/postfix-ternary-regressions.js                    |  20.77%   |         conditional          |
| js/explicit-resource-management/for-await-using-of-comments.js   |   0.00%   | explicit-resource-management |
| js/explicit-resource-management/valid-await-using-comments.js    |  66.67%   | explicit-resource-management |
| js/for/9812-unstable.js                                          |  45.45%   |             for              |
| js/for/9812.js                                                   |  82.83%   |             for              |
| js/for/for-in-with-initializer.js                                |  37.50%   |             for              |
| js/for/parentheses.js                                            |  96.00%   |             for              |
| js/identifier/for-of/let.js                                      |  92.31%   |          identifier          |
| js/identifier/parentheses/let.js                                 |  84.09%   |          identifier          |
| js/if/comment-between-condition-and-body.js                      |  65.79%   |              if              |
| js/if/expr_and_same_line_comments.js                             |  97.73%   |              if              |
| js/if/if_comments.js                                             |  76.00%   |              if              |
| js/if/trailing_comment.js                                        |  91.43%   |              if              |
| js/last-argument-expansion/dangling-comment-in-arrow-function.js |  22.22%   |   last-argument-expansion    |
| js/object-multiline/multiline.js                                 |  22.22%   |       object-multiline       |
| js/quote-props/classes.js                                        |  47.06%   |         quote-props          |
| js/quote-props/objects.js                                        |  45.10%   |         quote-props          |
| js/quote-props/with_numbers.js                                   |  46.43%   |         quote-props          |
| js/quotes/objects.js                                             |  80.00%   |            quotes            |
| js/sequence-expression/ignored.js                                |  25.00%   |     sequence-expression      |
| js/strings/template-literals.js                                  |  98.01%   |           strings            |
| js/ternaries/binary.js                                           |  18.42%   |          ternaries           |
| js/ternaries/func-call.js                                        |  25.00%   |          ternaries           |
| js/ternaries/indent-after-paren.js                               |  24.59%   |          ternaries           |
| js/ternaries/indent.js                                           |   4.94%   |          ternaries           |
| js/ternaries/nested-in-condition.js                              |  19.74%   |          ternaries           |
| js/ternaries/nested.js                                           |  15.12%   |          ternaries           |
| js/ternaries/parenthesis.js                                      |  12.50%   |          ternaries           |
| js/ternaries/test.js                                             |  22.40%   |          ternaries           |
| js/ternaries/parenthesis/await-expression.js                     |  14.29%   |          ternaries           |
| js/test-declarations/angularjs_inject.js                         |  91.53%   |      test-declarations       |
| js/test-declarations/test_declarations.js                        |  95.88%   |      test-declarations       |
| jsx/fbt/test.js                                                  |  84.06%   |             jsx              |
| jsx/jsx/quotes.js                                                |  79.41%   |             jsx              |
| jsx/single-attribute-per-line/single-attribute-per-line.js       |  43.37%   |             jsx              |
| jsx/text-wrap/test.js                                            |  99.56%   |             jsx              |

## TypeScript Test Results

| Test Name                                                                   | % Passing |         Code Area          |
| :-------------------------------------------------------------------------- | :-------: | :------------------------: |
| jsx/fbt/test.js                                                             |  84.06%   |            jsx             |
| jsx/jsx/quotes.js                                                           |  79.41%   |            jsx             |
| jsx/single-attribute-per-line/single-attribute-per-line.js                  |  43.37%   |            jsx             |
| jsx/text-wrap/test.js                                                       |  99.56%   |            jsx             |
| typescript/angular-component-examples/15934-computed.component.ts           |  76.92%   | angular-component-examples |
| typescript/angular-component-examples/15934.component.ts                    |  53.85%   | angular-component-examples |
| typescript/angular-component-examples/test.component.ts                     |  41.18%   | angular-component-examples |
| typescript/arrow/comments.ts                                                |  44.44%   |           arrow            |
| typescript/chain-expression/call-expression.ts                              |  82.81%   |      chain-expression      |
| typescript/chain-expression/member-expression.ts                            |  82.09%   |      chain-expression      |
| typescript/chain-expression/test.ts                                         |  50.00%   |      chain-expression      |
| typescript/class/empty-method-body.ts                                       |  80.00%   |           class            |
| typescript/class/extends_implements.ts                                      |  90.12%   |           class            |
| typescript/class/quoted-property.ts                                         |  66.67%   |           class            |
| typescript/class-and-interface/long-type-parameters/long-type-parameters.ts |  63.64%   |    class-and-interface     |
| typescript/class-comment/class-implements.ts                                |  89.25%   |       class-comment        |
| typescript/class-comment/declare.ts                                         |  84.62%   |       class-comment        |
| typescript/class-comment/generic.ts                                         |  92.00%   |       class-comment        |
| typescript/comments/mapped_types.ts                                         |  96.77%   |          comments          |
| typescript/comments/method_types.ts                                         |  82.05%   |          comments          |
| typescript/conditional-types/comments.ts                                    |  31.51%   |     conditional-types      |
| typescript/conditional-types/conditional-types.ts                           |  34.48%   |     conditional-types      |
| typescript/conditional-types/infer-type.ts                                  |   4.76%   |     conditional-types      |
| typescript/conditional-types/nested-in-condition.ts                         |  15.79%   |     conditional-types      |
| typescript/conditional-types/new-ternary-spec.ts                            |  10.67%   |     conditional-types      |
| typescript/conditional-types/parentheses.ts                                 |  15.22%   |     conditional-types      |
| typescript/decorators-ts/angular.ts                                         |  87.50%   |       decorators-ts        |
| typescript/instantiation-expression/17714.ts                                |   0.00%   |  instantiation-expression  |
| typescript/interface/comments-generic.ts                                    |  41.94%   |         interface          |
| typescript/interface/long-extends.ts                                        |  83.64%   |         interface          |
| typescript/interface/long-type-parameters/long-type-parameters.ts           |  87.33%   |         interface          |
| typescript/interface/no-semi/14040.ts                                       |  84.81%   |         interface          |
| typescript/interface2/break/break.ts                                        |  82.30%   |         interface2         |
| typescript/intersection/intersection-parens.ts                              |  86.17%   |        intersection        |
| typescript/intersection/consistent-with-flow/intersection-parens.ts         |  69.77%   |        intersection        |
| typescript/last-argument-expansion/decorated-function.tsx                   |  29.06%   |  last-argument-expansion   |
| typescript/mapped-type/issue-11098.ts                                       |  97.03%   |        mapped-type         |
| typescript/mapped-type/break-mode/break-mode.ts                             |  68.75%   |        mapped-type         |
| typescript/multiparser-css/issue-6259.ts                                    |  57.14%   |      multiparser-css       |
| typescript/non-null/optional-chain.ts                                       |  88.89%   |          non-null          |
| typescript/object-multiline/multiline.ts                                    |  23.21%   |      object-multiline      |
| typescript/prettier-ignore/mapped-types.ts                                  |  96.61%   |      prettier-ignore       |
| typescript/property-signature/consistent-with-flow/comments.ts              |  80.00%   |     property-signature     |
| typescript/property-signature/consistent-with-flow/union.ts                 |  85.71%   |     property-signature     |
| typescript/type-params/18041.ts                                             |  43.75%   |        type-params         |
| typescript/type-params/constraints-and-default-2.ts                         |  97.60%   |        type-params         |
| typescript/type-params/constraints-and-default.ts                           |  87.32%   |        type-params         |
| typescript/union/inlining.ts                                                |  97.78%   |           union            |
| typescript/union/union-parens.ts                                            |  92.59%   |           union            |
| typescript/union/consistent-with-flow/comment.ts                            |  78.26%   |           union            |
| typescript/union/single-type/single-type.ts                                 |   0.00%   |           union            |

## Code Area Summary

### JavaScript Areas with Most Issues

- **ternaries**: 9 tests, most with very low pass rates (4.94% - 25.00%)
- **comments**: 6 tests, mixed pass rates (22.22% - 98.28%)
- **conditional**: 4 tests, low pass rates (20.14% - 24.35%)
- **quote-props**: 3 tests, moderate pass rates (45.10% - 47.06%)
- **for**: 4 tests, varied pass rates (37.50% - 96.00%)
- **if**: 4 tests, moderate to high pass rates (65.79% - 97.73%)

### TypeScript Areas with Most Issues

- **conditional-types**: 6 tests, very low pass rates (4.76% - 34.48%)
- **interface**: 4 tests, mixed pass rates (41.94% - 87.33%)
- **jsx**: 4 tests, mixed pass rates (43.37% - 99.56%)
- **angular-component-examples**: 3 tests, low to moderate pass rates (41.18% - 76.92%)
