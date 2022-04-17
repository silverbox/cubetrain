import { v4 as uuidv4 } from 'uuid';
import { Axis, Layer, Dir, SymbolMark, RotateStatus, RotateInfo, cubeutils } from '@/class/cubeutils';
const { getRotateSymbol } = cubeutils();

export interface RotateStep {
  rid: string;
  axis: Axis;
  layer: Layer;
  dir: Dir;
  symbolMark: SymbolMark;
  rotateStatus: RotateStatus;
  bookMark: RotateBookmark | undefined;
}

interface RotateBookmark {
  stepIdx: number;
  name: string;
}

export class RotateStepManager {
  // bookMarkList: Array<RotateBookmark> = [];
  currentStepList: Array<RotateStep> = [];

  rotatingIdx = 0;

  clearStepList = () => {
    this.currentStepList = [];
    this.rotatingIdx = 0;
  }

  clearAllRotated = () => {
    this.rotatingIdx = 0;
    for (const step of this.currentStepList) {
      step.rotateStatus = "bef";
    }
  }

  addBookmark = (stepIdx: number, name: string) => {
    const bookMark = {
      name,
      stepIdx
    }
    this.currentStepList[stepIdx].bookMark = bookMark;
    // this.bookMarkList.push(bookMark);
  }

  revertStep = (idx:number): RotateInfo | undefined => {
    const removedStepList = this.currentStepList.splice(-1, 1);
    if (removedStepList.length == 0) {
      return undefined;
    }
    const removedStep = removedStepList[0];
    return {
      axis: removedStep.axis,
      layer: removedStep.layer,
      dir: removedStep.dir == "p" ? "n" : "p"
    }
  }

  revertRotated = (fromIdx: number) => {
    this.rotatingIdx = fromIdx;
    for (let wkIdx = this.rotatingIdx; wkIdx < this.currentStepList.length; wkIdx++) {
      this.currentStepList[wkIdx].rotateStatus = "bef";
    }
  }

  getActiveIdx = (): number => {
    return this.rotatingIdx;
  }

  getActiveStep = (): RotateStep | undefined => {
    for (let wkIdx = this.rotatingIdx; wkIdx < this.currentStepList.length; wkIdx++) {
      const wkStep = this.currentStepList[wkIdx];
      if (wkStep.rotateStatus == "done") continue;
      this.rotatingIdx = wkIdx;
      return wkStep;
    }
    return undefined;
  }

  setRotateStatus = (rotateStatus: RotateStatus) => {
    this.currentStepList[this.rotatingIdx].rotateStatus = rotateStatus;
  }

  addStep = (axis: Axis, layer: Layer, dir: Dir): RotateStep => {
    const step: RotateStep = {
      rid: uuidv4(),
      axis: axis,
      layer: layer,
      dir: dir,
      symbolMark: getRotateSymbol(axis, layer, dir),
      rotateStatus: "bef",
      bookMark: undefined
    }
    this.currentStepList.push(step);
    return step;
  };

  getStepListStr = (): string => {
    let retStr = "";
    for (const step of this.currentStepList) {
      retStr += (this.getStepStr(step) + this.getStepInfoStr(step) + "\n");
    }
    return retStr;
  };

  getCurrentStepList = (): Array<RotateStep> => {
    return this.currentStepList;
  };

  getStepInfoStr = (step: RotateStep): string => {
    return step.bookMark ? " " + step.bookMark.name : ""
  };

  getStepStr = (step: RotateStep): string => {
    return step.symbolMark.symbol + step.symbolMark.mark;
  };

}