/**  Paints html/body dark before Discord's own CSS loads, killing the white flash
 * during navigation. Runs at document_start on every page.
 */
(() => {
	const css = "html,body{background-color:#000 !important;color-scheme:dark}";
	const inject = () => {
		const style = document.createElement("style");
		style.textContent = css;
		(document.head || document.documentElement).appendChild(style);
	};
	if (document.documentElement) inject();
	else document.addEventListener("readystatechange", inject, { once: true });
})();
