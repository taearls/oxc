use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{
    context::LintContext,
    fixer::{RuleFix, RuleFixer},
    rule::Rule,
    AstNode,
};

fn no_extraneous_dependencies_diagnostic(span: Span) -> OxcDiagnostic {
    // See <https://oxc.rs/docs/contribute/linter/adding-rules.html#diagnostics> for details
    OxcDiagnostic::warn("Should be an imperative statement about what is wrong")
        .with_help("Should be a command-like statement that tells the user how to fix the issue")
        .with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct NoExtraneousDependencies;

declare_oxc_lint!(
    /// ### What it does
    /// Forbid the import of external modules that are not declared in the package.json's dependencies, devDependencies, optionalDependencies, peerDependencies, or bundledDependencies. The closest parent package.json will be used. If no package.json is found, the rule will not lint anything. This behavior can be changed with the rule option packageDir. Normally ignores imports of modules marked internal, but this can be changed with the rule option includeInternal. Type imports can be verified by specifying includeTypes.
    /// Modules have to be installed for this rule to work.
    ///
    /// ### Why is this bad?
    ///
    /// Developers may be importing functionality from transient dependencies that they have not directly installed. If the dependency pulling in this transient module is updated, or removed, it may disrupt their application from working as expected.
    ///
    /// ### Examples
    ///
    /// Given the following package.json:
    /// 
    /// ```json
    /// {
    ///     "name": "my-project",
    ///     "...": "...",
    ///     "dependencies": {
    ///         "builtin-modules": "^1.1.1",
    ///         "lodash.cond": "^4.2.0",
    ///         "lodash.find": "^4.2.0",
    ///         "pkg-up": "^1.0.0"
    ///     },
    ///     "devDependencies": {
    ///         "ava": "^0.13.0",
    ///         "eslint": "^2.4.0",
    ///         "eslint-plugin-ava": "^1.3.0",
    ///         "xo": "^0.13.0"
    ///     },
    ///     "optionalDependencies": {
    ///         "lodash.isarray": "^4.0.0"
    ///     },
    ///     "peerDependencies": {
    ///         "react": ">=15.0.0 <16.0.0"
    ///     },
    ///     "bundledDependencies": [
    ///         "@generated/foo",
    ///     ]
    /// }
    /// ```
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```js
    /// var _ = require('lodash');
    /// import _ from 'lodash';
    ///
    /// import react from 'react';
    ///
    /// /* eslint import/no-extraneous-dependencies: ["error", {"devDependencies": false}] */
    /// import test from 'ava';
    /// var test = require('ava');
    /// /* eslint import/no-extraneous-dependencies: ["error", {"optionalDependencies": false}] */
    /// import isArray from 'lodash.isarray';
    /// var isArray = require('lodash.isarray');
    /// 
    /// /* eslint import/no-extraneous-dependencies: ["error", {"bundledDependencies": false}] */
    /// import foo from '"@generated/foo"';
    /// var foo = require('"@generated/foo"');
    /// 
    /// /* eslint import/no-extraneous-dependencies: ["error", {"includeInternal": true}] */
    /// import foo from './foo';
    /// var foo = require('./foo');
    /// /* eslint import/no-extraneous-dependencies: ["error", {"includeTypes": true}] */
    /// import type { MyType } from 'foo';
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```js
    /// // Builtin and internal modules are fine
    /// var path = require('path');
    /// var foo = require('./foo');
    ///
    /// import test from 'ava';
    /// import find from 'lodash.find';
    /// import isArray from 'lodash.isarray';
    /// import foo from '"@generated/foo"';
    /// import type { MyType } from 'foo';
    ///
    /// /* eslint import/no-extraneous-dependencies: ["error", {"peerDependencies": true}] */
    /// import react from 'react';
    /// ```
    NoExtraneousDependencies,
    import,
    restriction,
);

impl Rule for NoExtraneousDependencies {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {}
}

#[test]
fn test() {
    use crate::tester::Tester;

    // Source: https://github.com/import-js/eslint-plugin-import/blob/v2.31.0/tests/src/rules/no-extraneous-dependencies.js#L34
    // base case
    let packages = vec!["glob", "eslint", "@org/package", "esm-package", "jquery", "lodash.cond", "find-up", "rxjs"];

//     const packageDirWithSyntaxError = path.join(__dirname, '../../files/with-syntax-error');
// const packageFileWithSyntaxErrorMessage = (() => {
//   try {
//     JSON.parse(fs.readFileSync(path.join(packageDirWithSyntaxError, 'package.json')));
//   } catch (error) {
//     return error.message;
//   }
// })();
// const packageDirWithFlowTyped = path.join(__dirname, '../../files/with-flow-typed');
// const packageDirWithTypescriptDevDependencies = path.join(__dirname, '../../files/with-typescript-dev-dependencies');
// const packageDirMonoRepoRoot = path.join(__dirname, '../../files/monorepo');
// const packageDirMonoRepoWithNested = path.join(__dirname, '../../files/monorepo/packages/nested-package');
// const packageDirWithEmpty = path.join(__dirname, '../../files/empty');
// const packageDirBundleDeps = path.join(__dirname, '../../files/bundled-dependencies/as-array-bundle-deps');
// const packageDirBundledDepsAsObject = path.join(__dirname, '../../files/bundled-dependencies/as-object');
// const packageDirBundledDepsRaceCondition = path.join(__dirname, '../../files/bundled-dependencies/race-condition');
// const emptyPackageDir = path.join(__dirname, '../../files/empty-folder');

    let pass = vec![
        r#"import "${pkg}""#,
        r#"import foo, { bar } from "${pkg}""#,
        r#"require("${pkg}")"#,
        r#"var foo = require("${pkg}")"#,
        r#"export { foo } from "${pkg}""#,
        r#"export * from "${pkg}""#,
        r#"import "eslint/lib/api""#,
        r#"import "fs""#,
        r#"import "./foo""#,
        r#"import "electron""#, // TODO: add settings settings: { 'import/core-modules': ['electron'] }
        r#"import "eslint""#, // TODO: add options options: [{ peerDependencies: true }]
        r#"import "importType""#, //       settings: { 'import/resolver': { node: { paths: [path.join(__dirname, '../../files')] } } },
        //
        r#"import chai from "chai""#, //options: [{ devDependencies: ['*.spec.js'] }], filename: 'foo.spec.js',
        r#"import chai from "chai""#, // options: [{ devDependencies: ['*.spec.js'] }], filename: path.join(process.cwd(), 'foo.spec.js'),
        r#"import chai from "chai""#, //     options: [{ devDependencies: ['*.test.js', '*.spec.js'] }],
        //     filename: path.join(process.cwd(), 'foo.spec.js'),
        r#"require(6)"#,
        r#"import "doctrine""#, //         //     options: [{ packageDir: path.join(__dirname, '../../../') }],
        r#"import type MyType from "myflowtyped";"#, //     options: [{ packageDir: packageDirWithFlowTyped }],
        //     parser: parsers.BABEL_OLD,
        r#"
            // @flow
            import typeof TypeScriptModule from 'typescript';
        "#,//     options: [{ packageDir: packageDirWithFlowTyped }],
        //     parser: parsers.BABEL_OLD,
        r#"import react from "react";"#, // options: [{ packageDir: packageDirMonoRepoWithNested }],
        r#"import leftpad from "left-pad";"#,//     options: [{ packageDir: [packageDirMonoRepoWithNested, packageDirMonoRepoRoot] }],
        r#"import leftpad from "left-pad";"#,//     options: [{ packageDir: packageDirMonoRepoRoot }],
        r#"import leftpad from "left-pad";"#, // options: [{ packageDir: [emptyPackageDir, packageDirMonoRepoRoot] }],
        r#"import leftpad from "left-pad";"#, //     options: [{ packageDir: [packageDirMonoRepoRoot, emptyPackageDir] }],
        r#"import react from "react";"#, //     options: [{ packageDir: [packageDirMonoRepoRoot, packageDirMonoRepoWithNested] }],
        r#"import leftpad from "left-pad";"#, //     options: [{ packageDir: [packageDirMonoRepoRoot, packageDirMonoRepoWithNested] }],
        r#"import rightpad from "right-pad";"#, //     options: [{ packageDir: [packageDirMonoRepoRoot, packageDirMonoRepoWithNested] }],
        r#"import foo from "@generated/foo""#,
        r#"import foo from "@generated/foo""#,  //     options: [{ packageDir: packageDirBundleDeps }],
        r#"import foo from "@generated/foo""#,//     options: [{ packageDir: packageDirBundledDepsAsObject }],
        r#"import foo from "@generated/foo""#, //     options: [{ packageDir: packageDirBundledDepsRaceCondition }],
        r#"export function getToken() {}"#,
        r#"export class Component extends React.Component {}"#,
        r#"export function Component() {}"#,
        r#"export const Component = () => {}"#,
        r#"import "not-a-dependency""#, //     filename: path.join(packageDirMonoRepoRoot, 'foo.js'),
        //     options: [{ packageDir: packageDirMonoRepoRoot }],
        //     settings: { 'import/core-modules': ['not-a-dependency'] },
        r#"import "@generated/bar/module""#, //     settings: { 'import/core-modules': ['@generated/bar'] },
        r#"import "@generated/bar/and/sub/path""#,         //     settings: { 'import/core-modules': ['@generated/bar'] },
        r#"import "rxjs/operators""#,
        r#"import "esm-package/esm-module";"#,
        r#"
            import "alias/esm-package/esm-module";
            import 'expose-loader?exposes[]=$&exposes[]=jQuery!jquery';
        "#, //     settings: { 'import/resolver': 'webpack' },
        r#"import "@custom-internal-alias/api/service";"#,   
        //     settings: {
        //       'import/resolver': {
        //         webpack: {
        //           config: {
        //             resolve: {
        //               alias: {
        //                 '@custom-internal-alias': testFilePath('internal-modules'),
        //               },
        //             },
        //           },
        //         },
        //       },
        //     },
        //   }),
        // ],
        
    ];


    let fail = vec![
        r#"import "not-a-dependency""#,     //         filename: path.join(packageDirMonoRepoRoot, 'foo.js'), options: [{ packageDir: packageDirMonoRepoRoot }],
        r#"import "not-a-dependency""#, //         filename: path.join(packageDirMonoRepoWithNested, 'foo.js'),
        //         options: [{ packageDir: packageDirMonoRepoRoot }],
        r#"import "not-a-dependency""#, //         options: [{ packageDir: packageDirMonoRepoRoot }],
        r#"import "not-a-dependency""#,
        r#"var donthaveit = require("@org/not-a-dependency")"#,
        r#"var donthaveit = require("@org/not-a-dependency/foo")"#,
        r#"import "eslint""#,     //         options: [{ devDependencies: false, peerDependencies: false }],
        r#"import "lodash.isarray""#, //    //         options: [{ optionalDependencies: false }],
        r#"var foo = require("not-a-dependency")"#,
        r#"var glob = require("glob")"#,    //         options: [{ devDependencies: false }],
        r#"import chai from "chai""#,    //         options: [{ devDependencies: ['*.test.js'] }],     //         filename: 'foo.test.js',
        r#"import chai from "chai""#, //         options: [{ devDependencies: ['*.test.js'] }],
    //         filename: path.join(process.cwd(), 'foo.test.js'),
        r#"import chai from "chai""#, //         options: [{ devDependencies: ['*.test.js', '*.spec.js'] }],
        //         filename: 'foo.tes.js',
        r#"import chai from "chai""#, //         options: [{ devDependencies: ['*.test.js', '*.spec.js'] }],
        //         filename: path.join(process.cwd(), 'foo.tes.js'),
        r#"var eslint = require("lodash.isarray")"#,    //         options: [{ optionalDependencies: false }],
        r#"import "not-a-dependency""#,    //         options: [{ packageDir: path.join(__dirname, '../../../') }],
        r#"import "bar""#,     //         options: [{ packageDir: path.join(__dirname, './doesn-exist/') }],
        r#"import foo from "foo""#,     //         options: [{ packageDir: packageDirWithSyntaxError }],
        r#"import leftpad from "left-pad";"#,  //         filename: path.join(packageDirMonoRepoWithNested, 'foo.js'),
        //         options: [{ packageDir: packageDirMonoRepoWithNested }],
        r#"import react from "react";"#, //         filename: path.join(packageDirMonoRepoRoot, 'foo.js'),
        r#"import react from "react";"#,  //         filename: path.join(packageDirMonoRepoWithNested, 'foo.js'),
        //         options: [{ packageDir: packageDirMonoRepoRoot }],
        r#"import "react";"#, //         filename: path.join(packageDirWithEmpty, 'index.js'),
        //         options: [{ packageDir: packageDirWithEmpty }],
        r#"import bar from "@generated/bar""#,
        r#"import foo from "@generated/foo""#,     //         options: [{ bundledDependencies: false }],
        r#"import bar from "@generated/bar""#,    //         options: [{ packageDir: packageDirBundledDepsRaceCondition }],
        r#"export { foo } from "not-a-dependency";"#,
        r#"export * from "not-a-dependency";"#,
        r#"import chai from "alias/chai";"#,    //         settings: { 'import/resolver': 'webpack' },
        r#"import "esm-package-not-in-pkg-json/esm-module";"#,
        r#"import "not-a-dependency""#,  //         settings: {
            //           'import/resolver': { node: { paths: [path.join(__dirname, '../../files')] } },
            //           'import/internal-regex': '^not-a-dependency.*',
            //         },
            //         options: [{ includeInternal: true }],
        
    ];

    Tester::new(NoExtraneousDependencies::NAME, NoExtraneousDependencies::PLUGIN, pass, fail)
        .test_and_snapshot();
}
