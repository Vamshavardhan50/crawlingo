// Source inventory, not a runtime compatibility or test-coverage claim.
const fs = require('node:fs');
const path = require('node:path');
const crypto = require('node:crypto');
const { execFileSync } = require('node:child_process');
const root = path.resolve(__dirname, '..');
const ts = require(path.join(root, 'sdk/nodejs/node_modules/typescript'));
const sdk = path.join(root, 'sdk/nodejs');
const config = ts.readConfigFile(path.join(sdk, 'tsconfig.json'), ts.sys.readFile);
const parsed = ts.parseJsonConfigFileContent(config.config, ts.sys, sdk);
const program = ts.createProgram(parsed.fileNames, parsed.options);
const checker = program.getTypeChecker();
const entry = program.getSourceFile(path.join(sdk, 'src/index.ts'));
const exported = checker.getExportsOfModule(checker.getSymbolAtLocation(entry));
const compact = value => value.replace(/\s+/g, ' ').trim();
const location = node => ({ file: path.relative(root, node.getSourceFile().fileName).replaceAll('\\', '/'), line: node.getSourceFile().getLineAndCharacterOfPosition(node.getStart()).line + 1 });
const description = node => ts.displayPartsToString(checker.getSymbolAtLocation(node.name)?.getDocumentationComment(checker) || []);
const privateNode = node => node.modifiers?.some(m => [ts.SyntaxKind.PrivateKeyword, ts.SyntaxKind.ProtectedKeyword].includes(m.kind));
function member(node) {
  const sig = node.parameters ? checker.getSignatureFromDeclaration(node) : undefined;
  const name = ts.isConstructorDeclaration(node) ? 'constructor' : node.name.getText();
  const isMethod = ts.isMethodDeclaration(node) || ts.isMethodSignature(node) || ts.isFunctionDeclaration(node) || ts.isConstructorDeclaration(node);
  let signature = isMethod ? name + checker.signatureToString(sig, node, ts.TypeFormatFlags.NoTruncation, ts.SignatureKind.Call) : compact(node.getText().split(' {')[0]);
  if (ts.isGetAccessorDeclaration(node)) signature = name + ': ' + checker.typeToString(checker.getReturnTypeOfSignature(sig), node, ts.TypeFormatFlags.NoTruncation);
  return {
    name, kind: isMethod ? 'method' : ts.isEnumMember(node) ? 'variant' : 'property', signature: signature.replace(/\r\n/g, '\n'),
    description: description(node), static: !!node.modifiers?.some(m => m.kind === ts.SyntaxKind.StaticKeyword),
    parameters: (node.parameters || []).map(p => ({ name: p.name.getText(), type: checker.typeToString(checker.getTypeAtLocation(p), p, ts.TypeFormatFlags.NoTruncation), optional: !!p.questionToken || !!p.initializer, default: p.initializer?.getText() ?? null })),
    returns: sig ? checker.typeToString(checker.getReturnTypeOfSignature(sig), node, ts.TypeFormatFlags.NoTruncation) : checker.typeToString(checker.getTypeAtLocation(node), node, ts.TypeFormatFlags.NoTruncation),
    ...location(node),
  };
}
const symbols = exported.map(exp => {
  const symbol = exp.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(exp) : exp;
  const node = symbol.declarations[0];
  const members = (node.members || []).filter(m => !privateNode(m)).map(member);
  // Public constructor parameter properties (e.g. Page.nativePage).
  for (const ctor of (node.members || []).filter(ts.isConstructorDeclaration)) {
    for (const p of ctor.parameters) if (p.modifiers && !privateNode(p)) members.push(member(p));
  }
  if (ts.isFunctionDeclaration(node)) members.push(member(node));
  const kind = ts.isClassDeclaration(node) ? 'class' : ts.isInterfaceDeclaration(node) ? 'interface' : ts.isEnumDeclaration(node) ? 'enum' : ts.isFunctionDeclaration(node) ? 'function' : 'type';
  return { name: exp.name, kind, exported: true,
    signature: kind === 'class' ? compact(node.getText().slice(0, node.getText().indexOf('{'))) : kind === 'function' ? member(node).signature : node.getText().replace(/\r\n/g, '\n'),
    description: description(node), members, ...location(node) };
});
const python = JSON.parse(execFileSync(process.env.PYTHON || 'python', [path.join(__dirname, 'export-python-api.py')], { encoding: 'utf8' }));
const files = new Set(['sdk/nodejs/src/index.ts', 'sdk/python/crawlingo/__init__.py', ...symbols.map(s => s.file), ...python.symbols.map(s => s.file)]);
// The Python binding audit reads Rust implementations, so include those inputs too.
function collectRust(directory) {
  for (const item of fs.readdirSync(directory, { withFileTypes: true })) {
    const file = path.join(directory, item.name);
    if (item.isDirectory()) collectRust(file);
    else if (file.endsWith('.rs')) files.add(path.relative(root, file).replaceAll('\\', '/'));
  }
}
collectRust(path.join(root, 'src'));
files.add('sdk/nodejs/native/src/lib.rs');
files.add('sdk/nodejs/tsconfig.json');
files.add('sdk/nodejs/package.json');
const sourceFiles = [...files].sort().map(file => ({ file, sha256: crypto.createHash('sha256').update(fs.readFileSync(path.join(root, file), 'utf8').replace(/\r\n/g, '\n')).digest('hex') }));
const inventory = {
  schemaVersion: 1,
  version: JSON.parse(fs.readFileSync(path.join(sdk, 'package.json'), 'utf8')).version,
  sourceCommit: execFileSync('git', ['rev-parse', 'HEAD'], { cwd: root, encoding: 'utf8' }).trim(),
  sourceModified: !!execFileSync('git', ['status', '--porcelain', '--', 'src', 'sdk'], { cwd: root, encoding: 'utf8' }).trim(),
  sourceHashFormat: 'utf8-lf',
  sourceDigest: crypto.createHash('sha256').update(JSON.stringify(sourceFiles)).digest('hex'),
  sourceFiles, node: { exports: exported.map(s => s.name), symbols }, python,
};
const output = path.resolve(process.argv[2] || path.join(root, 'crawlingo-docs/src/content/generated/sdk-api.json'));
fs.mkdirSync(path.dirname(output), { recursive: true });
fs.writeFileSync(output, JSON.stringify(inventory, null, 2) + '\n');
for (const language of ['node', 'python']) console.log(`${language}: ${inventory[language].exports.length} package exports; ${inventory[language].symbols.length} documented symbols; ${inventory[language].symbols.reduce((sum, s) => sum + s.members.length, 0)} declared members`);
console.log(`Saved source inventory to ${output}`);
