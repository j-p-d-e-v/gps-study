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
	geometry: new Point(fromLonLat([121.0471130482098, 14.650102983159213])),
});
const iconStyle = new Style({
	image: new Icon({
		anchor: [0, 0],
		src: "icons/github.png",
		width: 43,
		height: 43,
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

let object_coordinates = [];
map.on("pointermove", () => {
	let value = document.getElementById("mouse_position").textContent.trim();
	if (value.search(",") && value.length > 0) {
		value = value.split(",");
		object_coordinates.push({
			lon: value[0],
			lat: value[1],
		});
	}
});

function saveCoordinates() {
	let blob = new Blob([JSON.stringify(object_coordinates, null, 4)]);
	saveAs(blob, "test.json");
}
window.saveCoordinates = saveCoordinates;
window.object_coordinates = object_coordinates;
window.fromlonlat = fromLonLat;
window.map = map;

/*
 * Notes:
 * -[x] Dragging of the icons (alternative is mouse pointer)
 * -[x] Check if the icons can produce coordinates while its being dragged in the map. I use via Mouse Position
 * - [ ] Create a start and stop recording button when generating test data.
 * - [ ] Custom FileName when Saving
 * - Animation of the icon movements.
 * - https://openlayers.org/en/latest/examples/icon-scale.html
 *- https://openlayers.org/en/latest/examples/
 * **
 * **
 */
