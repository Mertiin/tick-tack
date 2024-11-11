import styles from "./tick-tak-grid.module.css";

import circle from "../assets/circle.svg";
import cross from "../assets/x.svg";
import { TickTackGridValues } from "../types/tick-tack";

interface TickTakGridProps {
  active: boolean;
  playersTurn: boolean;
  onClick: (x: number, y: number) => void;
  grid: TickTackGridValues;
}

const TickTakGrid = ({
  active,
  onClick,
  grid,
  playersTurn,
}: TickTakGridProps) => {
  return (
    <div
      className={`${styles.tickTakGrid} data-[active=false]:opacity-40 bg-white`}
      data-active={active}
    >
      {Array.from({ length: 9 }).map((_, index) => {
        let char = "";
        if (grid[index] === "Circle") {
          char = circle;
        } else if (grid[index] === "Cross") {
          char = cross;
        }

        return (
          <div
            className="cursor-pointer data-[active=false]:cursor-default"
            key={index}
            onClick={() => {
              if (!playersTurn) return;
              if (grid[index] !== "Empty") return;

              onClick(index % 3, Math.floor(index / 3));
            }}
            data-active={active && playersTurn}
          >
            {char && <img src={char} alt="" className="w-full h-full" />}
          </div>
        );
      })}
    </div>
  );
};

export { TickTakGrid };
