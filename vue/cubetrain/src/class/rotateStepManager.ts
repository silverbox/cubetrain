import {v4 as uuidv4} from 'uuid';

export interface RotateStep {
  rid: string;
  axis: string;
  layer: string;
  dir: string;
  symbol: string;
}

interface RotateBookmark {
  name: string;
  steps: Array<RotateStep>;
}

export class RotateStepManager {
  bookMarkList: Array<RotateBookmark> = [];

  currentStepList: Array<RotateStep> = [];

  clearStepList = () => {
    this.currentStepList = [];
  }

  addStep = (axis: string, layer: string, dir: string): RotateStep => {
    const step: RotateStep = {
      rid: uuidv4(),
      axis: axis,
      layer: layer,
      dir: dir,
      symbol: this._getSymbol(axis, layer, dir)
    }
    this.currentStepList.push(step);
    return step;
  }

  getCurrentStepList = (): Array<RotateStep> => {
    return this.currentStepList;
  }

  _getSymbol = (axis: string, layer: string, dir: string): string => {
    switch (axis) {
      case "x":
        switch (layer) {
          case "all": return "x" + this._getReverseMark(dir, false);
          case "pos": return "R" + this._getReverseMark(dir, false);
          case "neu": return "M" + this._getReverseMark(dir, false);
          case "neg": return "L" + this._getReverseMark(dir, true);
        }
        break;
      case "y":
        switch (layer) {
          case "all": return "y" + this._getReverseMark(dir, false);
          case "pos": return "U" + this._getReverseMark(dir, false);
          case "neu": return "E" + this._getReverseMark(dir, true);
          case "neg": return "D" + this._getReverseMark(dir, true);
        }
        break;
      case "z":
        switch (layer) {
          case "all": return "z" + this._getReverseMark(dir, false);
          case "pos": return "F" + this._getReverseMark(dir, false);
          case "neu": return "S" + this._getReverseMark(dir, false);
          case "neg": return "B" + this._getReverseMark(dir, true);
        }
        break;
    }
    return ""
  }

  _getReverseMark = (dir: string, alt: boolean) => {
    if (dir == "p") {
      return alt ? "" : "'";
    } else {
      return alt ? "'" : "";
    }
  }
}