import { useCallback, useEffect, useState } from "react";
import {
  ChevronDown,
  Loader2,
  MessageSquare,
  MoveRight,
  Send,
} from "lucide-react";
import { cn } from "@/lib/utils";
import {
  getTransitions,
  transitionIssue,
  addComment,
  type IssueTransition,
} from "@/services/ipc";

interface QuickActionsProps {
  issueKey: string;
  onActionComplete?: (() => void) | undefined;
}

export function QuickActions({
  issueKey,
  onActionComplete,
}: QuickActionsProps): React.JSX.Element {
  const [showTransitions, setShowTransitions] = useState(false);
  const [showComment, setShowComment] = useState(false);
  const [transitions, setTransitions] = useState<IssueTransition[]>([]);
  const [loadingTransitions, setLoadingTransitions] = useState(false);
  const [transitioning, setTransitioning] = useState<string | null>(null);
  const [commentText, setCommentText] = useState("");
  const [submittingComment, setSubmittingComment] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const loadTransitions = useCallback(async (): Promise<void> => {
    setLoadingTransitions(true);
    setError(null);
    try {
      const data = await getTransitions(issueKey);
      setTransitions(data);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoadingTransitions(false);
    }
  }, [issueKey]);

  useEffect(() => {
    if (showTransitions && transitions.length === 0) {
      void loadTransitions();
    }
  }, [showTransitions, transitions.length, loadTransitions]);

  const handleTransition = async (t: IssueTransition): Promise<void> => {
    setTransitioning(t.id);
    setError(null);
    try {
      await transitionIssue(issueKey, t.id);
      setShowTransitions(false);
      onActionComplete?.();
    } catch (e) {
      setError(String(e));
    } finally {
      setTransitioning(null);
    }
  };

  const handleComment = async (): Promise<void> => {
    if (!commentText.trim()) return;
    setSubmittingComment(true);
    setError(null);
    try {
      await addComment(issueKey, commentText.trim());
      setCommentText("");
      setShowComment(false);
      onActionComplete?.();
    } catch (e) {
      setError(String(e));
    } finally {
      setSubmittingComment(false);
    }
  };

  return (
    <div className="mt-3 border-t pt-3">
      <div className="flex items-center gap-2">
        <button
          className={cn(
            "flex items-center gap-1.5 rounded-md border px-2.5 py-1 text-xs transition",
            showTransitions
              ? "border-primary bg-primary/10 text-primary"
              : "text-muted-foreground hover:border-primary/40 hover:text-foreground",
          )}
          onClick={() => {
            setShowTransitions((v) => !v);
            setShowComment(false);
          }}
        >
          <MoveRight size={12} />
          Move
          <ChevronDown size={10} />
        </button>
        <button
          className={cn(
            "flex items-center gap-1.5 rounded-md border px-2.5 py-1 text-xs transition",
            showComment
              ? "border-primary bg-primary/10 text-primary"
              : "text-muted-foreground hover:border-primary/40 hover:text-foreground",
          )}
          onClick={() => {
            setShowComment((v) => !v);
            setShowTransitions(false);
          }}
        >
          <MessageSquare size={12} />
          Comment
        </button>
      </div>

      {error ? <p className="mt-2 text-xs text-destructive">{error}</p> : null}

      {/* Transitions dropdown */}
      {showTransitions ? (
        <div className="mt-2 rounded-md border bg-background p-2">
          {loadingTransitions ? (
            <div className="flex items-center gap-2 py-2 text-xs text-muted-foreground">
              <Loader2 className="animate-spin" size={12} />
              Loading transitions...
            </div>
          ) : transitions.length === 0 ? (
            <p className="py-2 text-xs text-muted-foreground">
              No available transitions.
            </p>
          ) : (
            <div className="space-y-1">
              {transitions.map((t) => (
                <button
                  className="flex w-full items-center gap-2 rounded-md px-2.5 py-1.5 text-left text-xs hover:bg-muted disabled:opacity-50"
                  disabled={transitioning !== null}
                  key={t.id}
                  onClick={() => void handleTransition(t)}
                >
                  {transitioning === t.id ? (
                    <Loader2 className="animate-spin" size={12} />
                  ) : (
                    <MoveRight size={12} />
                  )}
                  {t.name}
                </button>
              ))}
            </div>
          )}
        </div>
      ) : null}

      {/* Comment form */}
      {showComment ? (
        <div className="mt-2 space-y-2">
          <textarea
            className="w-full rounded-md border bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus:border-primary focus:ring-1 focus:ring-primary focus:outline-none"
            disabled={submittingComment}
            onChange={(e) => setCommentText(e.target.value)}
            placeholder="Add a comment..."
            rows={3}
            value={commentText}
          />
          <div className="flex justify-end">
            <button
              className="flex items-center gap-1.5 rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground disabled:opacity-50"
              disabled={submittingComment || !commentText.trim()}
              onClick={() => void handleComment()}
            >
              {submittingComment ? (
                <Loader2 className="animate-spin" size={12} />
              ) : (
                <Send size={12} />
              )}
              Send
            </button>
          </div>
        </div>
      ) : null}
    </div>
  );
}
