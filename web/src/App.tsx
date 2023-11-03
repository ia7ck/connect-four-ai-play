import { forwardRef, useEffect, useRef, useState } from "react";

import init, {
  Data,
  start,
  legal_actions,
  advance,
  DropPiece,
} from "./pkg/game_ai";

const colors = {
  human: "#ffe0c1",
  ai: "#c1e0ff",
  legal: "#efefef",
} as const;

type Board = Data["board"];

function drawGameBoard(ctx: CanvasRenderingContext2D, board: Board) {
  for (let i = 0; i < board.length; i++) {
    for (let j = 0; j < board[i].length; j++) {
      const h = ctx.canvas.height / board.length;
      const w = ctx.canvas.width / board[i].length;
      const y = i * h;
      const x = j * w;
      ctx.clearRect(x, y, w, h);
      if (board[i][j] !== undefined) {
        ctx.beginPath();
        ctx.ellipse(x + w / 2, y + h / 2, w / 2, h / 2, 0, 0, 2 * Math.PI);
        if (board[i][j] === "First") {
          ctx.fillStyle = colors["human"];
        } else {
          ctx.fillStyle = colors["ai"];
        }
        ctx.fill();
      }
    }
  }
}

function drawLegalActions(
  ctx: CanvasRenderingContext2D,
  board: Board,
  actions: DropPiece[],
) {
  for (const action of actions) {
    const h = ctx.canvas.height / board.length;
    const w = ctx.canvas.width / board[action.y].length;
    const y = action.y * h;
    const x = action.x * w;
    ctx.beginPath();
    ctx.ellipse(x + w / 2, y + h / 2, w / 2, h / 2, 0, 0, 2 * Math.PI);
    ctx.fillStyle = colors["legal"];
    ctx.fill();
  }
}

function findPosition(
  ctx: CanvasRenderingContext2D,
  board: Board,
  ev: React.MouseEvent,
): null | { row: number; column: number } {
  // rect.width < ctx.canvas.width がありうる
  const rect = ctx.canvas.getBoundingClientRect();
  const y = ev.clientY - rect.y;
  const x = ev.clientX - rect.x;
  for (let i = 0; i < board.length; i++) {
    for (let j = 0; j < board[i].length; j++) {
      const h = rect.height / board.length;
      const w = rect.width / board[i].length;
      // (y, x) が長方形範囲内か
      if (i * h < y && y < (i + 1) * h && j * w < x && x < (j + 1) * w) {
        return { row: i, column: j };
      }
    }
  }
  return null;
}

function GameInfo({ data }: { data: Data }) {
  let info = null;
  const yourTurn = data.turn % 2 === 0;
  if (data.status === "Ongoing") {
    const circleClass = yourTurn
      ? "circle bg-ffe0c1 vertical-align-text-top margin-left-2"
      : "circle bg-c1e0ff vertical-align-text-top margin-left-2";
    info = (
      <p>
        turn: {yourTurn ? "You" : "CPU"}
        <span className={circleClass}></span>
      </p>
    );
  } else if (data.status === "LastPlayerWin") {
    info = <p>{yourTurn ? "You Lose..." : "You Win!!"}</p>;
  } else {
    info = <p>Draw</p>;
  }
  return info;
}

const Canvas = forwardRef<
  HTMLCanvasElement,
  { height: number; width: number; onClick: (ev: React.MouseEvent) => void }
>(({ height, width, onClick }, ref) => {
  return (
    <canvas
      id="canvas"
      height={height}
      width={width}
      onClick={onClick}
      ref={ref}
      className="border-solid-1px display-block"
    ></canvas>
  );
});

function App() {
  const [data, setData] = useState<null | Data>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  // wasm
  useEffect(() => {
    init()
      .then(() => {
        setData(start());
      })
      .catch((err) => {
        console.error("error;;;", err);
      });
  }, []);

  // canvas描画
  useEffect(() => {
    if (data === null) {
      return;
    }
    const ctx = canvasRef.current?.getContext("2d");
    if (!ctx) {
      return;
    }
    drawGameBoard(ctx, data.board);
    if (data.status === "Ongoing") {
      drawLegalActions(ctx, data.board, legal_actions(data));
    }
  }, [data, canvasRef]);

  function handleClick(ev: React.MouseEvent) {
    if (data === null) {
      return;
    }
    if (data.status !== "Ongoing") {
      return;
    }
    if (data.turn % 2 === 1) {
      // AI
      return;
    }
    const ctx = canvasRef.current?.getContext("2d");
    if (!ctx) {
      return;
    }
    const position = findPosition(ctx, data.board, ev);
    if (position === null) {
      return;
    }
    const action = { y: position.row, x: position.column };
    const legal = legal_actions(data).some(
      ({ y, x }) => y === action.y && x === action.x,
    );
    if (legal === false) {
      return;
    }
    const newData = advance(data, { kind: "Human", action });
    setData(newData);
    if (newData.status === "Ongoing") {
      // AI move
      setTimeout(
        async () => {
          const newNewData = advance(newData, { kind: "Ai" });
          // AIの打つ手が速すぎるのですこし待つ
          await new Promise((resolve) =>
            setTimeout(resolve, 1000 + Math.random() * Math.random() * 1000),
          );
          setData(newNewData);
        },
        10, // ???
      );
    }
  }

  return (
    <main>
      <nav className="justify-content-end margin-bottom-0">
        <a href="https://github.com/ia7ck/connect-four-ai-play" target="blank">
          GitHub
        </a>
      </nav>
      <h1>Connect 4</h1>
      {data && (
        <>
          <GameInfo data={data}></GameInfo>
          <Canvas
            height={data.h * 100}
            width={data.w * 100}
            onClick={handleClick}
            ref={canvasRef}
          ></Canvas>
        </>
      )}
    </main>
  );
}

export default App;
