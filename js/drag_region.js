// #TARGET MACOS
(() => {
	const TOP_HEIGHT = 32;
	const TRAFFIC_LIGHT_WIDTH = 80;

	const style = document.createElement("style");
	style.textContent = `
    html.discord-tauri-drag-hover, html.discord-tauri-drag-hover * { cursor: grab !important; }
    html.discord-tauri-dragging, html.discord-tauri-dragging * { cursor: grabbing !important; }
  `;
	(document.head || document.documentElement).appendChild(style);

	const root = document.documentElement;
	const inZone = (e) =>
		e.clientY < TOP_HEIGHT && e.clientX > TRAFFIC_LIGHT_WIDTH;

	document.addEventListener(
		"mousemove",
		(e) => {
			root.classList.toggle("discord-tauri-drag-hover", inZone(e));
		},
		true,
	);

	document.addEventListener(
		"mousedown",
		(e) => {
			if (e.button === 0 && inZone(e) && window?.ipc.postMessage) {
				root.classList.add("discord-tauri-dragging");
				window.ipc.postMessage("drag");
			}
		},
		true,
	);

	const clearDragging = () => root.classList.remove("discord-tauri-dragging");
	document.addEventListener("mouseup", clearDragging, true);
	window.addEventListener("blur", clearDragging);
})();
