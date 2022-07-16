<template>
  <v-app>
    <v-app-bar
      app
      color="deep-purple accent-4"
      dense
      dark
      priority=0
      ref="appBar"
    >
      <template v-slot:prepend>
        <v-app-bar-nav-icon @click="showConfigMenu = !showConfigMenu"></v-app-bar-nav-icon>
      </template>

      <v-toolbar-title>{{ appTitle }}</v-toolbar-title>

      <!-- <v-btn icon>
        <v-icon>mdi-heart</v-icon>
      </v-btn>

      <v-btn icon>
        <v-icon>mdi-magnify</v-icon>
      </v-btn> -->
      <template v-slot:append>
        <v-btn icon @click="(showHistory = !showHistory)">
          <v-icon>mdi-history</v-icon>
        </v-btn>
      </template>
    </v-app-bar>

    <v-navigation-drawer permanent v-if="showConfigMenu">
      <ControlPanel
        :defspeed=40
        :defscramblestep=24
        :defIsButtonPanelVisible="isButtonPanelVisible"
        :defCubeViewType="cubeViewType"
        @controlAction="onControlAction"
        @changeButtonPanelVisible="onChangeButtonPanelVisible"
        @changeCubeViewType="onChangeCubeViewType"
      />
    </v-navigation-drawer>

    <v-navigation-drawer permanent position="right" v-if="showHistory">
      <v-list density="compact">
        <v-list-subheader>操作履歴</v-list-subheader>
        <v-list-subheader>
          <v-btn width="30" flat @click="onPlaybackOneStep" class="app_playbackonestep">
            <v-icon>mdi-chevron-left</v-icon>
            <v-tooltip activator="parent" anchor="start"><div>１ステップ戻します</div></v-tooltip>
          </v-btn>
          <v-btn width="30" flat @click="onPlayforwardOneStep" class="app_playforwardonestep">
            <v-icon>mdi-chevron-right</v-icon>
            <v-tooltip activator="parent" anchor="start"><div>１ステップ進めます</div></v-tooltip>
          </v-btn>
          <v-btn
            flat
            ref="historyMenuButton"
            class="app_history-menu-btn"
          >
            <v-icon>mdi-dots-vertical</v-icon>
            <v-menu
              activator="parent"
              anchor="start"
              transition="scale-transition"
              close-on-click=true
            >
              <v-list>
                <v-list-item
                  v-for="menu in menuitems"
                  :key="menu.id"
                  :disabled="!isActiveItem(menu)"
                  @click="onMenuClick(menu.id)"
                  :class="'app_history-menu-item-' + menu.id"
                >
                  <v-list-item-title>{{ menu.caption }}</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>
          </v-btn>
        </v-list-subheader>
      </v-list>
      <v-list density="compact" class="app_history-list" ref="rotateStepListElem" :height="roteteStepListHeight" three-line>
        <v-list-item
          v-for="(step, i) in rotateStepList"
          :key="i"
          :value="step"
          :active-class="activeRotateStepClass"
          :class="getStepClass(step)"
          active-color="primary"
          @click="onRoteteStepClick($event, i)"
        >
          <v-list-item-header>
            <v-list-item-title v-text="getStepStr(step)"></v-list-item-title>
             <v-list-item-subtitle v-text="getStepSubstr(step)"></v-list-item-subtitle>
          </v-list-item-header>
          <template v-slot:append v-if="step.bookmark != ''">
            <v-list-item-avatar end>
              <v-btn variant="text" color="grey lighten-1" icon="mdi-bookmark"></v-btn>
            </v-list-item-avatar>
          </template>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <!-- Provides the application the proper gutter -->
        <v-container class="grey lighten-5" fluid>
          <v-row>
            <v-col md="4" v-if="isButtonPanelVisible">
              <RotationPanel
                @rotateAction="onRotateAction"
              />
            </v-col>
            <v-col md="8">
              <WasmScreen
                id="wasmelemid"
                :cubeViewType="cubeViewType"
                ref="wasm"
                @rotateAction="onRotateAction"
              />
            </v-col>
          </v-row>
        </v-container>
    </v-main>
    <v-dialog
      v-model="fileImportDialog"
    >
      <v-card>
        <v-card-text>操作履歴ファイル取込み</v-card-text>
        <v-file-input
          accept="text/*"
          v-model="importTagetfile"
          class="dialog-main"
        ></v-file-input>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn @click="onImportExecute">取込</v-btn>
          <v-btn color="primary" @click="fileImportDialog = false">Close</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-dialog
      v-model="showBookmarkDialog"
    >
      <v-card>
        <v-card-text>ブックマーク</v-card-text>
        <v-text-field
          v-model="editingBookmark"
          class="dialog-main"
        ></v-text-field>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn @click="onBookmarkChanged">決定</v-btn>
          <v-btn color="primary" @click="showBookmarkDialog = false">Close</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-app>
</template>

<script lang="ts">
import { defineComponent, ref, onMounted, onUnmounted } from 'vue'
import RotationPanel from './components/RotationPanel.vue'
import ControlPanel from './components/ControlPanel.vue'
import WasmScreen from './components/WasmScreen.vue'
import { RotateStep, RotateStepManager,  } from '@/class/rotateStepManager'
import { Axis, Layer, Dir, RotateStatus, cubeutils } from '@/class/cubeutils';
import { CubeViewType } from '@/class/types';

const { getRotateInfoFromStr, getRandomRotateInfo } = cubeutils();

type stepMenuType = "import" | "export" | "revert" | "replay" | "bookmark";

export default defineComponent({
  name: 'App',
  setup(){
    const stepManager: RotateStepManager = new RotateStepManager();
    const wait = async (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
    
    // ref
    const wasm = ref();
    const appBar = ref();
    const historyMenuButton = ref();
    const rotateStepListElem = ref();
    // const
    const appTitle = ref<string>("ルービック・キューブ訓練アプリ");
    const menuitems = ref([
      {id: "export", caption: "出力"},
      {id: "import", caption: "取込"},
      {id: "revert", caption: "ここまで戻す"},
      {id: "replay", caption: "ここ以降再生"},
      {id: "bookmark", caption: "ブックマーク"},
    ]);
    const activeRotateStepClass = ref<string>("active-rotatestep");
    //
    const fileImportDialog = ref<boolean>(false);
    const showConfigMenu = ref<boolean>(true);
    const showHistory = ref<boolean>(true);
    const showBookmarkDialog = ref<boolean>(false);
    const importTagetfile = ref<any>(undefined);
    const editingBookmark = ref<string>("");
    const waitMSec = ref<number>(100);
    const rotateStepList = ref<Array<RotateStep>>(stepManager.getCurrentStepList());
    const selectedStep = ref<number>(-1);
    const roteteStepListHeight = ref<number>(500);
    //
    const isButtonPanelVisible = ref<boolean>(false);
    const cubeViewType = ref<CubeViewType>("horizon");
    //
    const onControlAction = async (type: string, val: number) => {
      if (wasm.value != null) {
        wasm.value.setConfig(type, val);
      }
      switch (type) {
        case "reset":
          onClearStep();
          break;
        case "scramble": {
          scramble(val);
          break;
        }
        case "speed": {
          waitMSec.value = 100;
          break;
        }
        default:
          break;
      }
    };
    const onChangeButtonPanelVisible = (isButtonPanelVisiblePrm: boolean) => {
      isButtonPanelVisible.value = isButtonPanelVisiblePrm;
    };
    const onChangeCubeViewType = (cubeViewTypePrm: CubeViewType) => {
      cubeViewType.value = cubeViewTypePrm;
    };
    const onClearStep = () => {
      wasm.value.setConfig("reset", 0);
      stepManager.clearStepList();
      selectedStep.value = -1;
      rotateStepList.value = stepManager.getCurrentStepList(); // workaround
      forceUpdate(); // workaround
    };
    const onRotateAction = (axis: Axis, layer: Layer, dir: Dir) => {
      if (wasm.value != null) {
        stepManager.addStep(axis, layer, dir, "");
        // rotateStepList.value = stepManager.getCurrentStepList(); // workaround
        forceUpdate(); // workaround
      }
      if (onAnimation() == 0) {
        startRotate();
      }
    };
    const scramble = (step: number) => {
      onClearStep();
      for (var i = 0;i < step; i++) {
        const rotateInfo = getRandomRotateInfo();
        const { axis, layer, dir } = rotateInfo;
        const bookmark = (i == step - 1) ? "scramble" : "";
        stepManager.addStep(axis, layer, dir, bookmark);
      }
      startRotate();
      forceUpdate(); // workaround
    };
    const onRoteteStepClick = (event: any, idx: number) => {
      selectedStep.value = idx;
    };
    const onMenuClick = (menuid: stepMenuType) => {
      switch (menuid) {
        case "import":
          fileImportDialog.value = true;
          break;
        case "export":
          onExportExecute();
          break;
        case "revert":
          onRevertStep(selectedStep.value);
          break;
        case "replay":
          onReplay(selectedStep.value);
          break;
        case "bookmark":
          onEditBookmark();
          break;
      }
      forceUpdate(); // TODO workaround for close-on-click
    };
    const onImportExecute = () => {
      fileImportDialog.value = false;
      const files = importTagetfile.value;
      const reader = new FileReader();
      reader.readAsText(files[0]);
      reader.onload = async () => {
        onClearStep();
        if (reader.result == null) {
          return;
        }
        const stepStrList = reader.result.toString().split("\n");
        for (const stepStr of stepStrList) {
          const wkStep = stepStr.split(" ");
          const rotateInfo = getRotateInfoFromStr(wkStep[0]);
          if (rotateInfo == undefined) continue;
          const { axis, layer, dir } = rotateInfo;
          const bookmark = (wkStep.length > 1) ? wkStep[1] : "";
          stepManager.addStep(axis, layer, dir, bookmark);
        }
        startRotate();
        forceUpdate(); // workaround
      };
    };
    const onExportExecute = () => {
      const a = document.createElement('a');
      a.href = URL.createObjectURL(new Blob([stepManager.getStepListStr()], {type: 'text/plain'}));
      a.download = "history.txt";
      a.style.display = 'none';
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
    };
    const onRevertStep = async (stepIndex: number) => {
      let wkIdx = rotateStepList.value.length - 1;
      while (wkIdx > stepIndex) {
        if (onAnimation() == 1) {
          await wait(waitMSec.value);
          continue;
        } else {
          const wkStep = stepManager.revertStep();
          if (wkStep != undefined) {
            forceUpdate(); // workaround
            wasm.value.rotate(wkStep.axis, wkStep.layer, wkStep.dir);
          }
          wkIdx--;
        }
      }
    };
    const onReplay = async (stepIndex: number) => {
      let bulkRotateStr = "";
      for (var i = 0;i <= stepIndex; i++) {
        const { axis, layer, dir } = rotateStepList.value[i];
        const rotateStr = `${axis},${layer},${dir}`
        if (bulkRotateStr != "") bulkRotateStr += ";";
        bulkRotateStr += rotateStr;
      }
      wasm.value.setConfig("bulkrotate", bulkRotateStr);
      stepManager.revertRotateStatus(stepIndex);
      startRotate();
    };
    const onPlaybackOneStep = async () => {
      if (onAnimation() == 1) {
        return;
      }
      const counterStep = stepManager.revertOneRotateStep();
      if (counterStep == undefined) {
        return;
      }
      wasm.value.rotate(counterStep.axis, counterStep.layer, counterStep.dir);
      forceUpdate(); // workaround
    };
    const onPlayforwardOneStep = async () => {
      if (onAnimation() == 1) {
        return;
      }
      let wkStep = stepManager.getActiveStep();
      if (wkStep == undefined) {
        return;
      }
      await rotateOneStep(wkStep);
    };
    const onEditBookmark = () => {
      editingBookmark.value = rotateStepList.value[selectedStep.value].bookmark;
      showBookmarkDialog.value = true;
    };
    const onBookmarkChanged = () => {
      showBookmarkDialog.value = false;
      rotateStepList.value[selectedStep.value].bookmark = editingBookmark.value;
      forceUpdate(); // workaround
    };
    //
    const startRotate = async () => {
      let wkStep = stepManager.getActiveStep();

      while (wkStep != undefined) {
        await rotateOneStep(wkStep);
        wkStep = stepManager.getActiveStep();
      }
    };
    const rotateOneStep = async (step: RotateStep) => {
      let cntCheck = 0;
      while (cntCheck < 100) {
        if (step.rotateStatus == "bef") {
          wasm.value.rotate(step.axis, step.layer, step.dir);
          setRotateStatus("doing");
          forceUpdate(); // workaround
        } else if (step.rotateStatus == "doing") {
          if (onAnimation() == 1) {
            await wait(waitMSec.value);
            cntCheck++;
          } else {
            setRotateStatus("done");
          }
        } else {
          break;
        }
      }
      forceUpdate(); // workaround
    };
    const setRotateStatus = (status: RotateStatus) => {
      stepManager.setRotateStatus(status);
    };
    //
    const getStepStr = (step: RotateStep): string => {
      return stepManager.getStepStr(step);
    };
    const getStepSubstr = (step: RotateStep): string => {
      if (step.bookmark == undefined) {
        return "";
      }
      return step.bookmark;
    };
    const getStepClass = (step: RotateStep): string => {
      let retclass = "app_history-item";
      retclass += (" rotate-" + step.rotateStatus);
      return retclass;
    };
    const isActiveItem = (menuitem: any): boolean => {
      if (menuitem.id == "import" || menuitem.id == "export") {
        return true;
      }
      if (rotateStepListElem.value && rotateStepListElem.value.$el) {
        const selectedRotateItem = rotateStepListElem.value.$el.getElementsByClassName(activeRotateStepClass.value);
        const hasSelectedRotateItem = selectedRotateItem != undefined && selectedRotateItem.length > 0;
        return selectedStep.value >= 0 && hasSelectedRotateItem;
      } else {
        return false;
      }
    };
    const forceUpdate = () => {
      if (showHistory.value && rotateStepListElem.value.$forceUpdate) {
        rotateStepListElem.value.$forceUpdate(); // workaround
      }
    }
    //
    const onResize = () => {
      roteteStepListHeight.value = document.documentElement.clientHeight - 150;
    }
    // for tet
    const waitForAnimation = async () => {
      while (onAnimation() == 1) {
        await wait(waitMSec.value);
      }
    }
    const onAnimation = () => {
      return wasm.value.onWasmAnimation();
    }
    //
    onMounted(() => {
      window.addEventListener('resize', onResize);
      onResize();
    });
    onUnmounted(() => {
      window.removeEventListener('resize', onResize);
    });
    //
    return {
      // refisSideBySideView
      wasm,
      appBar,
      historyMenuButton,
      rotateStepListElem,
      roteteStepListHeight,
      isButtonPanelVisible,
      cubeViewType,
      // const
      appTitle,
      menuitems,
      activeRotateStepClass,
      //
      fileImportDialog,
      showConfigMenu,
      showHistory,
      rotateStepList,
      showBookmarkDialog,
      importTagetfile,
      editingBookmark,
      // event action
      onControlAction,
      onRotateAction,
      onMenuClick,
      onImportExecute,
      onExportExecute,
      onRoteteStepClick,
      onBookmarkChanged,
      onPlaybackOneStep,
      onPlayforwardOneStep,
      onChangeButtonPanelVisible,
      onChangeCubeViewType,
      // conmuted
      getStepClass,
      getStepStr,
      getStepSubstr,
      isActiveItem,
      //
      waitForAnimation
    };
  },
  props: {
    wasmReady: {type: Boolean, required: false}
  },
  components: {
    ControlPanel,
    RotationPanel,
    WasmScreen
  },
})
</script>
<style scoped>
.history-button {
  position: absolute;
  right: 2px;
  top: 2px;
}
.dialog-main {
  width: 500px;
  margin-left: 10px;
  margin-right: 30px;
}
.rotate-bef {
  color: lightgray;
}
.rotate-doing {
  color: red;
}

</style>