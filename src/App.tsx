import React, { CSSProperties } from 'react';
import './App.css';

type AppProps = {};

type Node = {
  cost?: number;
  estm_cost?: number;
  from?: {x: number; y: number};
  traversable: boolean;
};

type AppState = {
  dijkstra?: (sx: number, sy: number, width: number, height: number, map: Uint8Array) => string;
  grid?: Node[][];
  sx: number;
  sy: number;
  lastT?: number;
  renderTimeout?: number;
};

const NODE_STYLE: CSSProperties = {
  width: '1.5em',
  height: '1.5em',
  display: 'inline-block',
  border: '1px solid lightgray',
  padding: 0,
  margin: 0,
  verticalAlign: 'bottom',
};

const UNTRAVERSABLE_NODE_STYLE: CSSProperties = {
  ...NODE_STYLE,
  background: 'gray',
};

const TIMEOUT_TIME = 0;

class App extends React.Component<AppProps, AppState> {
  constructor(props: AppProps) {
    super(props);
    const grid = [];
    const width = 50;
    const height = 50;
    for (let x = 0; x < width; x += 1) {
      const line = [];
      for (let y = 0; y < height; y += 1) {
        line.push({
          traversable: true,
        });
      }
      grid.push(line);
    }
    this.state = { sx: 1, sy: 1, grid };
  }

  componentDidMount() {
    this.loadWasm();
  }

  loadWasm = async () => {
    try {
      const { dijkstra } = await import('kai_rust_wasm_play');
      this.setState({ dijkstra });
      this.updatePathFinding();
    } catch (err) {
      console.error(`Unexpected error in loadWasm. [Message: ${err.message}]`);
    }
  };

  updatePathFinding = () => {
    const p = performance.now();
    const {
      dijkstra, sx, sy, grid, renderTimeout,
    } = this.state;
    if (!dijkstra || !grid) return;
    const map = grid.map((line) => line.map((node) => (node.traversable ? 1 : 0)));
    const width = map.length;
    const height = map[0].length;
    const result = dijkstra(sx, sy, width, height, new Uint8Array(map.flat()));
    const solvedGrid = JSON.parse(result) as Node[][];

    if (renderTimeout) clearTimeout(renderTimeout);
    const newRenderTimeout = window.setTimeout(() => {
      this.setState({ grid: solvedGrid });
    }, TIMEOUT_TIME);
    this.setState({ lastT: performance.now() - p, renderTimeout: newRenderTimeout });
  };

  setTraversable = (e: React.MouseEvent<HTMLSpanElement>, x: number, y: number) => {
    e.preventDefault();
    const { grid, renderTimeout } = this.state;
    if (!grid) return;
    grid[x][y].traversable = !grid[x][y].traversable;
    this.setState({ grid }, this.updatePathFinding);
  };

  setGoal = (sx: number, sy: number) => {
    this.setState({ sx, sy }, this.updatePathFinding);
  };

  render() {
    const {
      lastT, grid, sx, sy,
    } = this.state;
    return (
      <div className="App">
        {lastT && (
        <div>
          Last Calculation time =
          {' '}
          <pre>
            {lastT.toPrecision(2)}
            ms
          </pre>
        </div>
        )}
        {grid && grid.map((line, x) => (
          <div key={`line-${x}`}>
            {line.map((node, y) => (
              <span
                style={node.traversable ? NODE_STYLE : UNTRAVERSABLE_NODE_STYLE}
                key={`${x},${y}`}
                onContextMenu={(e) => { this.setTraversable(e, x, y); }}
                onClick={(e) => { this.setGoal(x, y); }}
              >
                {!node.traversable && '✕'}
                {node.from && node.from.x < x && '↑'}
                {node.from && node.from.y < y && '←'}
                {node.from && node.from.x > x && '↓'}
                {node.from && node.from.y > y && '→'}
                {x == sx && y == sy && '★'}
              </span>
            ))}
          </div>
        ))}
      </div>
    );
  }
}

export default App;
