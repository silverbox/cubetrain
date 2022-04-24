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
  bookmark: string;
}

export class RotateStepManager {
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

  getActiveIdx = (): number => {
    return this.rotatingIdx;
  }

  getActiveStep = (): RotateStep | undefined => {
    const statrIdx = this.rotatingIdx < 0 ? 0 : this.rotatingIdx;
    for (let wkIdx = statrIdx; wkIdx < this.currentStepList.length; wkIdx++) {
      const wkStep = this.currentStepList[wkIdx];
      if (wkStep.rotateStatus == "done") continue;
      this.rotatingIdx = wkIdx;
      return wkStep;
    }
    return undefined;
  }

  setBookmark = (idx: number, bookmark: string) => {
    this.currentStepList[idx].bookmark = bookmark;
  }

  setRotateStatus = (rotateStatus: RotateStatus) => {
    this.currentStepList[this.rotatingIdx].rotateStatus = rotateStatus;
  }

  addStep = (axis: Axis, layer: Layer, dir: Dir, bookmark: string): RotateStep => {
    const step: RotateStep = {
      rid: uuidv4(),
      axis: axis,
      layer: layer,
      dir: dir,
      symbolMark: getRotateSymbol(axis, layer, dir),
      rotateStatus: "bef",
      bookmark: bookmark
    }
    this.currentStepList.push(step);
    return step;
  };

  revertRotateStatus = (beginIdx: number) => {
    for (let wkIdx = 0; wkIdx < beginIdx; wkIdx++) {
      this.currentStepList[wkIdx].rotateStatus = "done";
    }
    for (let wkIdx = beginIdx + 1; wkIdx < this.currentStepList.length; wkIdx++) {
      this.currentStepList[wkIdx].rotateStatus = "bef";
    }
    this.rotatingIdx = beginIdx + 1;
  };

  revertOneRotateStep = (): RotateInfo | undefined => {
    if (this.rotatingIdx < 0 || this.rotatingIdx >= this.currentStepList.length) {
      return undefined;
    }
    this.currentStepList[this.rotatingIdx].rotateStatus = "bef";
    const revertStep = this.currentStepList[this.rotatingIdx];
    this.rotatingIdx -= 1;
    return {
      axis: revertStep.axis,
      layer: revertStep.layer,
      dir: revertStep.dir == "p" ? "n" : "p"
    }
  };

  revertStep = (): RotateInfo | undefined => {
    const removedStepList = this.currentStepList.splice(-1, 1);
    if (removedStepList.length == 0) {
      return undefined;
    }
    const removedStep = removedStepList[0];
    if (removedStep.rotateStatus != "done") {
      return undefined;
    }
    this.rotatingIdx -= 1;
    return {
      axis: removedStep.axis,
      layer: removedStep.layer,
      dir: removedStep.dir == "p" ? "n" : "p"
    }
  }

  getStepListStr = (): string => {
    let retStr = "";
    for (const step of this.currentStepList) {
      retStr += this.getStepStr(step);
      if (step.bookmark != "") retStr += (" " + step.bookmark);
      retStr += "\n";
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