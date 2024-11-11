import { useEffect, useRef, useState } from "react";
import { TickTakGrid } from "./components/tick-tak-grid";
import { TickTackGridValues } from "./types/tick-tack";

type WebSocketMessage =
  | {
      type: "user_id";
      userId: string;
    }
  | {
      type: "initial_match_data";
      board: Array<TickTackGridValues>;
      turn: "Circle" | "Cross";
      mark: "Circle" | "Cross";
    }
  | {
      type: "move";
      board: Array<TickTackGridValues>;
      turn: "Circle" | "Cross";
      activeGrid: { x: number; y: number };
    };

function App() {
  const userId = useRef<string | null>(null);
  const webSocketRef = useRef<WebSocket | null>(null);
  const [activeGrid, setActiveGrid] = useState({ x: 1, y: 1 });

  const [playerMark, setPlayerMark] = useState<"Circle" | "Cross" | null>(null);
  const [activePlayer, setActivePlayer] = useState<"Circle" | "Cross" | null>(
    null
  );

  const [metaGrid, setMetaGrid] = useState<Array<TickTackGridValues> | null>(
    null
  );

  useEffect(() => {
    if (webSocketRef.current === null) {
      const websocket = new WebSocket(
        "ws://127.0.0.1:8080/match/" + prompt("Enter match id")
      );
      websocket.onopen = () => {
        console.log("connected");
      };
      websocket.onmessage = (event) => {
        const data = JSON.parse(event.data) as WebSocketMessage;
        console.log(data);

        switch (data.type) {
          case "user_id":
            userId.current = data.userId;
            break;
          case "initial_match_data":
            setMetaGrid(data.board);
            setActivePlayer(data.turn);
            setPlayerMark(data.mark);
            break;
          case "move":
            setMetaGrid(data.board);
            setActivePlayer(data.turn);
            setActiveGrid(data.activeGrid);
            break;
          default:
            console.warn("Unknown message type: ", "Never");
        }
      };
      websocket.onclose = () => {
        alert("Connection closed");
      };
      webSocketRef.current = websocket;
    }
  }, []);

  useEffect(() => {
    if (metaGrid === null) return;

    const checkWinner = (grid: TickTackGridValues) => {
      const winningCombinations = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
      ];

      for (const combination of winningCombinations) {
        const [a, b, c] = combination;
        if (grid[a] !== "Empty" && grid[a] === grid[b] && grid[a] === grid[c]) {
          return grid[a];
        }
      }
      return null;
    };

    for (let i = 0; i < 9; i++) {
      const winner = checkWinner(metaGrid[i]);
      if (winner) {
        alert(`Winner: ${winner}`);
      } else if (i === activeGrid.y * 3 + activeGrid.x) {
        if (metaGrid[i].every((cell) => cell !== "Empty")) {
          alert("Draw");
        }
      }
    }
  }, [activeGrid.x, activeGrid.y, metaGrid]);

  if (metaGrid === null || activePlayer === null || playerMark === null) {
    return <div>Waiting...</div>;
  }

  return (
    <div className="flex flex-col items-center h-screen">
      <div className="grid grid-cols-3 grid-rows-3 gap-0 justify-center items-center aspect-square h-[calc(100vh_-_80px)]">
        {Array.from({ length: 9 }).map((_, index) => {
          const cord = { x: index % 3, y: Math.floor(index / 3) };
          const isActive = activeGrid.x === cord.x && activeGrid.y === cord.y;
          return (
            <TickTakGrid
              key={index}
              active={isActive}
              playersTurn={activePlayer === playerMark}
              onClick={(x, y) => {
                if (!isActive) return;

                console.log("sending move", index, y * 3 + x);
                webSocketRef.current?.send(
                  JSON.stringify({
                    type: "move",
                    grid: index,
                    x,
                    y,
                  })
                );

                // setActiveGrid({ x, y });
                // setPlayer((prev) => (prev === "Circle" ? "Cross" : "Circle"));
                // setMetaGrid((prevMetaGrid) => {
                //   if (prevMetaGrid === null) return prevMetaGrid;

                //   const newMetaGrid = [...prevMetaGrid];
                //   newMetaGrid[index][] = player;

                //   console.log("x");
                //   webSocketRef.current?.send(
                //     JSON.stringify({ newMetaGrid, player })
                //   );
                //   return newMetaGrid;
                // });
              }}
              grid={metaGrid[index]}
            />
          );
        })}
      </div>
      <div className="mb-4 p-4">
        <h1
          className="text-red-500 font-bold text-5xl data-[you=true]:text-green-600"
          data-you={activePlayer === playerMark}
        >
          Player: {activePlayer} {activePlayer === playerMark ? "(You)" : ""}
        </h1>
      </div>
    </div>
  );
}

export default App;
