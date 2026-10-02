import { useCallback, useEffect, useState } from "react";
import { Bookmark, BookmarkCheck, KanbanSquare, Loader2 } from "lucide-react";
import { cn } from "@/lib/utils";
import { IssueCard } from "@/components/issue-card";
import {
  listBoards,
  listSavedBoards,
  saveBoard,
  unsaveBoard,
  getBoardIssues,
  type Board,
  type SavedBoard,
  type IssueView,
} from "@/services/ipc";
import { useWorkspaceStore } from "@/stores/workspace-store";

export function BoardsView(): React.JSX.Element {
  const info = useWorkspaceStore((s) => s.info);
  const isConnected = info.connectionStatus === "connected";

  const [boards, setBoards] = useState<Board[]>([]);
  const [saved, setSaved] = useState<SavedBoard[]>([]);
  const [selectedBoardId, setSelectedBoardId] = useState<number | null>(null);
  const [boardIssues, setBoardIssues] = useState<IssueView[]>([]);
  const [loading, setLoading] = useState(false);
  const [issuesLoading, setIssuesLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const savedIds = new Set(saved.map((b) => b.boardId));

  const loadBoards = useCallback(async (): Promise<void> => {
    if (!isConnected) return;
    setLoading(true);
    setError(null);
    try {
      const [allBoards, savedBoards] = await Promise.all([
        listBoards(),
        listSavedBoards(),
      ]);
      setBoards(allBoards);
      setSaved(savedBoards);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, [isConnected]);

  useEffect(() => {
    void loadBoards();
  }, [loadBoards]);

  const handleSave = async (board: Board): Promise<void> => {
    try {
      await saveBoard({
        boardId: board.id,
        name: board.name,
        boardType: board.boardType,
        projectKey: board.projectKey,
      });
      setSaved((prev) => [
        ...prev,
        {
          boardId: board.id,
          name: board.name,
          boardType: board.boardType,
          projectKey: board.projectKey,
        },
      ]);
    } catch (e) {
      setError(String(e));
    }
  };

  const handleUnsave = async (boardId: number): Promise<void> => {
    try {
      await unsaveBoard(boardId);
      setSaved((prev) => prev.filter((b) => b.boardId !== boardId));
    } catch (e) {
      setError(String(e));
    }
  };

  const handleSelectBoard = async (boardId: number): Promise<void> => {
    setSelectedBoardId(boardId);
    setIssuesLoading(true);
    setError(null);
    try {
      const issues = await getBoardIssues(boardId);
      setBoardIssues(issues);
    } catch (e) {
      setError(String(e));
      setBoardIssues([]);
    } finally {
      setIssuesLoading(false);
    }
  };

  const selectedBoard = boards.find((b) => b.id === selectedBoardId) ?? null;

  if (!isConnected) {
    return (
      <main className="min-w-0 px-8 py-7">
        <h1 className="text-xl font-bold">Boards</h1>
        <div className="mt-8 rounded-lg border bg-card p-12 text-center">
          <p className="text-muted-foreground">
            Connect to Jira in Settings to browse boards.
          </p>
        </div>
      </main>
    );
  }

  return (
    <main className="min-w-0 px-8 py-7">
      <div className="flex items-center justify-between">
        <h1 className="text-xl font-bold">Boards</h1>
        <button
          className="flex items-center gap-2 rounded-md bg-primary px-3 py-1.5 text-sm font-medium text-primary-foreground disabled:opacity-50"
          disabled={loading}
          onClick={() => void loadBoards()}
        >
          {loading ? (
            <Loader2 className="animate-spin" size={14} />
          ) : (
            <KanbanSquare size={14} />
          )}
          Refresh
        </button>
      </div>

      {error ? (
        <div className="mt-4 rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
          {error}
        </div>
      ) : null}

      {/* Saved boards quick access */}
      {saved.length > 0 ? (
        <section className="mt-6">
          <h2 className="mb-3 text-sm font-semibold text-muted-foreground">
            SAVED BOARDS
          </h2>
          <div className="flex flex-wrap gap-2">
            {saved.map((sb) => (
              <button
                className={cn(
                  "flex items-center gap-2 rounded-lg border px-3 py-2 text-sm transition",
                  selectedBoardId === sb.boardId
                    ? "border-primary bg-primary/10 text-primary"
                    : "bg-card hover:border-primary/40",
                )}
                key={sb.boardId}
                onClick={() => void handleSelectBoard(sb.boardId)}
              >
                <BookmarkCheck size={14} />
                {sb.name}
                <span className="text-xs text-muted-foreground">
                  {sb.boardType}
                </span>
              </button>
            ))}
          </div>
        </section>
      ) : null}

      {/* Board issues */}
      {selectedBoard ? (
        <section className="mt-6">
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-lg font-semibold">{selectedBoard.name}</h2>
            <span className="text-xs text-muted-foreground">
              {boardIssues.length} issues
            </span>
          </div>
          {issuesLoading ? (
            <p className="py-8 text-center text-muted-foreground">
              Loading board issues...
            </p>
          ) : boardIssues.length === 0 ? (
            <p className="py-8 text-center text-muted-foreground">
              No issues on this board.
            </p>
          ) : (
            <div className="grid grid-cols-2 gap-4">
              {boardIssues.map((issue) => (
                <IssueCard issue={issue} key={issue.id} />
              ))}
            </div>
          )}
        </section>
      ) : null}

      {/* All boards list */}
      <section className="mt-8">
        <h2 className="mb-3 text-sm font-semibold text-muted-foreground">
          ALL BOARDS
        </h2>
        {loading ? (
          <p className="py-8 text-center text-muted-foreground">
            Loading boards...
          </p>
        ) : boards.length === 0 ? (
          <p className="py-8 text-center text-muted-foreground">
            No boards found.
          </p>
        ) : (
          <div className="grid grid-cols-3 gap-3">
            {boards.map((board) => (
              <div
                className={cn(
                  "cursor-pointer rounded-lg border bg-card p-4 transition hover:border-primary/40 hover:shadow-sm",
                  selectedBoardId === board.id && "border-primary",
                )}
                key={board.id}
                onClick={() => void handleSelectBoard(board.id)}
              >
                <div className="flex items-start justify-between">
                  <div>
                    <p className="font-medium">{board.name}</p>
                    <p className="mt-1 text-xs text-muted-foreground">
                      {board.boardType}
                      {board.projectKey ? ` · ${board.projectKey}` : ""}
                    </p>
                  </div>
                  <button
                    className="text-muted-foreground hover:text-primary"
                    onClick={(e) => {
                      e.stopPropagation();
                      void (savedIds.has(board.id)
                        ? handleUnsave(board.id)
                        : handleSave(board));
                    }}
                    title={
                      savedIds.has(board.id)
                        ? "Remove from saved"
                        : "Save board"
                    }
                  >
                    {savedIds.has(board.id) ? (
                      <BookmarkCheck size={16} />
                    ) : (
                      <Bookmark size={16} />
                    )}
                  </button>
                </div>
              </div>
            ))}
          </div>
        )}
      </section>
    </main>
  );
}
