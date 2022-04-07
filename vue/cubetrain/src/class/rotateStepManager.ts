import {v4 as uuidv4} from 'uuid';

export interface RotateStep {
  rid: string;
  axis: string;
  layer: string;
  dir: string;
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
      dir: dir
    }
    this.currentStepList.push(step);
    return step;
  }

  getCurrentStepList = (): Array<RotateStep> => {
    return this.currentStepList;
  }
}