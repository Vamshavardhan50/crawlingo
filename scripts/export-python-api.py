"""Extract the public Python wrapper surface without importing native binaries."""
import ast
import json
import pathlib
import re
import sys

root = pathlib.Path(__file__).resolve().parents[1]
package = root / "sdk/python/crawlingo"
init = ast.parse((package / "__init__.py").read_text(encoding="utf-8"))
exports = next(ast.literal_eval(n.value) for n in init.body
               if isinstance(n, ast.Assign) and any(isinstance(t, ast.Name) and t.id == "__all__" for t in n.targets))
reachable = {"Element", "CrawlResults", "RequestContext", "ResponseContext"}
protocols = {"__init__", "__enter__", "__exit__", "__iter__", "__len__", "__getitem__", "__repr__"}
symbols = []
native_methods = {}
for source in (root / "src").rglob("*.rs"):
    for match in re.finditer(r"impl (Py\w+) \{(.*?)^\}", source.read_text(encoding="utf-8"), re.S | re.M):
        native_methods.setdefault(match[1], set()).update(re.findall(r"(?:pub )?(?:async )?fn (\w+)\s*\(", match[2]))
        native_methods[match[1]].update(re.findall(r'#\[pyo3\(name\s*=\s*"([^"]+)"\)\]', match[2]))

def location(file, node):
    return {"file": file.relative_to(root).as_posix(), "line": node.lineno}

def function(file, node):
    parameters = []
    args = node.args.posonlyargs + node.args.args
    defaults = [None] * (len(args) - len(node.args.defaults)) + node.args.defaults
    for arg, default in zip(args, defaults):
        if arg.arg in {"self", "cls"}: continue
        parameters.append({"name": arg.arg, "type": ast.unparse(arg.annotation) if arg.annotation else "Not annotated",
                           "optional": default is not None, "default": ast.unparse(default) if default is not None else None})
    for arg, default in zip(node.args.kwonlyargs, node.args.kw_defaults):
        parameters.append({"name": arg.arg, "type": ast.unparse(arg.annotation) if arg.annotation else "Not annotated",
                           "optional": default is not None, "default": ast.unparse(default) if default is not None else None})
    for prefix, arg in [("*", node.args.vararg), ("**", node.args.kwarg)]:
        if arg:
            parameters.append({"name": prefix + arg.arg, "type": ast.unparse(arg.annotation) if arg.annotation else "Not annotated",
                               "optional": True, "default": None})
    decorators = [ast.unparse(d) for d in node.decorator_list]
    signature = ("async " if isinstance(node, ast.AsyncFunctionDef) else "") + "def " + node.name + "(" + ast.unparse(node.args) + ")"
    if node.returns: signature += " -> " + ast.unparse(node.returns)
    return {"name": node.name, "kind": "property" if "property" in decorators else "method",
            "signature": signature, "description": ast.get_docstring(node) or "",
            "parameters": parameters, "returns": ast.unparse(node.returns) if node.returns else "Not annotated; see behavior notes",
            "static": "classmethod" in decorators or "staticmethod" in decorators, **location(file, node)}

for file in sorted(package.glob("*.py")):
    tree = ast.parse(file.read_text(encoding="utf-8"))
    nested_contexts = [n for n in ast.walk(tree) if isinstance(n, ast.ClassDef) and n.name in {"RequestContext", "ResponseContext"}]
    for node in tree.body + nested_contexts:
        if isinstance(node, ast.ClassDef) and node.name in set(exports) | reachable:
            members = []
            for child in node.body:
                if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef)) and (not child.name.startswith("_") or child.name in protocols):
                    item = function(file, child)
                    missing = []
                    for call in ast.walk(child):
                        if not isinstance(call, ast.Call) or not isinstance(call.func, ast.Attribute): continue
                        owner = call.func.value
                        if isinstance(owner, ast.Attribute) and isinstance(owner.value, ast.Name) and owner.value.id == "self" and owner.attr.startswith("_core"):
                            binding = "Py" + node.name
                            if binding in native_methods and call.func.attr not in native_methods[binding]: missing.append(call.func.attr)
                    item["bindingIssues"] = sorted(set(missing))
                    members.append(item)
                    if child.name == "__init__":
                        params = {p["name"]: p["type"] for p in item["parameters"]}
                        for assignment in child.body:
                            if isinstance(assignment, ast.Assign):
                                for target in assignment.targets:
                                    if isinstance(target, ast.Attribute) and isinstance(target.value, ast.Name) and target.value.id == "self" and not target.attr.startswith("_"):
                                        typ = params.get(assignment.value.id, "Not annotated") if isinstance(assignment.value, ast.Name) else "Not annotated"
                                        members.append({"name": target.attr, "kind": "property", "signature": target.attr + ": " + typ, "description": "Instance field assigned by the constructor.", "parameters": [], "returns": typ, **location(file, assignment)})
                elif isinstance(child, ast.AnnAssign) and isinstance(child.target, ast.Name) and not child.target.id.startswith("_"):
                    members.append({"name": child.target.id, "kind": "property", "signature": ast.unparse(child),
                                    "description": "", "parameters": [], "returns": ast.unparse(child.annotation), **location(file, child)})
                elif node.name == "FieldType" and isinstance(child, ast.Assign):
                    members.append({"name": child.targets[0].id, "kind": "variant", "signature": ast.unparse(child),
                                    "description": "", "parameters": [], "returns": "FieldType", **location(file, child)})
            symbols.append({"name": node.name, "kind": "class", "exported": node.name in exports,
                            "signature": "class " + node.name + ("(" + ", ".join(ast.unparse(b) for b in node.bases) + ")" if node.bases else ""),
                            "description": ast.get_docstring(node) or "", "members": members, **location(file, node)})
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and (node.name in exports or file.stem == "hooks") and not node.name.startswith("_"):
            member = function(file, node)
            symbols.append({**member, "name": "hooks." + node.name if file.stem == "hooks" else node.name,
                            "kind": "function", "exported": file.stem != "hooks", "members": [member]})

# This public callback value is defined in Rust rather than a Python wrapper.
event = root / "src/change/detector.rs"
lines = event.read_text(encoding="utf-8").splitlines()
start = next(i for i, line in enumerate(lines) if "pub struct PyChangeEvent" in line)
members = []
for i in range(start + 1, len(lines)):
    line = lines[i].strip()
    if line == "}": break
    if line.startswith("pub "):
        name, typ = line[4:].split(":", 1)
        typ = "float" if typ.strip().startswith("f64") else "str"
        members.append({"name": name, "kind": "property", "signature": name + ": " + typ,
                        "description": "Read-only callback event field.", "parameters": [], "returns": typ,
                        "file": "src/change/detector.rs", "line": i + 1})
symbols.append({"name": "ChangeEvent", "kind": "class", "exported": True, "signature": "class ChangeEvent",
                "description": "Read-only value supplied to Python Watch callbacks; constructed by the native engine.",
                "members": members, "file": "src/change/detector.rs", "line": start + 1})
symbols.append({"name": "hooks", "kind": "module", "exported": True, "signature": "from crawlingo import hooks",
                "description": "Value-cleaning and logging helpers for Python Page hooks.",
                "members": [s["members"][0] for s in symbols if s["name"].startswith("hooks.")],
                "file": "sdk/python/crawlingo/hooks.py", "line": 1})
print(json.dumps({"exports": exports, "symbols": symbols}, ensure_ascii=False))
