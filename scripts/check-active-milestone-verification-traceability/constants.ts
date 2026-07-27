import {
  existsSync,
  readFileSync,
  readdirSync,
  statSync,
} from "node:fs";
import path from "node:path";

export const DEFAULT_ROOT_DIR = path.resolve(import.meta.dir, "../..");
export const ROADMAP_FILE = ".planning/ROADMAP.md";
export const REQUIREMENTS_FILE = ".planning/REQUIREMENTS.md";
export const ARCHIVED_V21_FILES = [
  ".planning/milestones/v2.1-ROADMAP.md",
  ".planning/milestones/v2.1-REQUIREMENTS.md",
  ".planning/milestones/v2.1-MILESTONE-AUDIT.md",
] as const;
export const PHASES_DIR = ".planning/phases";
export const ACTIVE_MILESTONE_HEADING = "## Active Milestone:";
export const REQUIREMENT_ID_PATTERN = "[A-Z]+-\\d+";

export type CheckActiveMilestoneVerificationTraceabilityOptions = {
  maybeRootDir?: string;
};

export type ActiveRequirement = {
  checked: boolean;
  id: string;
};

export type TraceabilityRow = {
  id: string;
  phase: number;
  status: "Complete" | "Pending";
};

export type LifecycleIdentity = {
  mode: string;
  phaseLifecycleId: string;
};

export type PhaseCorpus = {
  directory: string;
  lifecycle: LifecycleIdentity | null;
  phase: number;
  summaries: Artifact[];
  verifications: Artifact[];
};

export type Artifact = {
  frontmatter: string | null;
  relativePath: string;
  text: string;
};
