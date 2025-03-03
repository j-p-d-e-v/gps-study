import "./style.css";
import Map from "ol/Map";
import OSM from "ol/source/OSM";
import TileLayer from "ol/layer/Tile";
import View from "ol/View";
import Point from "ol/geom/Point";
import { fromLonLat } from "ol/proj";
import Style from "ol/style/Style";
import Icon from "ol/style/Icon";
import Feature from "ol/Feature";
import VectorLayer from "ol/layer/Vector";
import VectorSource from "ol/source/Vector";
import MousePosition from "ol/control/MousePosition";
import { defaults as defaultControls } from "ol/control/defaults.js";
import { saveAs } from "file-saver";
const mousePosition = new MousePosition({
	projection: "EPSG:4326",
	target: document.getElementById("mouse_position"),
});
window.mouse_position = mousePosition;

const iconFeature = new Feature({
	geometry: new Point(fromLonLat([0, 0])),
	name: "andresbonifacio",
});
const iconStyle = new Style({
	image: new Icon({
		anchor: [0.5, 0.5],
		src: "icons/andresbonifacio.png",
		width: 60,
		height: 73,
	}),
});
const vectorSource = new VectorSource({
	features: [iconFeature],
});
// So in applying style in the feature, should it be the same index numbers?
iconFeature.setStyle([iconStyle]);

const vectorLayer = new VectorLayer({
	source: vectorSource,
});

const map = new Map({
	target: "map",
	layers: [
		new TileLayer({
			source: new OSM(),
		}),
		vectorLayer,
	],
	controls: defaultControls().extend([mousePosition]),
	view: new View({
		center: fromLonLat([121.0471130482098, 14.650102983159213]),
		zoom: 16,
	}),
});
let start_recording = false;
let object_coordinates = [];
map.on("pointermove", () => {
	if (start_recording) {
		console.log("pointermove");
		let value = document.getElementById("mouse_position").textContent.trim();
		if (value.search(",") && value.length > 0) {
			value = value.split(",");
			object_coordinates.push({
				lon: value[0],
				lat: value[1],
			});
			document.getElementById("total_coordinates").textContent =
				object_coordinates.length;
		}
	}
});

function resetCoordinates() {
	object_coordinates = [];
	document.getElementById("total_coordinates").textContent = 0;
}

function saveCoordinates() {
	if (object_coordinates.length == 0) {
		alert("No coordinates generated");
		return false;
	}
	let blob = new Blob([JSON.stringify(object_coordinates, null, 4)]);
	saveAs(blob, [document.getElementById("file_name").value, "json"].join("."));
}

function toggleRecording(evt) {
	if (evt.shiftKey) {
		console.log(evt.keyCode, evt.shiftKey);
		if (evt.keyCode == 81) {
			//Start Recording
			//Shift + Q
			resetCoordinates();
			start_recording = true;
			document.getElementById("recording_status").textContent = "RECORDING";
		} else if (evt.keyCode == 87) {
			//Stop Recording
			//Shift + W
			start_recording = false;
			document.getElementById("recording_status").textContent = "NOT RECORDING";
		} else if (evt.keyCode == 83) {
			// SaveAs
			// Shift + S
			saveCoordinates();
		}
	}
}

function setCoordinates(feature_name, lon, lat) {
	let feature = vectorSource
		.getFeatures()
		.find((f) => f.values_.name == feature_name);
	if (feature !== undefined) {
		return feature.setGeometry(
			new Point(fromLonLat([parseFloat(lon), parseFloat(lat)])),
		);
	}
}

function moveObject(name, data, index) {
	let item = data[index];
	let lon = item["lon"];
	let lat = item["lat"];
	setCoordinates(name, lon, lat);
	setTimeout(() => {
		index += 1;
		if (index < data.length) {
			moveObject(name, data, index);
		} else {
			console.log(name + " is done walking.");
		}
	}, 5);
}

document.onkeypress = toggleRecording;

window.saveCoordinates = saveCoordinates;
window.object_coordinates = object_coordinates;
window.fromlonlat = fromLonLat;
window.map = map;
window.vector_source = vectorSource;
window.setCoordinates = setCoordinates;
window.moveObject = moveObject;
/*
 * Notes:
 * -[x] Dragging of the icons (alternative is mouse pointer)
 * -[x] Check if the icons can produce coordinates while its being dragged in the map. I use via Mouse Position
 * - [x] Create a start and stop recording button when generating test data.
 * - [x] Custom FileName when Saving
 * - [ ] Animation of the icon movements.
 * - https://openlayers.org/en/latest/examples/icon-scale.html
 *- https://openlayers.org/en/latest/examples/
 * **
 * **
 */
