"use client";

import { useMemo } from "react";

interface SourceViewerProps {
  source: string;
  filename: string;
}

function highlightRustLine(line: string): React.ReactNode[] {
  const trimmed = line.trimStart();

  // Comments
  if (trimmed.startsWith("//")) {
    return [<span key={0} className="text-zinc-400 italic">{line}</span>];
  }
  if (trimmed.startsWith("*") || trimmed.startsWith("/*")) {
    return [<span key={0} className="text-zinc-400 italic">{line}</span>];
  }

  const keywordSet = new Set([
    "pub", "use", "mod", "crate", "super", "self", "struct", "enum",
    "trait", "impl", "fn", "async", "await", "return", "if", "else",
    "match", "while", "for", "loop", "break", "continue", "let", "mut",
    "const", "static", "ref", "move", "where", "dyn", "type", "as",
    "in", "true", "false", "Some", "None", "Ok", "Err", "Result",
  ]);

  const parts = line.split(
    /(\b(?:pub|use|mod|crate|super|self|struct|enum|trait|impl|fn|async|await|return|if|else|match|while|for|loop|break|continue|let|mut|const|static|ref|move|where|dyn|type|as|in|true|false|Some|None|Ok|Err|Result)\b|"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*'|\/\/.*$|\b\d+(?:\.\d+)?\b)/
  );

  return parts.map((part, idx) => {
    if (!part) return null;
    if (keywordSet.has(part)) {
      return <span key={idx} className="text-blue-400 font-medium">{part}</span>;
    }
    if (part === "self" || part === "super" || part === "crate") {
      return <span key={idx} className="text-purple-400">{part}</span>;
    }
    if (part.startsWith("//")) {
      return <span key={idx} className="text-zinc-400 italic">{part}</span>;
    }
    if (
      (part.startsWith('"') && part.endsWith('"')) ||
      (part.startsWith("'") && part.endsWith("'"))
    ) {
      return <span key={idx} className="text-emerald-500">{part}</span>;
    }
    if (/^\d+(?:\.\d+)?$/.test(part)) {
      return <span key={idx} className="text-orange-400">{part}</span>;
    }
    return <span key={idx}>{part}</span>;
  });
}

export function SourceViewer({ source, filename }: SourceViewerProps) {
  const lines = useMemo(() => source.split("\n"), [source]);

  return (
    <div className="rounded-lg border border-zinc-200 dark:border-zinc-700">
      <div className="flex items-center gap-2 border-b border-zinc-200 px-4 py-2 dark:border-zinc-700">
        <div className="flex gap-1.5">
          <span className="h-3 w-3 rounded-full bg-red-400" />
          <span className="h-3 w-3 rounded-full bg-yellow-400" />
          <span className="h-3 w-3 rounded-full bg-green-400" />
        </div>
        <span className="font-mono text-xs text-zinc-400">{filename}</span>
      </div>
      <div className="overflow-x-auto bg-zinc-950">
        <pre className="p-2 text-[10px] leading-4 sm:p-4 sm:text-xs sm:leading-5">
          <code>
            {lines.map((line, i) => (
              <div key={i} className="flex">
                <span className="mr-2 inline-block w-6 shrink-0 select-none text-right text-zinc-600 sm:mr-4 sm:w-8">
                  {i + 1}
                </span>
                <span className="text-zinc-200">
                  {highlightRustLine(line)}
                </span>
              </div>
            ))}
          </code>
        </pre>
      </div>
    </div>
  );
}
