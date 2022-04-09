import { v4 as uuidv4 } from 'uuid';
import { Axis, Layer, Dir, SymbolMark, cubeutils } from '@/class/cubeutils';
const { getRotateSymbol } = cubeutils();

export interface RotateStep {
  rid: string;
  axis: Axis;
  layer: Layer;
  dir: Dir;
  symbolMark: SymbolMark;
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

  addStep = (axis: Axis, layer: Layer, dir: Dir): RotateStep => {
    const step: RotateStep = {
      rid: uuidv4(),
      axis: axis,
      layer: layer,
      dir: dir,
      symbolMark: getRotateSymbol(axis, layer, dir)
    }
    this.currentStepList.push(step);
    return step;
  };

  getStepListStr = (): string => {
    let retStr = "";
    for (const step of this.currentStepList) {
      retStr += (this.getStepStr(step) + "\n");
    }
    return retStr;
  };

  getCurrentStepList = (): Array<RotateStep> => {
    return this.currentStepList;
  };

  getStepStr = (step: RotateStep): string => {
    return step.symbolMark.symbol + step.symbolMark.mark;
  };

}