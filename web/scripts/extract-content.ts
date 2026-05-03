import * as fs from "fs";
import * as path from "path";
import type {
  LessonVersion,
  LessonDiff,
  DocContent,
  LessonIndex,
} from "../src/types/agent-data";
import { LESSON_META, LESSON_ORDER, LEARNING_PATH } from "../src/lib/constants";

const WEB_DIR = path.resolve(__dirname, "..");
const REPO_ROOT = path.resolve(WEB_DIR, "..");
const LESSONS_DIR = path.join(REPO_ROOT, "openai", "src", "lessons");
const DOCS_DIR = path.join(REPO_ROOT, "docs", "openai");
const OUT_DIR = path.join(WEB_DIR, "src", "data", "generated");

// Map Rust lesson filenames to lesson IDs
// lesson00_basic_chat.rs -> L00
// lesson13_full_agent.rs -> L13
function filenameToLessonId(filename: string): string | null {
  const base = path.basename(filename, ".rs");
  const match = base.match(/^lesson(\d+)_/);
  if (!match) return null;
  const num = parseInt(match[1]);
  return `L${num.toString().padStart(2, "0")}`;
}

// Map doc filenames to lesson IDs
// lesson00-basic-chat.md -> L00
// lesson13-full-agent.md -> L13
function docFilenameToLessonId(filename: string): string | null {
  const match = filename.match(/^lesson(\d+)/);
  if (!match) return null;
  const num = parseInt(match[1]);
  return `L${num.toString().padStart(2, "0")}`;
}

// Extract public Rust types from source
function extractClasses(
  lines: string[]
): { name: string; startLine: number; endLine: number }[] {
  const classes: { name: string; startLine: number; endLine: number }[] = [];
  const classPattern = /(?:pub\s+)?(?:struct|enum|trait)\s+(\w+)/;

  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(classPattern);
    if (m) {
      const name = m[1];
      const startLine = i + 1;
      // Find end of class by matching braces
      let braceCount = 0;
      let endLine = lines.length;
      for (let j = i; j < lines.length; j++) {
        for (const ch of lines[j]) {
          if (ch === "{") braceCount++;
          if (ch === "}") braceCount--;
        }
        if (braceCount <= 0 && j > i) {
          endLine = j + 1;
          break;
        }
      }
      classes.push({ name, startLine, endLine });
    }
  }
  return classes;
}

// Extract functions from Rust source
function extractMethods(
  lines: string[]
): { name: string; signature: string; startLine: number }[] {
  const methods: { name: string; signature: string; startLine: number }[] = [];
  const methodPattern = /(?:pub\s+)?(?:async\s+)?fn\s+(\w+)\s*\(([^)]*)\)/;

  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(methodPattern);
    if (m && !["if", "while", "for", "switch", "catch", "class", "new"].includes(m[1])) {
      methods.push({
        name: m[1],
        signature: m[0].trim(),
        startLine: i + 1,
      });
    }
  }
  return methods;
}

// Extract tool names from Rust source
// Looks for "name" fields in tool definitions
function extractTools(source: string): string[] {
  const tools = new Set<string>();
  // Match patterns like ToolSpec::object("tool_name") or "name": "tool_name"
  const patterns = [
    /ToolSpec::object\(\s*"(\w+)"/g,
    /"name"\s*:\s*"(\w+)"/g,
  ];
  for (const pattern of patterns) {
    let m;
    while ((m = pattern.exec(source)) !== null) {
      tools.add(m[1]);
    }
  }
  return Array.from(tools);
}

// Count non-blank, non-comment lines
function countLoc(lines: string[]): number {
  return lines.filter((line) => {
    const trimmed = line.trim();
    return trimmed !== "" && !trimmed.startsWith("//") && !trimmed.startsWith("*") && trimmed !== "/*" && trimmed !== "*/";
  }).length;
}

function main() {
  console.log("Extracting content from lessons and docs...");
  console.log(`  Repo root: ${REPO_ROOT}`);
  console.log(`  Lessons dir: ${LESSONS_DIR}`);
  console.log(`  Docs dir: ${DOCS_DIR}`);

  // Skip extraction if source directories don't exist
  if (!fs.existsSync(LESSONS_DIR)) {
    console.log("  Lessons directory not found, skipping extraction.");
    console.log("  Using pre-committed generated data.");
    return;
  }

  // 1. Read all lesson files
  const lessonFiles = fs
    .readdirSync(LESSONS_DIR)
    .filter((f) => f.match(/^lesson\d+_.*\.rs$/));

  console.log(`  Found ${lessonFiles.length} lesson files`);

  const versions: LessonVersion[] = [];

  for (const filename of lessonFiles) {
    const lessonId = filenameToLessonId(filename);
    if (!lessonId) {
      console.warn(`  Skipping ${filename}: could not determine lesson ID`);
      continue;
    }

    const filePath = path.join(LESSONS_DIR, filename);
    const source = fs.readFileSync(filePath, "utf-8");
    const lines = source.split("\n");

    const meta = LESSON_META[lessonId];
    const classes = extractClasses(lines);
    const methods = extractMethods(lines);
    const tools = extractTools(source);
    const loc = countLoc(lines);

    versions.push({
      id: lessonId,
      filename,
      title: meta?.title ?? lessonId,
      subtitle: meta?.subtitle ?? "",
      loc,
      tools,
      newTools: [],
      coreAddition: meta?.coreAddition ?? "",
      keyInsight: meta?.keyInsight ?? "",
      classes,
      methods,
      layer: meta?.layer ?? "tools",
      source,
    });
  }

  // Sort versions according to LESSON_ORDER
  const orderMap = new Map(LESSON_ORDER.map((v, i) => [v, i]));
  versions.sort(
    (a, b) => (orderMap.get(a.id as any) ?? 99) - (orderMap.get(b.id as any) ?? 99)
  );

  // 2. Compute newTools for each version
  for (let i = 0; i < versions.length; i++) {
    const prev = i > 0 ? new Set(versions[i - 1].tools) : new Set<string>();
    versions[i].newTools = versions[i].tools.filter((t) => !prev.has(t));
  }

  // 3. Compute diffs between adjacent lessons
  const diffs: LessonDiff[] = [];
  const versionMap = new Map(versions.map((v) => [v.id, v]));

  for (let i = 1; i < LEARNING_PATH.length; i++) {
    const fromId = LEARNING_PATH[i - 1];
    const toId = LEARNING_PATH[i];
    const fromVer = versionMap.get(fromId);
    const toVer = versionMap.get(toId);

    if (!fromVer || !toVer) continue;

    const fromClassNames = new Set(fromVer.classes.map((c) => c.name));
    const fromMethodNames = new Set(fromVer.methods.map((f) => f.name));
    const fromToolNames = new Set(fromVer.tools);

    diffs.push({
      from: fromId,
      to: toId,
      newClasses: toVer.classes
        .map((c) => c.name)
        .filter((n) => !fromClassNames.has(n)),
      newMethods: toVer.methods
        .map((f) => f.name)
        .filter((n) => !fromMethodNames.has(n)),
      newTools: toVer.tools.filter((t) => !fromToolNames.has(t)),
      locDelta: toVer.loc - fromVer.loc,
    });
  }

  // 4. Read doc files from locale subdirectories (en/, zh/)
  const docs: DocContent[] = [];

  if (fs.existsSync(DOCS_DIR)) {
    const localeDirs = ["en", "zh"];
    let totalDocFiles = 0;

    for (const locale of localeDirs) {
      const localeDir = path.join(DOCS_DIR, locale);
      if (!fs.existsSync(localeDir)) continue;

      const docFiles = fs
        .readdirSync(localeDir)
        .filter((f) => f.endsWith(".md"));

      totalDocFiles += docFiles.length;

      for (const filename of docFiles) {
        const lessonId = docFilenameToLessonId(filename);
        if (!lessonId) {
          console.warn(`  Skipping doc ${locale}/${filename}: could not determine lesson ID`);
          continue;
        }

        const filePath = path.join(localeDir, filename);
        const content = fs.readFileSync(filePath, "utf-8");

        const titleMatch = content.match(/^#\s+(.+)$/m);
        const title = titleMatch ? titleMatch[1] : filename;

        docs.push({ version: lessonId, locale: locale as "en" | "zh", title, content });
      }
    }

    console.log(`  Found ${totalDocFiles} doc files across ${localeDirs.length} locales`);
  } else {
    console.warn(`  Docs directory not found: ${DOCS_DIR}`);
  }

  // 5. Write output
  fs.mkdirSync(OUT_DIR, { recursive: true });

  const index: LessonIndex = { versions, diffs };
  const indexPath = path.join(OUT_DIR, "versions.json");
  fs.writeFileSync(indexPath, JSON.stringify(index, null, 2));
  console.log(`  Wrote ${indexPath}`);

  const docsPath = path.join(OUT_DIR, "docs.json");
  fs.writeFileSync(docsPath, JSON.stringify(docs, null, 2));
  console.log(`  Wrote ${docsPath}`);

  // Summary
  console.log("\nExtraction complete:");
  console.log(`  ${versions.length} versions`);
  console.log(`  ${diffs.length} diffs`);
  console.log(`  ${docs.length} docs`);
  for (const v of versions) {
    console.log(
      `    ${v.id}: ${v.loc} LOC, ${v.tools.length} tools, ${v.classes.length} classes, ${v.methods.length} methods`
    );
  }
}

main();
