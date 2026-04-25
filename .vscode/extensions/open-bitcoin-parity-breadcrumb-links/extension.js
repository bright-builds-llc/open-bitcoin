"use strict";

const fs = require("node:fs");
const path = require("node:path");
const vscode = require("vscode");

const BREADCRUMB_PATTERN = /packages\/bitcoin-knots\/[A-Za-z0-9._/-]+/g;

function activate(context) {
  const provider = {
    provideDocumentLinks(document) {
      const links = [];

      for (let lineNumber = 0; lineNumber < document.lineCount; lineNumber += 1) {
        const line = document.lineAt(lineNumber).text;
        for (const match of line.matchAll(BREADCRUMB_PATTERN)) {
          const relativePath = match[0];
          const maybeTarget = maybeResolveTarget(document.uri.fsPath, relativePath);
          if (maybeTarget === null) {
            continue;
          }

          const start = match.index;
          const end = start + relativePath.length;
          const range = new vscode.Range(lineNumber, start, lineNumber, end);
          const link = new vscode.DocumentLink(range, vscode.Uri.file(maybeTarget));
          link.tooltip = `Open ${relativePath}`;
          links.push(link);
        }
      }

      return links;
    },
  };

  context.subscriptions.push(
    vscode.languages.registerDocumentLinkProvider({ scheme: "file", language: "rust" }, provider),
  );
}

function maybeResolveTarget(documentPath, relativePath) {
  let directory = path.dirname(documentPath);

  while (true) {
    const candidate = path.join(directory, relativePath);
    if (fs.existsSync(candidate)) {
      return candidate;
    }

    const parent = path.dirname(directory);
    if (parent === directory) {
      return null;
    }
    directory = parent;
  }
}

function deactivate() {}

module.exports = {
  activate,
  deactivate,
};
