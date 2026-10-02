export interface MockIssue {
  key: string;
  summary: string;
  project: string;
  status: "To Do" | "In Progress" | "Review" | "Done";
  priority: "Highest" | "High" | "Medium" | "Low";
  updatedMinutesAgo: number;
  dueLabel?: string;
}

const ISSUE_SEED: ReadonlyArray<MockIssue> = [
  {
    key: "TPT-7042",
    summary: "Update Snowplow tracking",
    project: "E-commerce",
    status: "In Progress",
    priority: "High",
    updatedMinutesAgo: 18,
    dueLabel: "Today",
  },
  {
    key: "PRCM-2579",
    summary: "Checkout component update",
    project: "Funnels",
    status: "Review",
    priority: "Medium",
    updatedMinutesAgo: 42,
  },
  {
    key: "TPT-7101",
    summary: "Fix mobile funnel issue",
    project: "E-commerce",
    status: "To Do",
    priority: "High",
    updatedMinutesAgo: 67,
    dueLabel: "Tomorrow",
  },
  {
    key: "TPT-6981",
    summary: "Update product selector",
    project: "Presells",
    status: "In Progress",
    priority: "Medium",
    updatedMinutesAgo: 125,
  },
];

export function buildMockIssues(count = ISSUE_SEED.length): Array<MockIssue> {
  return Array.from({ length: count }, (_, index) => {
    const source = ISSUE_SEED[index % ISSUE_SEED.length];
    if (source === undefined) {
      throw new Error("Fixture seed is empty");
    }
    const cycle = Math.floor(index / ISSUE_SEED.length);
    return {
      ...source,
      key: cycle === 0 ? source.key : `${source.key}-${cycle}`,
      summary: cycle === 0 ? source.summary : `${source.summary} ${cycle + 1}`,
      updatedMinutesAgo: source.updatedMinutesAgo + cycle,
    };
  });
}

export const mockIssues = buildMockIssues();
