<script setup lang="ts">
import { ref } from "vue";
import GPSData from "./components/GPSData.vue";
import Map from "./components/Map.vue";
import gps_pin_icon from "./assets/pin-icon-animated.gif";

const map = ref(null);

function coordinatesReceived(data) {
  //console.log("Coodinates Received", data);
  console.log(map.value.insertCoordinatesData(data));
}
</script>

<template>
  <div
    class="grid bg-neutral-800 grid-cols-[40%_60%] grid-rows-1 w-full h-[100vh]"
  >
    <div class="bg-neutral-700 p-3">
      <div class="text-[20px] font-semibold">
        <img :src="gps_pin_icon" width="48" class="float-left" />
        <span class="float-left mt-2">GPS SIMULATION APP</span>
        <div class="clear-both"></div>
      </div>
      <GPSData @coordinates-received="coordinatesReceived" />
    </div>
    <div>
      <Map ref="map" />
    </div>
  </div>
</template>

<style scoped></style>
