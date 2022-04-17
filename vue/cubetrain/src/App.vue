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
      <v-app-bar-nav-icon></v-app-bar-nav-icon>

      <v-toolbar-title>Page title</v-toolbar-title>

      <v-spacer></v-spacer>

      <v-btn icon>
        <v-icon>mdi-heart</v-icon>
      </v-btn>

      <v-btn icon>
        <v-icon>mdi-magnify</v-icon>
      </v-btn>
    </v-app-bar>

    <v-navigation-drawer permanent>
      <!-- -->
    </v-navigation-drawer>

    <v-navigation-drawer permanent position="right" v-if="showHistory">
      <v-list density="compact">
        <v-list-subheader>操作履歴
          <v-btn
            flat
            class="history-button"
            ref="historyMenuButton"
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
                  @click="onMenuClick(menu.id)"
                  :disabled="!isActiveItem(menu)"
                >
                  <v-list-item-title>{{ menu.caption }}</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>
          </v-btn>
        </v-list-subheader>
      </v-list>
      <v-list density="compact" class="history-list">
        <v-list-item
          v-for="(step, i) in rotateStepList"
          :key="i"
          :value="step"
          active-color="primary"
          @click="onRoteteStepClick(i)"
        >
          <v-list-item-header>
            <v-list-item-title v-text="getStepStr(step)"></v-list-item-title>
            <v-list-item-subtitle v-text="getStepBubStr(step)"></v-list-item-subtitle>
          </v-list-item-header>
          <template v-slot:append v-if="step.bookMark != undefined">
            <v-list-item-avatar end>
              <v-btn variant="text" color="grey lighten-1" icon="mdi-bookmark"></v-btn>
            </v-list-item-avatar>
          </template>
          <!-- <v-menu activator="parent" close-on-content-click=true anchor="start">
            <v-list>
              <v-list-item
                v-for="(item) in rotateStepMenu"
                :key="item.id"
                :value="item.id"
                @click="onRotateStepMenu($event, item.id, i)"
              >
                <v-list-item-title>{{ item.caption }}</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-menu> -->
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <!-- Provides the application the proper gutter -->
        <v-container class="grey lighten-5" fluid>
          <v-row>
            <v-col md="4">
              <ControlPanel
                :defspeed=100
                :defscramblestep=10
                @controlAction="onControlAction"
                @rotateAction="onRotateAction"
              />
            </v-col>
            <v-col md="8">
              <WasmScreen
                id="wasmelemid"
                ref="wasm"
              />
            </v-col>
          </v-row>
          <v-btn
            class="history-button"
            @click="(showHistory = !showHistory)"
          >
            <v-icon v-if="showHistory">mdi-arrow-right</v-icon>
            <v-icon v-else>mdi-animation</v-icon>
          </v-btn>
        </v-container>
    </v-main>
    <v-dialog
      v-model="fileImportDialog"
    >
      <v-card>
        <!-- eslint-disable-next-line -->
        <v-card-text>[　　　　　操作履歴ファイル取込み　　　　　]</v-card-text> <!-- TODO workaround for width -->
        <v-file-input
          accept="text/*"
          ref="fileDialogInput"
        ></v-file-input>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn @click="onImportExecute">取込</v-btn>
          <v-btn color="primary" @click="fileImportDialog = false">Close</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <v-label ref="dmyLabel"/>
  </v-app>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import ControlPanel from './components/ControlPanel.vue'
import WasmScreen from './components/WasmScreen.vue'
import { RotateStep, RotateStepManager,  } from '@/class/rotateStepManager'
import { Axis, Layer, Dir, RotateInfo, RotateStatus, cubeutils } from '@/class/cubeutils';
const { getRotateInfoFromStr, getRandomRotateInfo } = cubeutils();

import { on_animation } from '@/wasm/package.js';

type fileModeType = "import" | "export" | "revert" | "replay" | "";
// type rotateMenuType = "revert" | "replay" | "";

export default defineComponent({
  name: 'App',
  setup(){
    const stepManager: RotateStepManager = new RotateStepManager();
    const wait = async (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
    //
    const wasm = ref();
    const appBar = ref();
    const fileDialogInput = ref();
    const historyMenuButton = ref();
    const menuitems = ref([
      {id: "export", caption: "出力"},
      {id: "import", caption: "取込"},
      {id: "revert", caption: "ここまで戻す"},
      // {id: "replay", caption: "ここ以降再生"}
    ]);
    // const rotateStepMenu = ref([
    //   {id: "revert", caption: "ここまで戻す"},
    //   {id: "replay", caption: "ここ以降再生"}
    // ]);
    //
    const fileImportDialog = ref<boolean>(false);
    const showHistory = ref<boolean>(true);
    const rotateStepList = ref<Array<RotateStep>>([]);
    const waitMSec = ref<number>(100);
    const selectedStep = ref<number>(-1);
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
    const onClearStep = () => {
      wasm.value.setConfig("reset", 0);
      rotateStepList.value = [];
      stepManager.clearStepList();
      selectedStep.value = -1;
    };
    const onRotateAction = (axis: Axis, layer: Layer, dir: Dir) => {
      if (wasm.value != null) {
        const step = stepManager.addStep(axis, layer, dir);
        rotateStepList.value.push(step);
      }
      startRotate();
    };
    const scramble = async (step: number) => {
      onClearStep();
      const rotateInfoList: Array<RotateInfo> = [];
      for (var i = 0;i < step; i++) {
        rotateInfoList.push(getRandomRotateInfo());
      }
      await bulkRotate(rotateInfoList);
      stepManager.addBookmark(step - 1, "scramble");
    };
    const onMenuClick = (menuid: fileModeType) => {
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
        // case "replay":
        //   onRevertStep(selectedStep.value);
        //   break;
      }
      historyMenuButton.value.$el.click(); // TODO workaround for close-on-click
    };
    const onRoteteStepClick = (idx: number) => {
      selectedStep.value = idx;
    };
    // const onRotateStepMenu = (event: Event, menuid: rotateMenuType, index: number) => {
    //   console.log(menuid + ":" + index);
    //   switch (menuid) {
    //     case "revert":
    //       break;
    //     case "replay":
    //       stepManager.revertRotated(index);
    //       startRotate();
    //       break;
    //   }
    //   appBar.value.$el.click(); // TODO workaround for close-on-click
    // };
    const onImportExecute = async () => {
      fileImportDialog.value = false;
      const inputElem = fileDialogInput.value.$el.getElementsByTagName('input')[0]; // TODO workaround for file-input value
      const files = inputElem.files;
      const reader = new FileReader();
      reader.readAsText(files[0]);
      reader.onload = async () => {
        onClearStep();
        if (reader.result == null) {
          return;
        }
        const stepStrList = reader.result.toString().split("\n");
        const rotateInfoList: Array<RotateInfo> = [];
        const bookmarkMap: any = new Map();
        for (const stepStr of stepStrList) {
          const wkStrList = stepStr.split(" ");
          if (wkStrList.length == 0) continue;
          const rotateInfo = getRotateInfoFromStr(wkStrList[0]);
          if (rotateInfo == undefined) continue;
          rotateInfoList.push(rotateInfo);
          if (wkStrList.length > 1) {
            bookmarkMap.set(rotateInfoList.length - 1, wkStrList[1]);
          }
        }
        await bulkRotate(rotateInfoList);
        for (const key of bookmarkMap.keys()) {
          stepManager.addBookmark(key, bookmarkMap.get(key));
        }
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
    const onRevertStep = async (idx: number) => {
      let wkIdx = rotateStepList.value.length - 1;
      while (wkIdx > idx) {
        if (on_animation() == 1) {
          await wait(waitMSec.value);
          continue;
        } else {
          const wkStep = stepManager.revertStep(wkIdx);
          if (wkStep == undefined) {
            continue;
          }
          wasm.value.rotate(wkStep.axis, wkStep.layer, wkStep.dir);
          rotateStepList.value.splice(-1, 1);
          wkIdx--;
        }
      }
    };
    //
    const bulkRotate = async (rotateInfoList: Array<RotateInfo>) => {
      for (const rotateInfo of rotateInfoList) {
        const { axis, layer, dir } = rotateInfo;
        const step = stepManager.addStep(axis, layer, dir);
        rotateStepList.value.push(step);
      }
      startRotate();
    };
    const startRotate = async () => {
      let wkStep = stepManager.getActiveStep();

      while (wkStep != undefined) {
        if (wkStep.rotateStatus == "bef") {
          wasm.value.rotate(wkStep.axis, wkStep.layer, wkStep.dir);
          setRotateStatus("doing");
        } else if (wkStep.rotateStatus == "doing") {
          if (on_animation() == 1) {
            await wait(waitMSec.value);
          } else {
            setRotateStatus("done");
          }
        } else {
          wkStep = stepManager.getActiveStep();
        }
      }
    }
    const setRotateStatus = (status: RotateStatus) => {
      stepManager.setRotateStatus(status);
      rotateStepList.value[stepManager.getActiveIdx()].rotateStatus = status;
    }
    //
    const getStepStr = (step: RotateStep): string => {
      return stepManager.getStepStr(step);
    };
    const getStepBubStr = (step: RotateStep): string => {
      if (step.bookMark == undefined) {
        return "";
      }
      return step.bookMark.name;
    };
    const isActiveItem = (menuitem: any): boolean => {
      if (menuitem.id == "import" || menuitem.id == "export") {
        return true;
      }
      return selectedStep.value >= 0;
    };
    //
    return {
      wasm,
      appBar,
      fileDialogInput,
      menuitems,
      // rotateStepMenu,
      historyMenuButton,
      //
      fileImportDialog,
      showHistory,
      rotateStepList,
      //
      onControlAction,
      onRotateAction,
      onMenuClick,
      onRoteteStepClick,
      // onRotateStepMenu,
      onImportExecute,
      onExportExecute,
      //
      getStepStr,
      getStepBubStr,
      isActiveItem
    };
  },
  props: {
    wasmReady: {type: Boolean, required: false}
  },
  components: {
    ControlPanel,
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
.history-list {
  height: 500px;
}
</style>