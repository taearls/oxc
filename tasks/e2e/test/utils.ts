import fs from 'node:fs';
import path from 'node:path';
import url from 'node:url';

import { createFsRequire } from 'fs-require';
import { Volume } from 'memfs';
import { minify as oxcMinify } from 'oxc-minify';
import { transform as oxcTransform } from 'oxc-transform';

const nodeModulesPath = path.resolve(path.dirname(url.fileURLToPath(import.meta.url)), '../node_modules');

const minifyOptions: any[] = [
  { compress: true, mangle: true, codegen: { whitespace: true } },
  { compress: true, mangle: true, codegen: { whitespace: true } },
].map((o) => ({ type: 'minify', ...o }));

const transformOptions: any[] = [
  { target: 'esnext' },
  { target: 'es2024' },
  { target: 'es2023' },
  { target: 'es2022' },
  { target: 'es2021' },
  { target: 'es2020' },
  { target: 'es2019' },
  { target: 'es2018' },
  { target: 'es2017' },
  { target: 'es2016' },
  { target: 'es2015' },
].map((o) => ({ type: 'transform', ...o }));

export function getModules(dir: string, fileName: string) {
  const p = path.join(nodeModulesPath, dir + fileName);
  const code = fs.readFileSync(p, 'utf8');
  return minifyOptions.concat(transformOptions)
    .map(({ type, ...options }) => {
      const modifiedCode = {
        minify: oxcMinify,
        transform: oxcTransform,
      }[type](fileName, code).code;
      return { module: fsRequire(modifiedCode), type, options };
    });
}

function fsRequire(code: string) {
  const vol = Volume.fromJSON({ '/index.js': code });
  const fsRequire = createFsRequire(vol);
  return fsRequire('/index.js');
}
