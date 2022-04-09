<template>
  <v-app>
    <v-app-bar
      app
      color="deep-purple accent-4"
      dense
      dark
      priority=0
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
              anchor="bottom"
              transition="scale-transition"
              close-on-click=true
            >
              <v-list>
                <v-list-item
                  v-for="menu in menuitems"
                  :key="menu.id"
                  @click="onMenuClick(menu.id)"
                >
                  <v-list-item-title>{{ menu.caption }}</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-menu>
          </v-btn>

        </v-list-subheader>
        <v-list-item
          v-for="(step, i) in rotateStepList"
          :key="i"
          :value="step"
          active-color="primary"
        >
          <v-list-item-title v-text="getStepStr(step)"></v-list-item-title>
        </v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <!-- Provides the application the proper gutter -->
        <v-container class="grey lighten-5" fluid>
          <v-row>
            <v-col md="4">
              <ControlPanel
                :defspeed=40
                :defscramblestep=24
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
  </v-app>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue'
import ControlPanel from './components/ControlPanel.vue'
import WasmScreen from './components/WasmScreen.vue'
import { RotateStep, RotateStepManager,  } from '@/class/rotateStepManager'
import { Axis, Layer, Dir, cubeutils } from '@/class/cubeutils';
const { getRotateInfoFromStr, getRandomRotateInfo } = cubeutils();

type fileModeType = "import" | "export" | "";

export default defineComponent({
  name: 'App',
  setup(){
    const waitMSec = 300; // TODO
    const stepManager: RotateStepManager = new RotateStepManager();
    const wait = async (ms: number) => new Promise(resolve => setTimeout(resolve, ms));
    //
    const wasm = ref();
    const fileDialogInput = ref();
    const historyMenuButton = ref();
    const menuitems = ref([
      {id: "export", caption: "出力"},
      {id: "import", caption: "取込"}
    ]);
    //
    const fileImportDialog = ref<boolean>(false);
    const showHistory = ref<boolean>(false);
    const rotateStepList = ref<Array<RotateStep>>([]);
    //
    const onControlAction = async (type: string, val: number) => {
      if (wasm.value != null) {
        wasm.value.setConfig(type, val);
      }
      switch (type) {
        case "reset":
          onClearStep();
          break;
        case "scramble":
          onClearStep();
          for (var i = 0;i < val; i++) {
            const { axis, layer, dir } = getRandomRotateInfo();
            onRotateAction(axis, layer, dir);
            await wait(waitMSec);
          }
          break;
      }
    };
    const onClearStep = () => {
      wasm.value.setConfig("reset", 0);
      rotateStepList.value = [];
      stepManager.clearStepList();
    };
    const onRotateAction = (axis: Axis, layer: Layer, dir: Dir) => {
      if (wasm.value != null) {
        wasm.value.rotate(axis, layer, dir);
        const step = stepManager.addStep(axis, layer, dir);
        rotateStepList.value.push(step);
      }
    };
    const onMenuClick = (menuid: fileModeType) => {
      switch (menuid) {
        case "import":
          fileImportDialog.value = true;
          break;
        case "export":
          onExportExecute();
          break;
      }
      historyMenuButton.value.$el.click(); // TODO workaround for close-on-click
    };
    const onImportExecute = () => {
      fileImportDialog.value = false;
      const inputElem = fileDialogInput.value.$el.getElementsByTagName('input')[0]; // TODO workaround for file-input value
      const files = inputElem.files;
      const reader = new FileReader();
      reader.readAsText(files[0]);
      reader.onload = async () => {
        onClearStep();
        const stepList = reader.result.toString().split("\n");
        for (const step of stepList) {
          const rotateInfo = getRotateInfoFromStr(step);
          if (rotateInfo == undefined) continue;
          const { axis, layer, dir } = rotateInfo;
          onRotateAction(axis, layer, dir);
          await wait(waitMSec);
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
    //
    const getStepStr = (step: RotateStep): string => {
      return stepManager.getStepStr(step);
    };
    //
    return {
      wasm,
      fileDialogInput,
      menuitems,
      historyMenuButton,
      //
      fileImportDialog,
      showHistory,
      rotateStepList,
      //
      onControlAction,
      onRotateAction,
      onMenuClick,
      onImportExecute,
      onExportExecute,
      //
      getStepStr,
    };
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
</style>