<template>
  <div>
    <v-btn flat>
      Normal
    </v-btn>

    <v-btn
      flat
      color="secondary"
    >
      Secondary
    </v-btn>

    <v-btn
      flat
      color="error"
    >
      Error
    </v-btn>

    <v-btn
      flat
      disabled
    >
      Disabled
    </v-btn>
  </div>
</template>
<script lang="ts">
import { defineComponent, /*toRefs, onMounted*/ } from 'vue';

export default defineComponent({
})
</script>