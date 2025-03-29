<script setup lang="ts">
import { ref, onMounted } from "vue";
import { GPSData } from "../interfaces";

const coordinates_data = ref<CoordinatesData>([]);
const ws = ref<WebSocket | null>(null);
const ws_open = ref<boolean>(false);
const is_working = ref<boolean>(false);

const emit = defineEmits(["coordinates-received"]);

function connect() {
  ws.value = new WebSocket("ws://127.0.0.1:4090/ws");
  ws.value.onopen = function () {
    ws_open.value = true;
  };
  ws.value.onmessage = function (result) {
    is_working.value = true;
    let data = JSON.parse(result.data);
    coordinates_data.value.push(data);
    emit("coordinates-received", data);
  };
  ws.value.onclose = function () {
    is_working.value = false;
    ws_open.value = false;
  };
}
function watch() {
  ws.value.send("");
}

function disconnect() {
  is_working.value = false;
  ws_open.value = false;
  ws.value.close();
}
onMounted(() => {});
</script>

<template>
  <div>
    <div>
      <template v-if="ws_open">
        <button
          @click="watch"
          :class="
            'btn btn-soft btn-sm p-3 bg-neutral-600 ' +
            (is_working ? 'btn-disabled' : 'btn-active')
          "
        >
          <span
            v-if="is_working"
            class="loading loading-spinner text-neutral-300"
          ></span>
          Simulate
        </button>
        <button
          @click="disconnect"
          class="ml-3 btn btn-soft btn-sm p-3 bg-neutral-600"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
            class="lucide lucide-unplug-icon lucide-unplug"
          >
            <path d="m19 5 3-3" />
            <path d="m2 22 3-3" />
            <path
              d="M6.3 20.3a2.4 2.4 0 0 0 3.4 0L12 18l-6-6-2.3 2.3a2.4 2.4 0 0 0 0 3.4Z"
            />
            <path d="M7.5 13.5 10 11" />
            <path d="M10.5 16.5 13 14" />
            <path
              d="m12 6 6 6 2.3-2.3a2.4 2.4 0 0 0 0-3.4l-2.6-2.6a2.4 2.4 0 0 0-3.4 0Z"
            />
          </svg>
          Disconnect
        </button>
      </template>
      <template v-else>
        <button @click="connect" class="btn btn-soft btn-sm p-3 bg-neutral-600">
          Connect
        </button>
      </template>
      <div class="overflow-x-auto h-[80vh] mt-3 border-1 border-neutral-500">
        <table
          class="table-zebra table-rows-cols table-pin-cols table-sm w-full border-none"
        >
          <thead class="text-sm">
            <tr class="sticky">
              <th>USER</th>
              <th>LATITUDE</th>
              <th>LONGITUDE</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="item in coordinates_data" v-bind:key="item">
              <td>{{ item.user_id }}</td>
              <td>{{ item.lat }}</td>
              <td>{{ item.lon }}</td>
            </tr>
          </tbody>
        </table>
      </div>
      <div>
        <small>Total Coordinates: {{ coordinates_data.length }}</small>
      </div>
    </div>
  </div>
</template>
