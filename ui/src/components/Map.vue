<script setup lang="ts">
import { ref, onMounted } from "vue";
import { GPSData } from "../interfaces";
import Map from "ol/Map.js";
import TileLayer from "ol/layer/Tile.js";
import View from "ol/View.js";
import OSM from "ol/source/OSM.js";

const coordianates_data = ref<GPSData[]>([]);
defineExpose({
  insertCoordinatesData,
});

function insertCoordinatesData(data) {
  coordianates_data.value.push(data);
}

const map_instance = ref<Map | null>(null);

onMounted(() => {
  map_instance.value = new Map({
    target: "map",
    layers: [
      new TileLayer({
        source: new OSM(),
      }),
    ],
    view: new View({
      center: [0, 0],
      zoom: 2,
    }),
  });
});
</script>
<template>
  <div>
    <div id="map" class="w-full h-[100vh]"></div>
  </div>
</template>
