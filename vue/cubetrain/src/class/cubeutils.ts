export type Axis = "x" | "y" | "z";
export type Layer = "all" | "pos" | "neu" | "neg";
export type Dir = "p" | "n";
export type Symbol = "x" | "y" | "z" | "R" | "M" | "L" | "U" | "E" | "D" | "F" | "S" | "B";
export type RevMark = "'" | "";
export type RotateStatus = "bef" | "doing" | "done";

export interface RotateInfo {
  axis: Axis;
  layer: Layer;
  dir: Dir;
}

export interface SymbolMark {
  symbol: Symbol;
  mark: RevMark;
}

export const cubeutils = () => {
  const getRotateSymbol = (axis: Axis, layer: Layer, dir: Dir): SymbolMark => {
    switch (axis) {
      case "x":
        switch (layer) {
          case "all": return { symbol: "x", mark: _getReverseMark(dir, false)};
          case "pos": return { symbol: "R", mark: _getReverseMark(dir, false)};
          case "neu": return { symbol: "M", mark: _getReverseMark(dir, false)};
          case "neg": return { symbol: "L", mark: _getReverseMark(dir, true)};
        }
        break; // workaround
      case "y":
        switch (layer) {
          case "all": return { symbol: "y", mark: _getReverseMark(dir, false)};
          case "pos": return { symbol: "U", mark: _getReverseMark(dir, false)};
          case "neu": return { symbol: "E", mark: _getReverseMark(dir, true)};
          case "neg": return { symbol: "D", mark: _getReverseMark(dir, true)};
        }
        break; // workaround
      case "z":
        switch (layer) {
          case "all": return { symbol: "z", mark: _getReverseMark(dir, false)};
          case "pos": return { symbol: "F", mark: _getReverseMark(dir, false)};
          case "neu": return { symbol: "S", mark: _getReverseMark(dir, false)};
          case "neg": return { symbol: "D", mark: _getReverseMark(dir, true)};
        }
        break; // workaround
    }
  }

  const _getReverseMark = (dir: Dir, alt: boolean): RevMark => {
    if (dir == "p") {
      return alt ? "" : "'";
    } else {
      return alt ? "'" : "";
    }
  }

  const getSymbolFromKey = (key: string): Symbol | undefined => {
    const lk = key.toLowerCase();
    switch (lk) {
      case "x": return "x";
      case "y": return "y";
      case "z": return "z";
      case "r": return "R";
      case "u": return "U";
      case "f": return "F";
      case "m": return "M";
      case "e": return "E";
      case "s": return "S";
      case "l": return "L";
      case "d": return "D";
      case "b": return "B";
    }
    return undefined;
  }

  const _getRandomInt = (max: number) => {
    return Math.floor(Math.random() * max);
  }
  const _getRandAxis = (): Axis => {
    const rndAxis = _getRandomInt(3);
    switch (rndAxis) {
      case 1: return "x";
      case 2: return "y";
      default: return "z";
    }
  }
  const _getRandLayer = (): Layer => {
    const rndAxis = _getRandomInt(4);
    switch (rndAxis) {
      case 0: return "all";
      case 1: return "pos";
      case 2: return "neu";
      default: return "neg";
    }
  }
  const _getRandDir = (): Dir => {
    const rndAxis = _getRandomInt(4);
    switch (rndAxis) {
      case 0: return "p";
      default: return "n";
    }
  }
  const getRandomRotateInfo = (): RotateInfo => {
    return { axis: _getRandAxis(), layer: _getRandLayer(), dir: _getRandDir() };
  }

  const getRotateInfo = (symbol: Symbol, alt: boolean): RotateInfo => {
    switch (symbol) {
      case "x": return { axis: "x", layer: "all", dir: _getDirSub(symbol, alt) };
      case "y": return { axis: "y", layer: "all", dir: _getDirSub(symbol, alt) };
      case "z": return { axis: "z", layer: "all", dir: _getDirSub(symbol, alt) };
      case "R": return { axis: "x", layer: "pos", dir: _getDirSub(symbol, alt) };
      case "U": return { axis: "y", layer: "pos", dir: _getDirSub(symbol, alt) };
      case "F": return { axis: "z", layer: "pos", dir: _getDirSub(symbol, alt) };
      case "M": return { axis: "x", layer: "neu", dir: _getDirSub(symbol, alt) };
      case "E": return { axis: "y", layer: "neu", dir: _getDirSub(symbol, alt) };
      case "S": return { axis: "z", layer: "neu", dir: _getDirSub(symbol, alt) };
      case "L": return { axis: "x", layer: "neg", dir: _getDirSub(symbol, alt) };
      case "D": return { axis: "y", layer: "neg", dir: _getDirSub(symbol, alt) };
      case "B": return { axis: "z", layer: "neg", dir: _getDirSub(symbol, alt) };
    }
  };

  const getRotateInfoFromStr = (symbolMark: string): RotateInfo | undefined => {
    const symbol = getSymbolFromKey(symbolMark.substring(0, 1));
    if (symbol == undefined) {
      return undefined;
    }
    const alt = symbolMark.length > 1;
    return getRotateInfo(symbol, alt);
  };

  const _getDirSub = (symbol: Symbol, alt: boolean): Dir => {
    const isAltKey = (["E", "L", "D", "B"].includes(symbol));
    if (alt) {
      return isAltKey ? "n" : "p"; 
    } else {
      return isAltKey ? "p" : "n";
    }
  };

  return  {
    getRandomRotateInfo,
    getRotateSymbol,
    getRotateInfo,
    getRotateInfoFromStr,
    getSymbolFromKey
  }
}